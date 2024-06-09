---
bad_links: 
aliases: []
tags: [coding, rust]
---
# Procedural [[Rust Macros|Macro]]

Procedural [[Rust Macros|macros]] in Rust are a type of [[Rust Macros|macro]] that accept some code as an input and produce some code as an output. They are more flexible than [[Declarative Macro|declarative macros]], as they allow for more complex manipulations of the input code. Procedural [[Rust Macros|macros]] are defined using the Rust programming language itself, and they are expanded at compile time. They come in three types: custom derive, attribute-like, and function-like. Procedural [[Rust Macros|macros]] are powerful tools for [[Metaprogramming|metaprogramming]] in Rust, but they can also be more difficult to write and understand than [[Declarative Macro|declarative macros]].

Take the following example code
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
In this example, we define a procedural [[Rust Macros|macro]] `make_answer!` that generates a function `answer()` which returns 42 when called. Note that procedural [[Rust Macros|macros]] are more complex than [[Declarative Macro|declarative macros]] and require the use of the `proc_macro` crate.

## Sources

(Youtube Video)[https://www.youtube.com/watch?v=MWRPYBoCEaY]