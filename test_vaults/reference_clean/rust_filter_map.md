---
bad_links: 
aliases: []
tags: [rust]
---
# Rust Filter Map

The `filter_map` function in Rust is a method available on iterators. It combines the functionalities of `filter` and `map`, two fundamental operations in functional programming, into a single operation. 

The `filter_map` function takes a closure that returns an `Option<T>`. If the closure returns `Some(element)`, the `element` is returned. If the closure returns `None`, the element is skipped. This allows you to simultaneously transform and filter elements in a single pass.

Here is the signature of `filter_map`:

```rust
fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F> 
where
    F: FnMut(Self::Item) -> Option<B>,
    Self: Sized
```

In this signature, `B` is the type of elements after transformation, `F` is the type of the closure, and `Self::Item` is the type of elements in the original iterator.

Here is an example of `filter_map` usage:

```rust
let a = [1, 2, 3];
let b: Vec<_> = a.iter().filter_map(|&x| if x % 2 == 0 { Some(x * 2) } else { None }).collect();
println!("{:?}", b); // prints: [4]
```

In this example, `filter_map` is used to double the value of even numbers and discard odd numbers.

Tangentially related items include the `filter` and `map` functions in Rust. `filter` takes a closure that returns a boolean and only keeps elements for which the closure returns `true`. `map` takes a closure that transforms an element into another. 

There are no specific mathematical formulas, derivations, or proofs directly related to `filter_map`, as it is more of a programming concept. However, it is worth noting that `filter_map`, `filter`, and `map` are all examples of higher-order functions, a fundamental concept in functional programming and lambda calculus.

> For more information, you can refer to the [Rust documentation on filter_map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map).