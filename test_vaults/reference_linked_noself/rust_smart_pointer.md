---
aliases: []
tags: [rust]
bad_links:
---
# Rust Smart [[Pointer|Pointer]]

Rust's Smart Pointers are data structures that not only act like pointers but also have additional metadata and capabilities. Unlike traditional pointers, smart pointers are structures that implement the `Deref` and `Drop` [[Rust Traits|traits.]] The `Deref` [[Rust Traits|trait]] allows an instance of the smart [[Pointer|pointer]] struct to behave like a reference so you can write code that works with either references or smart pointers. The `Drop` [[Rust Traits|trait]] allows you to customize what happens when a value goes out of scope.

There are different types of smart pointers in Rust, each with its own characteristics:

1. **Box\<T>**: This is the simplest form of smart [[Pointer|pointer,]] used for allocating values on the heap. A box allows you to store data on the heap rather than the stack. What remains on the stack is the [[Pointer|pointer]] to the heap data. Boxes don't have performance overhead, other than storing their data on the heap. They also don't have many extra capabilities. One of the primary use cases for boxes is to ensure that a large amount of data doesn't stack overflow; they also have another use case that we'll cover when we discuss [[Rust Traits|trait]] objects.

2. **Rc\<T>**: This smart [[Pointer|pointer]] allows for multiple owners of the same data; "Rc" stands for "Reference Counting." The data type `Rc<T>` keeps track of the number of references to a value which determines whether or not a value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

3. **RefCell\<T>**: Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the data it holds. So what makes `RefCell<T>` different from a unique reference? `RefCell<T>` allows for "interior mutability", a design pattern in Rust that allows you to [[Mutability|mutate]] data even when there are [[Mutability|immutable]] references to that data.

In addition to these, there are other smart pointers like `Arc<T>`, `Mutex<T>`, `RwLock<T>`, etc. which are used in concurrent programming.

Here is a simple example of using `Box<T>`:

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

In this code, `b` is a [[Pointer|pointer]] to an integer value on the heap. When `b` goes out of scope at the end of `main`, the integer on the heap will be deallocated because `Box<T>` implements the `Drop` [[Rust Traits|trait.]]

> For more information, you can refer to the [Rust documentation on Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html).