---
bad_links: 
aliases: []
tags: [rust]
---
# Rust String vs. OsString

In Rust, `String` and `OsString` are two distinct types used for storing string data, each with its own use cases and characteristics.

1. **String**: This is a UTF-8 encoded growable string. It's the most common string type used in Rust and is often used for text manipulation. It guarantees that the data it holds is valid UTF-8, which makes it safe to perform operations that expect valid UTF-8 strings. However, this also means that it can't directly handle strings that aren't valid UTF-8.

2. **OsString**: This type is used for interfacing with the operating system. It's a more flexible string type that can hold any sequence of bytes. This is necessary because file names, command line arguments, and other OS-level strings can contain any sequence of bytes, not just valid UTF-8. `OsString` can be converted to and from `String`, but these conversions can fail if the `OsString` contains non-UTF-8 data.

The relationship between `String` and `OsString` can be understood in terms of the type system's guarantees about the data they hold. `String` provides stronger guarantees (i.e., that its data is valid UTF-8), but is less flexible. `OsString` provides weaker guarantees (i.e., that its data is some sequence of bytes), but is more flexible.

Here's a simple example of how you might use these types:

```rust
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;

let os_string = OsString::from_vec(vec![65, 66, 67, 255]); // ABC and a non-UTF-8 character
let string = String::from("ABC"); // A UTF-8 string

// Converting OsString to String can fail
let converted = os_string.into_string();
match converted {
    Ok(s) => println!("Converted: {}", s),
    Err(os_string) => println!("Failed to convert: {:?}", os_string),
}

// Converting String to OsString always succeeds
let os_string: OsString = string.into();
println!("Converted: {:?}", os_string);
```

In this example, converting the `OsString` to a `String` fails because it contains a non-UTF-8 character. But converting the `String` to an `OsString` succeeds because `OsString` can hold any sequence of bytes.

> For more information, you can refer to the Rust documentation on [String](https://doc.rust-lang.org/std/string/struct.String.html) and [OsString](https://doc.rust-lang.org/std/ffi/struct.OsString.html).