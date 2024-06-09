---
aliases: []
tags: [rust]
bad_links:
---
# Rust Env and Option_env Macros

The `env!` and `option_env!` macros in Rust are used to access environment variables at compile time.

1. **env! [[Rust Macros.md|macro]]**: The `env!` macro allows you to access the value of an environment variable during the compilation process. It takes a string literal representing the name of the environment variable as an argument and returns the value of the environment variable as a static string (`&'static str`). If the environment variable is not set, the `env!` macro will cause a compile-time error. Here is an example of its usage:

```rust
let path = env!("PATH");
println!("The PATH variable is: {}", path);
```

1. **option_env! [[Rust Macros.md|macro]]**: The `option_env!` macro is similar to `env!`, but instead of causing a compile-time error when the environment variable is not set, it returns an `Option<&'static str>`. If the environment variable is set, it returns `Some(value)`, otherwise, it returns `None`. Here is an example of its usage:

```rust
match option_env!("SOME_VARIABLE") {
    Some(val) => println!("The variable is: {}", val),
    None => println!("The variable is not set"),
}
```

These macros are part of Rust's standard library and are used to embed environment-specific information into the compiled program. They are evaluated at compile time, which means the environment variable values are fixed once the program is compiled.

Tangentially related to these macros are the `std::env::var` and `std::env::var_os` functions, which are used to access environment variables at runtime. `env!` and `option_env!`, these functions can be used to access environment variables that are set or changed after the program is compiled.

> For more information, you can refer to the official Rust documentation on [`env!`](https://doc.rust-lang.org/std/macro.env.html) and [`option_env!`](https://doc.rust-lang.org/std/macro.option_env.html) macros.