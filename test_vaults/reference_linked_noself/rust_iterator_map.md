---
aliases: []
tags: [rust]
bad_links:
---
# Rust Iterator Map

The `map` method in Rust is a part of the `Iterator` [[Rust Traits|trait]]. It is a higher-order function that takes a [[closure.md|closure]] and creates an iterator that applies the [[closure.md|closure]] to every element of the underlying iterator.

Here is the signature of the `map` function:

```rust
fn map<B, F>(self, f: F) -> Map<Self, F>
where
    F: FnMut(Self::Item) -> B,
```

In this signature, `B` is the type of items that the [[closure.md|closure]] `f` returns, and `F` is the type of the [[closure.md|closure]]. The `map` function takes `self` and a [[closure.md|closure]] `f` as arguments, and returns a `Map` struct that contains the original iterator and the [[closure.md|closure]].

The `Map` struct is defined as follows:

```rust
pub struct Map<I, F> {
    iter: I,
    f: F,
}
```

Here, `I` is the type of the underlying iterator and `F` is the type of the [[closure.md|closure]].

The `Map` struct also implements the `Iterator` [[Rust Traits|trait.]] The `next` method of the `Iterator` [[Rust Traits|trait]] is implemented for the `Map` struct as follows:

```rust
fn next(&mut self) -> Option<B> {
    self.iter.next().map(|a| (self.f)(a))
}
```

In this implementation, the `next` method calls the `next` method of the underlying iterator and applies the [[closure.md|closure]] `f` to the result. The `map` method of the `Option` enum is used here, which applies the [[closure.md|closure]] to the `Some` variant and does nothing to the `None` variant.

The `map` method is used to transform the elements of an iterator. Here is an example:

```rust
let v = vec![1, 2, 3];
let v_squared: Vec<_> = v.iter().map(|&x| x * x).collect();
assert_eq!(v_squared, [1, 4, 9]);
```

In this example, the `map` method is used to square each element of the vector `v`. The result is collected into a new vector `v_squared`.

> For more information, you can refer to the [Rust documentation on Iterator::map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map).