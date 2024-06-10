---
aliases:
tags:
  - rust
  - coding
bad_links:
---
# Rust ToOwned Function

The `ToOwned` [[rust_traits.md|trait]] in Rust is crucial for creating owned types from borrowed types. It is primarily used within the context of converting a borrowed data (`&T`) to its owned counterpart (`T`), where `T` can be owned or cloned into a newly allocated instance.

## What is `ToOwned`?

In the standard library, `ToOwned` is defined as a [[rust_traits.md|trait]] which is used to generalize the concept of cloning to a more abstract level. It permits a more decoupled approach wherein a user of the data does not need to know the specifics of how to copy or clone the underlying data.

```rust
pub trait ToOwned {
    type Owned: Borrow<Self>;
    fn to_owned(&self) -> Self::Owned;
}
```

The [[rust_traits.md|trait]] has an associated type, `Owned`, which is what the `to_owned` method will return. This [[rust_traits.md|trait]] is extensively applied in types that have a borrowed version, `&T` and an owned version, `T`. For example, in the case of strings, `&str` is a borrowed reference of `String`.

## Usage of `ToOwned`

The `ToOwned` [[rust_traits.md|trait]] provides a generic way to move from borrowed to owned data, which is especially useful in scenarios where the type of conversion might not be directly obvious or when you are writing functions that strive to be generic.

### Example 1: Using `to_owned` with `str`

```rust
let my_str = "Hello, Rust!";
let my_string = my_str.to_owned();
println!("{}", my_string); // Outputs: Hello, Rust!
```

Here, `to_owned` is used to convert from the string slice `&str` to the `String` type.

### Example 2: Custom Types Implementing `ToOwned`

Implementing `ToOwned` can be beneficial for custom data types where cloning or exact copies are necessary. Here is a simple custom type `Point` for which `ToOwned` is implemented:

```rust
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl ToOwned for Point {
    type Owned = Point;

    fn to_owned(&self) -> Self::Owned {
        Point { x: self.x, y: self.y }
    }
}

let point = Point { x: 1, y: 2 };
let owned_point = point.to_owned();
println!("{:?}", owned_point); // Outputs: Point { x: 1, y: 2 }
```

## Conclusion

`ToOwned` thus serves as a powerful component in Rust's abstraction arsenal, allowing for flexible conversion between borrowed and owned types. By implementing this [[rust_traits.md|trait]], it becomes possible to handle data in a more generic fashion without tying down to specific cloning implementations, allowing Rust programs to be more modular and reusable. This [[rust_traits.md|trait]] effectively underpins Rust's efficient memory management capabilities, ensuring safety without sacrificing performance.