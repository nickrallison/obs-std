---
bad_links: 
aliases: []
tags: [coding, rust]
---
# [[Rust Traits|Trait]] Bounds

[[Rust Traits|Trait]] bounds in Rust are a way to specify that a generic type parameter must implement a particular [[Rust Traits|trait]]. This is useful when you want to use methods from a [[Rust Traits|trait]] on a generic type. 

Here's a basic example:

```rust
fn display<T: Display>(t: T) {
    println!("{}", t);
}
```

In this function, `T: Display` is a [[Rust Traits|trait]] bound. It means "T must implement the Display trait". The `Display` [[Rust Traits|trait]] is part of Rust's standard library and provides a method for formatting the type into a string.

[[Rust Traits|Trait]] bounds can also be specified with the `where` keyword, which can make complex [[Rust Traits|trait]] bounds more readable:

```rust
fn display<T>(t: T) where T: Display {
    println!("{}", t);
}
```

Multiple [[Rust Traits|trait]] bounds can be specified for a single type:

```rust
fn compare<T: PartialEq + PartialOrd>(a: T, b: T) -> bool {
    a == b
}
```

This function requires `T` to implement both `PartialEq` (for equality comparison) and `PartialOrd` (for [[Ordering|ordering]] comparison).

[[Rust Traits|Trait]] bounds can also be used with structs and enums:

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

In this example, `Wrapper` is a struct that can hold any type `T`, as long as `T` implements the `Display` [[Rust Traits|trait]].

> For more information, you can refer to the [Rust Book's section on Trait Bounds](https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters).