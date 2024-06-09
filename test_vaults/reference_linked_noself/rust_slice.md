---
bad_links: 
aliases: []
tags: [rust]
---
# Rust Slice

Rust Slices are a data type that does not have ownership. They let you reference a contiguous sequence of elements in a collection rather than the whole collection. Here's how you define a slice:

```rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```

In this example, `hello` and `world` are slices that reference the first and second word of the string `s`, respectively.

The starting index in the range is included in the slice, and the ending index is excluded. This is also known as half-open range syntax, and it's represented as `[start..end]`. If you want to start at the first index (zero), you can drop the value before the two periods. Similarly, if you want to slice to the end of the string, you can drop the trailing number:

```rust
let s = String::from("hello");
let slice = &s[0..2];
let slice = &s[..2];
```

Both of these slices would be `he`.

Slices are useful because they prevent you from creating bugs with off-by-one errors, which are common when dealing with direct indexing. They also increase the readability of your code.

Slices are a reference to a part of the memory. Therefore, just like references, slices are read-only by default. You can make them [[Mutability|mutable]] by declaring the reference as mutable:

```rust
let mut s = String::from("hello world");
let word = first_word(&mut s);
```

In this case, `first_word` is a function that returns a slice that contains the first word of the sentence.

Slices can be used with other data structures as well, such as arrays:

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```

This slice has the type `&[i32]` and works the same way as string slices do, but with `i32` values instead of characters.

> For more information, you can refer to the [Rust documentation on slices](https://doc.rust-lang.org/book/ch04-03-slices.html).