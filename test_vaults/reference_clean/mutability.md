---
aliases:
  - mut
  - mutable
  - mutate
  - immutable
  - immutability
  - mutable reference
  - immutable reference
tags:
  - coding
  - rust
bad_links:
---
# Mutability
In the context of coding and especially in Rust, mutability refers to the ability of data to be changed after it is initially created. Rust places a strong emphasis on safety and concurrency, and as such, it handles mutability in a way that is somewhat unique compared to other programming languages.

## Basics of Mutability in Rust

Rust has a powerful type system and memory safety guarantees that enforce how data is accessed and modified. Here are the key concepts:

### Immutable by Default

Variables in Rust are immutable by default. This means that once a variable is assigned a value, it cannot be changed unless explicitly marked as mutable.

```rust
let x = 5;
x = 6; // This will cause a compile-time error

let mut y = 5;
y = 6; // This is allowed because `y` is mutable
```

### Ownership and Borrowing

Rusts ownership system ensures safe memory management. Ownership, along with the rules for borrowing and lifetimes, plays a crucial role in how mutability is managed.

- **Ownership**: Each value in Rust has a single owner, and when the owner goes out of scope, the value is dropped.
- **[[Rust Ownership.md|Borrowing]]**: Rust allows references to values, and these can be either mutable or immutable.
- **Lifetimes**: Lifetimes ensure that references are always valid as long as they are in use.

### Mutable References

To modify data, Rust uses mutable references. However, Rust enforces a rule that there can be either one mutable reference or any number of immutable references to a piece of data in a particular scope, but not both. This rule helps prevent data races at compile time.

```rust
let mut s = String::from("hello");

{
    let r1 = &s; // OK: first immutable reference
    let r2 = &s; // OK: another immutable reference
    // let m = &mut s; // ERROR: cannot borrow `s` as mutable because it is also borrowed as immutable
}

let m = &mut s; // OK: mutable reference is allowed here as no immutable references are in scope
```

### Interior and Exterior Mutability

Rust also differentiates between interior and exterior mutability:

- **Interior Mutability**: A design pattern in Rust that allows you to mutate data even when there are immutable references to that data, by safely encapsulating the mutation.
- **Exterior Mutability**: The standard mutability model where you explicitly mark variables as mutable with the `mut` keyword.

A common use case for interior mutability is when using concurrency or when you want to cache results inside a struct without altering its external API.

Interior mutability is achieved using types like `Cell<T>`, `RefCell<T>`, `Mutex<T>`, and `RwLock<T>`, which follow the rules of borrowing and ownership but allow for mutation within a controlled environment.

## Conclusion

Mutability in Rust might seem restrictive at first, but it is a core part of the language�s focus on safety and concurrency. By requiring explicit labeling of mutable variables and enforcing strict borrowing rules, Rust aims to eliminate a whole class of bugs that are common in other languages. Understanding and working within these constraints is key to becoming proficient in Rust.