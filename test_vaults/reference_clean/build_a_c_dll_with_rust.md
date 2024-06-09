---
aliases: 
tags:
  - coding
  - rust
  - compilers
bad_links:
---
# Build A C DLL With Rust

In the `[lib]` section of the cargo.toml file, make sure to include this line: crate-type = ["cdylib"]

```toml
[package]
name = "rust_c_dlib"
version = "0.1.0"
edition = "2021"

[lib]
name = "rust_c_dlib"
crate-type = ["cdylib"]
```

The functions being exposed should look like this:
```rust
#[no_mangle]
pub extern fn add(left: usize, right: usize) -> usize {
    left + right
}
```

When you run `nm -D target/*/*.so`, you should see the add function in the symbol table