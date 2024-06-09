---
bad_links: 
aliases: [ARC, atomic reference count]
tags: [coding, rust]
---
# ARC Smart [[Pointer|Pointer]]

The Rust ARC Smart [[Pointer|Pointer]] is a concurrency primitive in the Rust programming language. It stands for "Atomic Reference Counting" and it's used to share read-only data between multiple threads in a safe manner. It is similar to the Rc ([[RC Smart Pointer|Reference Counting]]) [[Pointer|pointer]], but safe to use across multiple threads. When the last reference to a value inside an Arc [[Pointer|pointer]] goes out of scope, the value is automatically cleaned up. This [[Pointer|pointer]] type is especially useful when you want to share data but don't know at compile time how many references there'll be or when they'll be dropped.

```rust
// Import the needed libraries
use std::sync::Arc;
use std::thread;

// This is an example of ARC Smart [[Pointer.md|Pointer]] in Rust
fn main() {
    // Create a number inside an Arc [[Pointer.md|pointer]]
    let number = Arc::new(5);

    // Create a vector to hold our child threads
    let mut children = vec![];

    for _ in 0..10 {
        // Clone the Arc [[Pointer.md|pointer]] (this increments the internal reference count)
        let number = Arc::clone(&number);

        // Spawn a new thread
        let child = thread::spawn(move || {
            // Here we can read the value inside the Arc [[Pointer.md|pointer]] from our thread
            println!("Number: {}", *number);
        });

        // Add the child thread to our vector of children
        children.push(child);
    }

    // Wait for all children to finish executing
    for child in children {
        let _ = child.join();
    }
}
```
In this code, we create an `Arc` that wraps around the integer `5`. We then clone this `Arc` and pass it into multiple threads. Each thread can read from this `Arc` safely and concurrently. When each thread finishes execution, it drops its reference to the `Arc`, and when all references have been dropped, the `Arc` is cleaned up.