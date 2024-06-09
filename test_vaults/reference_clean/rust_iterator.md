---
bad_links: 
aliases: []
tags: [rust]
---
# Rust Iterator

The `Iterator` trait in Rust is a fundamental part of the language's approach to sequences of items. It is used in a variety of contexts, from looping over arrays to processing streams of data. 

The `Iterator` trait is defined in the Rust standard library as follows:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // methods with default implementation elided
}
```

The `Iterator` trait has one associated type, `Item`, and one required method, `next`. The `Item` type represents the type of the elements being iterated over. The `next` method is called to advance the iterator and returns `Option<Self::Item>`. If the iterator has more items, `next` returns `Some(item)`, and if there are no more items, it returns `None`.

The `Iterator` trait also provides a variety of other methods with default implementations, such as `map`, `filter`, `fold`, and `collect`, which allow for powerful and expressive operations on sequences of items. These methods are implemented in terms of `next`, so any type that implements `Iterator` gets these methods for free.

Here's an example of how you might use an iterator in Rust:

```rust
let vec = vec![1, 2, 3, 4, 5];
let mut iter = vec.iter();

while let Some(v) = iter.next() {
    println!("{}", v);
}
```

In this example, `vec.iter()` returns an iterator over the items in `vec`. The `while let` loop repeatedly calls `next` on the iterator and prints the returned item until `next` returns `None`.

Tangentially related to `Iterator` is the `IntoIterator` trait, which is used to convert a value into an iterator. This trait is used in Rust's `for` loop to allow looping over any value that can be converted into an iterator.

```rust
let vec = vec![1, 2, 3, 4, 5];

for v in vec {
    println!("{}", v);
}
```

In this example, `vec` is converted into an iterator using `IntoIterator`, and the `for` loop repeatedly calls `next` on the iterator until it returns `None`.

> For more information, you can refer to the [Rust documentation on Iterators](https://doc.rust-lang.org/std/iter/trait.Iterator.html) and the [Rust Book's chapter on Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html).