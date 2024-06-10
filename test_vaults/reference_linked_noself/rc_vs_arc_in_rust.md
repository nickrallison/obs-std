---
aliases:
tags:
  - rust
  - coding
bad_links:
---
# RC Vs. [[ARC Smart Pointer.md|ARC]] in Rust

In Rust, memory safety and management are among the core principles that make it unique and powerful. Rust employs ownership, [[rust_ownership.md|borrowing]], and lifetimes to ensure memory safety without needing a garbage collector. However, there are scenarios where multiple ownership of a single resource is necessary. This is where Rust's smart pointers come into play, with `Rc` and `Arc` being two of the primary types designed for this purpose. Understanding the differences between `Rc` and `Arc` is crucial for Rust developers, especially when dealing with concurrent programming.

## `Rc<T>`: [[rc_smart_pointer.md|Reference Counting]] for Single-threaded Scenarios

The `Rc<T>` type, short for [[rc_smart_pointer.md|Reference Counting]], is a non-thread-safe smart [[Pointer.md|pointer]] that enables multiple owners of a single piece of data. The `T` here stands for the data type `Rc` is generic over. The [[rc_smart_pointer.md|reference count]] ensures that data stays alive as long as there is at least one owner and is cleaned up once the last reference is dropped. This is especially useful for complex data structures like graphs, where multiple elements can own the same node.

### How `Rc<T>` Works

- **Creation**: An `Rc<T>` is created using `Rc::new(data)`, wrapping the data with a [[rc_smart_pointer.md|reference count]].
- **Cloning for Sharing**: To share ownership, `Rc<T>` is cloned, which doesn't duplicate the data but instead increases the [[rc_smart_pointer.md|reference count]].
- **Dropping References**: Once a variable that owns an `Rc<T>` goes out of scope or is explicitly dropped, the [[rc_smart_pointer.md|reference count]] is decreased.
- **Cleanup**:When the [[rc_smart_pointer.md|reference count]] reaches zero, the data is cleaned up.

### Limitations of `Rc<T>`

Being non-thread-safe, `Rc<T>` cannot be used in multi-threaded contexts. Attempting to share an `Rc<T>` across threads will result in a compile-time error, safeguarding against potential data races.

## `Arc<T>`: [[atomic_types.md|Atomic]] [[rc_smart_pointer.md|Reference Counting]] for Multi-threaded Scenarios

[[arc_smart_pointer.md|ARC]] or [[atomic_types.md|Atomic]] [[rc_smart_pointer.md|Reference Counting]], is the thread-safe counterpart of `Rc<T>`. It enables multiple thread-safe references to the same piece of data. The atomicity comes from the fact that incrementing or decrementing the [[rc_smart_pointer.md|reference count]] uses [[Atomic Types.md|atomic]] operations, ensuring thread safe smart pointers.

### How`Arc<T>` Works

- **Creation and Sharing**: Similar to `Rc<T>`, an `Arc<T>` is created with `Arc::new(data)` and shared among owners through cloning.
- **Thread Safety**: The [[Atomic Types.md|atomic]] operations on the [[rc_smart_pointer.md|reference count]] make it safe to share `Arc<T>` across threads.
- **Use Cases**: `Arc<T>` is ideal for concurrent programs where data needs to be safely shared between threads without explicit locking mechanisms.

### Performance Considerations

The thread safety of `Arc<T>` comes with a performance cost due to the use of [[Atomic Types.md|atomic]] operations for managing the [[rc_smart_pointer.md|reference count]]. These operations are more expensive than their non-atomic counterparts used in `Rc<T>`, potentially affecting performance in highly concurrent scenarios.

### Choosing Between `Rc<T>` and `Arc<T>`

The choice between `Rc<T>` and `Arc<T>` primarily depends on the context:
- Use `Rc<T>` in single-threaded situations where you need to share data without taking ownership.
- Opt for `Arc<T>` in multi-threaded applications where data needs to be accessed from multiple threads.

Understanding and choosing between `Rc<T>` and `Arc<T>` is crucial for Rust developers, especially when working on applications where concurrency plays a significant role. It's also worth noting that Rust's emphasis on memory safety makes both `Rc<T>` and `Arc<T>` safe alternatives to manual [[rc_smart_pointer.md|reference counting]], greatly reducing the risk of [[memory_leak.md|memory leaks]] and dangling references.