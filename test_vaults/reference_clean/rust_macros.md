---
bad_links: 
aliases: [macro]
tags: [coding]
title: Rust Macros
date created: Sunday, July 16th 2023, 4:41:08 pm
---
# Rust Macros

Rust macros are powerful features in the Rust programming language that allow for metaprogramming, which is the creation of code that manipulates other code. Macros work by taking in code as an argument and producing code as a result. They are useful for reducing redundancy, improving code readability, and enabling complex, flexible functionality. There are two types of macros in Rust: declarative macros defined with macro_rules! and procedural macros which include custom derive, attribute-like and function-like macros.

Sure, here are examples of both types of Rust macros:

1. Declarative Macros:
```rust
macro_rules! say_hello {
    () => (
        println!("Hello, world!");
    )
}

fn main() {
    say_hello!();
}
```
In this example, we define a simple macro `say_hello!` that prints "Hello, world!" when called.

2. Procedural Macros:
```rust
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

fn main() {
    make_answer!();
    println!("The answer is: {}", answer());
}
```
In this example, we define a procedural macro `make_answer!` that generates a function `answer()` which returns 42 when called. Note that procedural macros are more complex and require the use of the `proc_macro` crate.
