---
bad_links: 
aliases: []
tags: [coding, rust]
---
# Trait Bounds

Trait bounds in Rust are a way to specify that a generic type parameter must implement a particular trait. This is useful when you want to use methods from a trait on a generic type. 

Here's a basic example:

```rust
fn display<T: Display>(t: T) {
    println!("{}", t);
}
```

In this function, `T: Display` is a trait bound. It means "T must implement the Display trait". The `Display` trait is part of Rust's standard library and provides a method for formatting the type into a string.

Trait bounds can also be specified with the `where` keyword, which can make complex trait bounds more readable:

```rust
fn display<T>(t: T) where T: Display {
    println!("{}", t);
}
```

Multiple trait bounds can be specified for a single type:

```rust
fn compare<T: PartialEq + PartialOrd>(a: T, b: T) -> bool {
    a == b
}
```

This function requires `T` to implement both `PartialEq` (for equality comparison) and `PartialOrd` (for ordering comparison).

Trait bounds can also be used with structs and enums:

```rust
struct Wrapper<T: Display> {
    value: T,
}

impl<T: Display> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}
```

In this example, `Wrapper` is a struct that can hold any type `T`, as long as `T` implements the `Display` trait.

> For more information, you can refer to the [Rust Book's section on Trait Bounds](https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters).