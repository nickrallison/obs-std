---
bad_links: 
aliases: []
tags: [rust]
---
# Rust String vs. &str

In Rust, `String` and `&str` are two different types used to handle string data, but they serve different purposes and have different characteristics.

1. **String**: `String` is a growable, mutable, owned, heap-allocated data structure. It also implements the `Clone` trait, so it can be duplicated with the `.clone()` method. This means that when you want to modify or extend a string, you would use `String`. Here's an example of a `String`:

```rust
let mut s = String::from("Hello");
s.push_str(", world!"); // s now contains "Hello, world!"
```

1. **&str**: `&str` is an immutable reference to a string slice. It's a view into an already allocated string, such as a `String` or a string literal. You can think of `&str` as a pointer to a pre-existing string along with its length. Here's an example of a `&str`:

```rust
let s: &str = "Hello, world!";
```

The main differences between `String` and `&str` are ownership and where they are stored. `String` is owned and can be changed, and it is stored on the heap. `&str`, on the other hand, is a borrowed reference to a string that is stored elsewhere, either on the heap (in the case of referencing a `String`) or in the program's binary (in the case of referencing a string literal), and it is immutable.

Tangentially related to `String` and `&str` is the concept of string slicing. You can take a slice of a `String` or a `&str` using range syntax, and the result will be a `&str`. This allows you to create a new view into a portion of the string without having to clone or copy the string data. Here's an example:

```rust
let s = String::from("Hello, world!");
let hello = &s[0..5]; // hello is a &str that contains "Hello"
```

In terms of formulas, derivations, or proofs, string handling in Rust is more about understanding the language's memory management model and ownership system than about mathematical concepts. However, understanding the size of these types can be useful. A `String` is 24 bytes (on 64-bit systems): 8 bytes for the pointer to the heap, 8 bytes for the length, and 8 bytes for the capacity. A `&str` is 16 bytes (on 64-bit systems): 8 bytes for the pointer to the string and 8 bytes for the length.

> For more information, you can refer to the [Rust Book's chapter on Strings](https://doc.rust-lang.org/book/ch08-02-strings.html) and the [Rust by Example's section on Strings](https://doc.rust-lang.org/rust-by-example/std/str.html).