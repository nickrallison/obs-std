---
aliases:
tags:
  - coding
  - rust
  - c
bad_links:
---

# [[Pointer.md|Pointer]] Vs. Reference

When discussing pointers and references, especially in the context of coding, it's essential to understand how they differ and function in languages like C and Rust, which have different approaches and philosophies towards memory management and safety.

## In C

C is a procedural programming language that provides low-level access to memory management through the use of pointers. A [[Pointer.md|pointer]] in C is essentially a variable that holds the [[Pointer.md|memory address]] of another variable. Pointers allow for efficient manipulation of arrays, strings, and structures, and they are crucial for dynamic memory allocation, as well as for implementing complex data structures like linked lists, trees, and graphs.

- **Pointers**:
    - Can be null, pointing to no memory location.
    - Allow arithmetic operations (e.g., incrementing to move to the next memory location).
    - Need to be dereferenced to access the data they point to (`*pointer` gives the value).
    - They are somewhat unsafe by nature, as improper use can lead to undefined behavior, memory leaks, or other vulnerabilities (e.g., dangling pointers, buffer overflows).

C does not have native reference types as seen in C++ or Rust. However, pointers can be used to achieve similar behavior where a function argument is modified by passing the address of the variable (a pointer) to the function.

## In Rust

Rust is a systems programming language focusing on safety, especially memory safety, concurrency, and performance. Rust introduces several concepts not found in C, such as ownership, borrowing, lifetimes, and more stringent type checking, to ensure memory safety at compile time without requiring a garbage collector.

- **Pointers**:
    - Rust does have pointers, but they are typically referred to as "raw pointers" (`*const T` for [[Mutability.md|immutable]] pointers and `*mut T` for [[Mutability.md|mutable]] pointers).
    - Raw pointers can be null, and their use is unsafe within Rust's safety guarantees, requiring an `unsafe` block to dereference.
    - Rust's ownership model does not allow arbitrary memory access without explicit markers of unsafe code.

- **References**:
    - References in Rust (`&T` for [[Mutability.md|immutable]] references and `&mut T` for [[Mutability.md|mutable]] references) are always safe to use.
    - They cannot be null and do not allow arithmetic operations.
    - References enforce Rust's [[Rust Ownership.md|borrowing]] rules, ensuring that either one [[Mutability.md|mutable]] reference or any number of [[Mutability.md|immutable]] references to a particular resource exist at any one time, preventing data races at compile time.
    - References are a fundamental part of Rust's memory safety guarantees and are preferred over raw pointers for general use.

Rust's approach to pointers and references is designed to maximize memory safety without sacrificing performance. While C gives programmers full control over memory, leading to powerful but potentially unsafe code practices, Rust enforces strict rules at compile time through its ownership and [[Rust Ownership.md|borrowing]] system, making it more challenging to write unsafe code inadvertently.

In summary, pointers in C offer flexible and direct memory manipulation but come with significant safety risks. In contrast, Rust's system of references and controlled use of pointers aims to provide similar levels of control with a far greater emphasis on safety.