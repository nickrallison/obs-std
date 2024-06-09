---
bad_links:
aliases:
tags:
  - rust
---
# Rust Struct Methods and Ownership

In Rust, methods can borrow (`&self`), mutably borrow (`&mut self`), or take ownership (`self`) of the instance they are called on. However, by default, methods borrow the instance, not take ownership. This is because of Rust's ownership model, which is designed to prevent data races at compile time.

1. **Ownership and [[Rust Ownership.md|Borrowing]]**: In Rust, each value has a variable that's called its owner. There can only be one owner at a time. When the owner goes out of scope, the value will be dropped. Borrowing allows you to have a reference to a value without taking ownership. This is important because it allows you to use a value without the risk of it being dropped when you're done.

2. **Methods and [[Rust Ownership.md|Ownership]]**: If struct methods took ownership by default, you would not be able to call any other methods on that instance after the first one, because the first method call would move the instance. This would be very limiting. By borrowing the instance instead, Rust allows you to call multiple methods on the same instance.

3. **[[Mutability.md|Mutable]] [[Rust Ownership.md|Borrowing]]**: If you need to modify the instance, you can use mutable borrowing (`&mut self`). This still doesn't take ownership, but it does prevent any other references to the instance for as long as the mutable borrow lasts, preventing data races.

4. **Taking Ownership**: There are cases where you might want a method to take ownership (`self`). This is usually when the method transforms the instance into something else and you want to prevent any further use of the original instance.

Here's a simple example to illustrate these concepts:

```rust
struct Foo {
    x: i32,
}

impl Foo {
    fn borrow(&self) {
        println!("{}", self.x);
    }

    fn borrow_mut(&mut self) {
        self.x += 1;
    }

    fn take(self) -> i32 {
        self.x
    }
}

fn main() {
    let mut foo = Foo { x: 42 };
    foo.borrow(); // borrows `foo`
    foo.borrow_mut(); // mutably borrows `foo`
    let x = foo.take(); // takes ownership of `foo`
    // foo is now moved and can't be used anymore
}
```
