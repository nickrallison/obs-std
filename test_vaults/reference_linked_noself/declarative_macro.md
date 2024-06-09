---
bad_links: 
aliases: []
tags: [coding, rust]
title: Rust Macros
date created: Sunday, July 16th 2023, 4:41:08 pm
---
# Declarative [[Rust Macros|Macro]]

Declarative [[Rust Macros|macros]] are powerful features in the Rust programming language that allow for [[Metaprogramming|metaprogramming]], which is the creation of code that manipulates other code. [[Rust Macros|Macros]] work by taking in code as an argument and producing code as a result. They are useful for reducing redundancy, improving code readability, and enabling complex, flexible functionality.

Here is an example of a declarative [[Rust Macros|macro]]
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
In this example, we define a simple [[Rust Macros|macro]] `say_hello!` that prints "Hello, world!" when called.

Similarly there exists [[Procedural Macro|procedural macros]] for more complex behavior at compile-time

