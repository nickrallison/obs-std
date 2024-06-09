---
aliases: []
tags: [rust]
bad_links:
---
# Rust RefCell

`RefCell` is a part of Rust's system for managing memory safety at runtime, as opposed to compile time. It's a part of the standard library's `std::cell` module, which provides "shareable [[Mutability|mutable]] containers". 

In Rust, the [[Rust Ownership|borrow]] checker enforces the rules of references at compile time: you can have either many [[Mutability|immutable]] references or one [[Mutability|mutable]] reference. This is a core aspect of Rust's memory safety guarantees. However, there are situations where you might need to [[Mutability|mutate]] something that you have a shared reference to, and for those situations, Rust provides the `RefCell` type.

`RefCell` enforces the [[Rust Ownership|borrowing]] rules at runtime instead of compile time. This means that you can have many references to a `RefCell`, and it will dynamically check whether you have an exclusive [[Mutability|mutable]] reference or shared references. If you try to violate the [[Rust Ownership|borrowing]] rules, it will panic and crash your program.

Here's a basic usage of `RefCell`:

```rust
use std::cell::RefCell;

let x = RefCell::new(42);
{
    let y = x.borrow_mut();
    // y is a mutable reference to the value inside the RefCell
}
// the mutable borrow ends here
{
    let z = x.borrow();
    // z is an immutable reference to the value inside the RefCell
}
```

In this example, `x` is a `RefCell` that contains the value `42`. The `borrow_mut` method returns a `RefMut`, a type which represents a [[Mutability|mutable]] reference to the inner value. Similarly, the `borrow` method returns a `Ref`, which represents an [[Mutability|immutable]] reference. The `RefCell` keeps track of how many `Ref` and `RefMut` objects exist: the [[Rust Ownership|borrow]] counts. If you try to create a `RefMut` while a `Ref` exists, the `RefCell` will panic.

Tangentially related to `RefCell` are other types in the `std::cell` module, such as `Cell` and `UnsafeCell`. `Cell` is similar to `RefCell`, but it works for `Copy` types and allows you to replace the value it contains. `UnsafeCell` is the fundamental building block for these types, and it's the only way to obtain aliasable, [[Mutability|mutable]] memory in safe Rust.

> For more information, you can refer to the [Rust documentation on `RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html) and the [Rust book's chapter on `RefCell`](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#enforcing-borrowing-rules-at-runtime-with-refcellt).