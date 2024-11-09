use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use anyhow::Result;
use crate::error::ThreadPoolError;
use std::panic::{self, UnwindSafe};

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Option<Job>>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            for _ in &self.workers {
                let _ = sender.send(None); // Sending shutdown signal
            }
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                if let Err(e) = thread.join() {
                    eprintln!("Worker {} failed to shut down cleanly: {:?}", worker.id, e);
                }
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + UnwindSafe + 'static>;

impl ThreadPool {
    pub fn new(num_threads: usize) -> Result<Self, ThreadPoolError> {
        if num_threads == 0 {
            return Err(ThreadPoolError::ZeroThreads);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(num_threads);
        for id in 0..num_threads {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }

    pub fn execute<F>(&self, f: F) -> Result<(), ThreadPoolError>
    where
        F: FnOnce() + Send + UnwindSafe + 'static,
    {
        let job = Box::new(f);

        self.sender
            .as_ref()
            .ok_or(ThreadPoolError::Send)?
            .send(Some(job))
            .map_err(|_| ThreadPoolError::Send)
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Option<Job>>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            match receiver.lock().unwrap().recv() {
                Ok(Some(job)) => {
                    println!("Worker {id} executing a job.");
                    // Use catch_unwind to handle potential panics in jobs
                    let _ = panic::catch_unwind(job);
                }
                Ok(None) => {
                    println!("Worker {id} received shutdown signal.");
                    break;
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            };
        });

        Worker {
            id,
            thread: Some(handle),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use crate::error::ThreadPoolError;

    #[test]
    fn test_thread_pool_zero_threads() {
        let result = ThreadPool::new(0);
        assert!(matches!(result, Err(ThreadPoolError::ZeroThreads)));
    }

    #[test]
    fn test_thread_pool_single_task_execution() {
        let pool = ThreadPool::new(2).expect("Failed to create thread pool");

        let counter = Arc::new(Mutex::new(0));
        let counter_clone = Arc::clone(&counter);

        pool.execute(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        })
        .expect("Failed to execute task");

        thread::sleep(Duration::from_millis(50));

        assert_eq!(*counter.lock().unwrap(), 1);
    }

    #[test]
    fn test_thread_pool_multiple_task_execution() {
        let pool = ThreadPool::new(4).expect("Failed to create thread pool");

        let counter = Arc::new(Mutex::new(0));
        let num_tasks = 10;

        for _ in 0..num_tasks {
            let counter_clone = Arc::clone(&counter);
            pool.execute(move || {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            })
            .expect("Failed to execute task");
        }

        thread::sleep(Duration::from_millis(200));

        assert_eq!(*counter.lock().unwrap(), num_tasks);
    }

    #[test]
    fn test_thread_pool_graceful_shutdown_with_pending_tasks() {
        let pool = ThreadPool::new(2).expect("Failed to create thread pool");

        let counter = Arc::new(Mutex::new(0));
        let num_tasks = 5;

        for _ in 0..num_tasks {
            let counter_clone = Arc::clone(&counter);
            pool.execute(move || {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }).expect("Failed to execute task");
        }

        drop(pool);

        assert_eq!(*counter.lock().unwrap(), num_tasks);
    }

    #[test]
    fn test_thread_pool_task_panic_handling() {
        let pool = ThreadPool::new(2).expect("Failed to create thread pool");

        let counter = Arc::new(Mutex::new(0));

        // This task should panic but not affect other tasks
        pool.execute(move || {
            panic!("Intentional panic in task");
        }).expect("Failed to execute task");

        // Task to ensure thread pool is still functional
        let counter_clone = Arc::clone(&counter);
        pool.execute(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        }).expect("Failed to execute task");

        thread::sleep(Duration::from_millis(50));

        assert_eq!(*counter.lock().unwrap(), 1);
    }

    #[test]
    fn test_thread_pool_high_volume_tasks() {
        let pool = ThreadPool::new(4).expect("Failed to create thread pool");

        let counter = Arc::new(Mutex::new(0));
        let num_tasks = 1000;

        for _ in 0..num_tasks {
            let counter_clone = Arc::clone(&counter);
            pool.execute(move || {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }).expect("Failed to execute task");
        }

        thread::sleep(Duration::from_millis(500));

        assert_eq!(*counter.lock().unwrap(), num_tasks);
    }

    #[test]
    fn test_thread_pool_execute_after_shutdown() {
        let mut pool = ThreadPool::new(2).expect("Failed to create thread pool");

        // Explicitly set sender to None to simulate shutdown
        drop(pool.sender.take());

        // Attempt to execute a task, expecting a Send error
        let result = pool.execute(|| println!("Task after shutdown"));
        assert_eq!(result, Err(ThreadPoolError::Send));
    }
}
