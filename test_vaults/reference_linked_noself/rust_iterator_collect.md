---
aliases: []
tags: [rust]
bad_links:
---
# Rust Iterator Collect

The `collect` function is a method provided by the Iterator [[Rust Traits|trait]] in Rust. It's used to transform an iterator into a collection. The function is quite versatile and can produce different types of collections based on the context in which it's used. Here's a basic usage example:

```rust
let v: Vec<_> = (0..10).collect();
```

In this example, `collect` is used to transform a range (which is an iterator) into a vector. The type of the vector is inferred from the context.

The `collect` function is defined as follows in the Iterator [[rust_traits.md|trait]]:

```rust
fn collect<B: FromIterator<Self::Item>>(self) -> B
where
    Self: Sized,
```

This function takes `self` by value (consuming the iterator), and returns a collection of type `B`. The type `B` must implement the `FromIterator` [[Rust Traits|trait,]] which is implemented by many collection types in Rust's standard library.

The `FromIterator` [[Rust Traits|trait]] is defined as follows:

```rust
pub trait FromIterator<A>: Sized {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}
```

The `from_iter` function takes an iterator and transforms it into a collection. This is the function that `collect` uses to create the collection.

The `collect` function is essentially a bridge between the Iterator [[Rust Traits|trait]] and the FromIterator [[Rust Traits|trait.]] It allows you to transform an iterator into a collection in a generic way.

The `collect` function is quite powerful and can be used in a variety of ways. For example, you can use it to transform an iterator of Results into a Result of a collection, handling errors along the way:

```rust
let results: Vec<Result<_, _>> = vec![Ok(1), Ok(2), Ok(3)];
let collection: Result<Vec<_>, _> = results.into_iter().collect();
```

In this example, if all the Results in the iterator are `Ok`, `collect` will return an `Ok` containing a collection of the `Ok` values. If any of the Results are `Err`, `collect` will return the first `Err` it encounters.

> For more information, you can refer to the [Rust documentation on the Iterator trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html) and the [Rust documentation on the FromIterator trait](https://doc.rust-lang.org/std/iter/trait.FromIterator.html).