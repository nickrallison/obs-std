---
bad_links: 
aliases: []
tags: [rust]
---
# Rust Fragment Specifiers

In Rust, [[Rust Macros|macros]] are a way of defining reusable chunks of code. They are defined with a syntax similar to function syntax, but instead of producing a value, they generate code. Fragment specifiers are used in the definition of these macros to indicate the type of argument a [[Rust Macros|macro]] should accept.

Here are the fragment specifiers available in Rust:

1. `ident`: an identifier. Examples: `x`; `foo`.
2. `path`: a qualified name. Example: `T::SpecialA`.
3. `expr`: an expression. Examples: `2 + 2`; `if true { 1 } else { 2 }`; `f(42)`.
4. `ty`: a type. Examples: `i32`; `Vec<(char, String)>`; `&T`.
5. `pat`: a pattern. Examples: `Some(t)`; `(17, 'a')`; `_`.
6. `stmt`: a single statement. Example: `let x = 3`.
7. `block`: a brace-delimited sequence of statements. Example: `{ let x = 3; x }`.
8. `item`: an [item](https://doc.rust-lang.org/reference/items.html). Examples: `fn foo() { }`; `struct Bar;`.
9. `meta`: a "meta item", as found in attributes. Example: `cfg(target_os = "windows")`.
10. `tt`: a single [[Compiler Tokens|token]] tree. 

Each of these specifiers corresponds to a particular grammar production in the Rust language. For example, `expr` corresponds to an expression, `ty` to a type, and so on. When you write a [[Rust Macros|macro,]] you use these specifiers in the definition to indicate what kind of Rust syntax you expect in each part of the [[Rust Macros|macro]] invocation.

Here's an example of a simple [[Rust Macros|macro]] that uses fragment specifiers:

```rust
macro_rules! create_function {
    ($func_name:ident) => (
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name))
        }
    )
}

create_function!(foo);
foo();
```

In this example, `$func_name:ident` is a fragment specifier that matches any identifier (like a function or variable name). The `stringify!` [[Rust Macros|macro]] turns `foo` into a string `"foo"` for the `println!` [[Rust Macros|macro.]]

> For more information, you can refer to the [Rust documentation on Macros](https://doc.rust-lang.org/book/ch19-06-macros.html) and [Rust reference on Macro By Example](https://doc.rust-lang.org/reference/macros-by-example.html).