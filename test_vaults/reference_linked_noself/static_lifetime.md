---
bad_links: 
aliases: []
tags: [coding, rust]
---
# Static Lifetime

In Rust, memory safety is ensured through a system of ownership with a set of rules that the compiler checks at compile time. No [[Garbage Collector|garbage collector]] is needed, and no runtime costs are incurred. Among these rules, lifetimes are a way of ensuring that all references are valid. 

The `'static` lifetime is a special lifetime in Rust that represents the entire duration of the program. It's the longest possible lifetime, and it's available for data stored in the binary of your program (Like a [[Static Variables|static variable]]). 

Here's a simple example of a static lifetime:

```rust
let s: &'static str = "hello world";
```

In this case, `s` is a reference to a string literal, which is a `&'static str`. String literals are stored directly in the program binary, which is always available, hence the `'static` lifetime.

The `'static` lifetime is also the default for string literals and for constants. For example:

```rust
static HELLO_WORLD: &str = "Hello, world!";
```

In this case, `HELLO_WORLD` has a `'static` lifetime because it's a constant.

The `'static` lifetime can also be used in the context of [[Static Variables|static variables]], which are variables stored outside any stack frames and exist for the entire duration of the program:

```rust
static mut COUNTER: u32 = 0;

fn add_to_counter(val: u32) {
    unsafe {
        COUNTER += val;
    }
}
```

In this case, `COUNTER` has a `'static` lifetime because it's a [[Static Variables|static variable]].

It's important to note that the `'static` lifetime is inferred in many cases, and you often don't need to explicitly annotate it.

> For more information, you can refer to the [Rust documentation on Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) and the [Rust documentation on 'static](https://doc.rust-lang.org/std/keyword.static.html).