---
bad_links: 
aliases: []
tags: [rust]
---
# Rust Result Enum

The `Result` enum in Rust is a powerful tool for handling the possibility of failure. In many programming languages, error handling is done using exceptions. Rust does not have exceptions. Instead, it has the type `Result<T, E>` for recoverable errors and the `panic!` [[Rust Macros|macro]] that stops execution when the program encounters an unrecoverable error.

The `Result` enum is defined as follows:

```rust
pub enum Result<T, E> {
   Ok(T),
   Err(E),
}
```

This enum has two variants:

1. `Ok`: An operation was successful, and it contains the resulting value of type `T`.
2. `Err`: An operation failed, and it contains an error of type `E`.

Here's an example of a function that returns a `Result`:

```rust
use std::fs::File;

fn open_file(filename: &str) -> Result<File, std::io::Error> {
    File::open(filename)
}
```

In this example, the `open_file` function returns a `Result` type. If the file is successfully opened, it returns `Ok` containing the file handle. If it fails, it returns `Err` containing an error.

The `Result` type is intended to be used with the `match` statement, which can be used to handle both the `Ok` and `Err` cases:

```rust
match open_file("file.txt") {
    Ok(file) => {
        // use file
    },
    Err(error) => {
        // handle error
    }
}
```

Rust also provides the `?` operator, which can be used to propagate errors up the call stack. It works by returning early if the `Result` is `Err`, otherwise it unwraps the `Ok` variant:

```rust
fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = open_file(filename)?;
    // continue with file reading
}
```

In this example, if `open_file` returns an `Err`, the `read_file` function will immediately return that `Err`. If `open_file` returns `Ok`, the `?` operator unwraps the `Ok` variant, and `file` is assigned the file handle.

> For more information, you can refer to the [Rust Book's chapter on Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html).