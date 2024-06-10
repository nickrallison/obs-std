---
bad_links: 
aliases: []
tags: [rust]
---
# Rust Path vs. PathBuf

In Rust, `Path` and `PathBuf` are two types that are used to work with file system paths. They are part of the `std::path` module.

1. `Path`: This is an [[Mutability|immutable]] sequence of components representing a file system path. It's essentially a slice (similar to `str`), meaning it's a reference to some path data stored elsewhere. It's used when you want to [[Rust Ownership|borrow]] a path.

2. `PathBuf`: This is an owned, [[Mutability|mutable]] version of `Path` (similar to `String`). It's used when you want to create or modify paths. 

The relationship between `Path` and `PathBuf` is similar to the relationship between `str` and `String` in Rust. `Path` is a borrowed reference to a path, and `PathBuf` is an owned path.

Here's a simple example of how you might use these types:

```rust
use std::path::{Path, PathBuf};

let path = Path::new("/tmp/foo"); // `path` is a `&Path`
let mut path_buf = PathBuf::new(); // `path_buf` is a `PathBuf`
path_buf.push("/tmp/bar"); // You can modify `path_buf`
```

In this example, `path` is a `&Path` that cannot be modified (because `Path` is immutable), while `path_buf` is a `PathBuf` that can be modified.

As for tangentially related items, it's worth noting that `Path` and `PathBuf` are designed to be used with Rust's `std::fs` module, which provides functions for working with the file system. For example, you might use `Path` or `PathBuf` with `std::fs::read_to_string` to read the contents of a file into a string.

> For more information, you can refer to the [Rust documentation on `std::path`](https://doc.rust-lang.org/std/path/index.html) and the [Rust book's chapter on file I/O](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html).