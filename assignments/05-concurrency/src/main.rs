use std::sync::{Arc, Mutex};
use crate::thread_pool::ThreadPool;
use anyhow::Result;
use rand::Rng;
use std::time::Instant;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

mod error;
mod thread_pool;

/// Generate a random vector of size `capacity` filled with random `i64`s.
fn random_vec(capacity: usize) -> Vec<i64> {
    let mut vec = vec![0; capacity];
    rand::thread_rng().fill(&mut vec[..]);
    vec
}

/// Perform a k-way merge on multiple sorted arrays using a min-heap (priority queue).
///
/// `sorted_chunks` is a vector of sorted arrays to merge.
fn k_way_merge(sorted_chunks: Vec<Vec<i64>>) -> Vec<i64> {
    let mut result = Vec::new();
    let mut min_heap = BinaryHeap::new();

    for (i, chunk) in sorted_chunks.iter().enumerate() {
        if let Some(&val) = chunk.get(0) {
            min_heap.push(Reverse((val, i, 0))); // (value, chunk_index, element_index)
        }
    }

    while let Some(Reverse((val, chunk_idx, elem_idx))) = min_heap.pop() {
        result.push(val);

        if let Some(&next_val) = sorted_chunks[chunk_idx].get(elem_idx + 1) {
            min_heap.push(Reverse((next_val, chunk_idx, elem_idx + 1)));
        }
    }

    result
}

/// Sorts the array concurrently by dividing it into chunks, sorting each chunk, and then performing a k-way merge.
fn merge_sort_concurrent_with_k_way_merge(data: &mut [i64], pool: &ThreadPool, num_chunks: usize) -> Result<()> {
    let chunk_size = (data.len() + num_chunks - 1) / num_chunks;
    let sorted_chunks_arc = Arc::new(Mutex::new(Vec::with_capacity(num_chunks)));

    let chunks = data.chunks_mut(chunk_size);
    for chunk in chunks {
        let sorted_chunks_arc = Arc::clone(&sorted_chunks_arc);
        let chunk_vec = chunk.to_vec();

        pool.execute(move || {
            let mut sorted_chunk = chunk_vec;
            sorted_chunk.sort_unstable();
            sorted_chunks_arc.lock().unwrap().push(sorted_chunk);
        })?;
    }

    std::thread::sleep(std::time::Duration::from_millis(100));

    let sorted_chunks = sorted_chunks_arc.lock().unwrap();
    let merged_data = k_way_merge(sorted_chunks.clone());

    data.copy_from_slice(&merged_data);

    Ok(())
}

fn main() -> Result<()> {
    // Create a large vector of random numbers
    let data = random_vec(10_000_000);

    // Benchmark the sort with different thread counts
    for num_threads in [1, 2, 4, 8, 16, 32, 64, 100] {
        let mut data_clone = data.clone();
        let pool = ThreadPool::new(num_threads)?;
        let start = Instant::now();

        merge_sort_concurrent_with_k_way_merge(&mut data_clone, &pool, num_threads)?;

        let duration = start.elapsed();
        println!("Threads: {}, Time taken: {:?}", num_threads, duration);
    }

    Ok(())
}
