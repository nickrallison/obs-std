---
aliases: []
tags: [rust, coding]
bad_links:
---
# Rust Turbofish Syntax

The Turbofish syntax in Rust is a way to explicitly specify generic parameters in function calls or type instantiations. It's called "Turbofish" because the syntax looks like a fish (`::<>`). 

Here's a simple example of Turbofish syntax:

```rust
let v: Vec<u32> = Vec::new();
let v = Vec::<u32>::new(); // equivalent to the above line using Turbofish
```

In the above example, `Vec::<u32>::new()` is equivalent to `Vec::new::<u32>()`. The `::<u32>` part is the Turbofish syntax, which explicitly specifies that the `Vec` is of type `u32`.

The Turbofish syntax is particularly useful in situations where the Rust compiler cannot infer the type of the generic parameters. For example, consider the `parse` method, which is a method that converts a string into some other type:

```rust
let s = "42";
let x = s.parse().unwrap(); // Error: type annotations needed
```

In the above code, the Rust compiler cannot infer the type that `s` should be parsed into, so it throws an error. We can fix this by using the Turbofish syntax to explicitly specify the type:

```rust
let s = "42";
let x = s.parse::<i32>().unwrap(); // No error
```

In the above code, `s.parse::<i32>().unwrap()` tells the Rust compiler to parse `s` into an `i32`.

The Turbofish syntax is also used in situations where you want to call associated functions (functions associated with a type, not an instance of a type) on a generic type. For example:

```rust
fn foo<T: Default>() {
    let t = T::default(); // Error: type annotations needed
}
```

In the above code, the Rust compiler cannot infer the type of `T`, so it throws an error. We can fix this by using the Turbofish syntax to explicitly specify the type:

```rust
fn foo<T: Default>() {
    let t = T::default::<T>(); // No error
}
```

In the above code, `T::default::<T>()` tells the Rust compiler to call the `default` function on the type `T`.

> For more information, you can refer to the ["Why Not"](https://stevedonovan.github.io/rustifications/2017/09/08/common-rust-lifetime-misconceptions.html) section of the Rust documentation, which explains why the Turbofish syntax is necessary in certain situations. You can also refer to the ["Type ascription"](https://github.com/rust-lang/rfcs/blob/master/text/0803-type-ascription.md) RFC, which proposes a syntax for explicitly specifying types in more situations.