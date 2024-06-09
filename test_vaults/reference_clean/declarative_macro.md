---
bad_links: 
aliases: []
tags: [coding, rust]
title: Rust Macros
date created: Sunday, July 16th 2023, 4:41:08 pm
---
# Declarative Macro

Declarative macros are powerful features in the Rust programming language that allow for metaprogramming, which is the creation of code that manipulates other code. Macros work by taking in code as an argument and producing code as a result. They are useful for reducing redundancy, improving code readability, and enabling complex, flexible functionality.

Here is an example of a declarative macro
```rust
macro_rules! say_hello {
    () => (
        println!("Hello, world!");
    );
    ($s:ident) => (
        println!("Hello, {}!", $s);
    );
}

fn main() {
    say_hello!();
}
```
In this example, we define a simple macro `say_hello!` that prints "Hello, world!" when called.

Similarly there exists procedural macros for more complex behavior at compile-time

