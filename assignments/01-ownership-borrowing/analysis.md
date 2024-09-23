### `find_in_string` explanation

In the `find_in_string` function, the compiler needs more information due to Rust's strict lifetime rules. Since the function returns a slice of the `haystack` string, Rust requires a lifetime annotation to ensure that the returned reference does not outlive the `haystack`. The error occurs because Rust needs to guarantee that the reference is valid for as long as expected. By explicitly specifying the lifetime `'a`, I tie the returned slice's lifetime to that of the `haystack`, satisfying Rust's safety requirements.

### Doubly Linked Stack

It is difficult to create a doubly linked list in safe Rust because of Rust's ownership and borrowing rules. Each node needs to point to both its previous and next neighbors, but Rust’s ownership model enforces that each piece of data can only have a single owner. This makes it tricky to establish these two-way connections without running into issues with borrowing rules.

For instance, managing the ownership of nodes when inserting or removing elements in the middle of the list is particularly tough. Rust’s strict borrowing principles prevent you from having multiple mutable references to a node, making it difficult to update pointers without violating Rust’s safety guarantees.