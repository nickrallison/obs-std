---
bad_links: 
aliases: [Borrowing, borrow]
tags: [rust]
---
# Rust Ownership

Rust is a statically typed, compiled language that emphasizes safety and performance. One of the key features of Rust is its ownership system, which helps manage memory safety without a garbage collector. 

In Rust, all values have a variable that's called its *owner*. There can only be one owner at a time. When the owner goes out of scope, the value will be dropped.

Here's a simple example:

```rust
{
    let s = "hello";   // s is in scope.
    // do something with s
}                      // s is out of scope and dropped.
```

This ownership rule applies to all data types in Rust, including integers, booleans, structs, enums, and even complex types like vectors and strings.

Ownership can be transferred in Rust, which is known as *moving*. Here's an example:

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 has been moved into s2
```

After the move, you can no longer use `s1`. This prevents double free error because when `s2` and `s1` go out of scope, only `s2` will attempt to free the memory.

Rust also has a feature called *borrowing*. You can have unlimited borrows for read-only access (immutable borrows), or one borrow for write access (mutable borrow). But you can't have both at the same time. This is enforced at compile time and is known as Rust's borrow checker. Here's an example:

```rust
let mut s = String::from("hello");
{
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
} // r1 and r2 are no longer used after this point

let r3 = &mut s; // no problem
println!("{}", r3);
```

This borrowing rule prevents data races at compile time.

Ownership, moving, and borrowing are the key concepts of Rust's memory management. They ensure memory safety without needing a garbage collector, and they prevent data races. This makes Rust unique and suitable for systems programming.

> For more in-depth information, you can refer to the [Rust Book's chapter on Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html).