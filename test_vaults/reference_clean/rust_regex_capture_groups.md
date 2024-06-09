---
aliases: []
tags: [rust, coding]
bad_links:
---
# Rust Regex Capture Groups

In Rust, regular expressions (regex) are used to match patterns in strings. A regex engine is used to parse and execute these patterns. The `regex` crate in Rust provides functionality for working with regular expressions.

Capture groups are a part of regular expressions that allow you to not only match text but also extract information for further processing. They are denoted by parentheses `()`. The text that matches the pattern inside the parentheses can be extracted for further use.

Here's a simple example:

```rust
let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
let caps = re.captures("2022-01-01").unwrap();

assert_eq!("2022", &caps[1]);
assert_eq!("01", &caps[2]);
assert_eq!("01", &caps[3]);
```

In this example, `(\d{4})-(\d{2})-(\d{2})` is the regular expression. `\d{4}` matches any four digits, and `(\d{4})` creates a capture group for those four digits. Similarly, `(\d{2})` creates capture groups for two digits. The `-` matches the literal character `-`. The `captures` method returns a `Captures` object which holds the matched string and the contents of all capture groups.

Capture groups can also be named, which can make your code more readable. Named capture groups are denoted by `(?P<name>…)`. Here's an example:

```rust
let re = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
let caps = re.captures("2022-01-01").unwrap();

assert_eq!("2022", &caps["year"]);
assert_eq!("01", &caps["month"]);
assert_eq!("01", &caps["day"]);
```

In this example, `(?P<year>\d{4})` creates a named capture group with the name "year".

Capture groups can also be used for backreferences, which allow you to specify that the contents of a previous capture group must be repeated. Backreferences are denoted by `\1`, `\2`, etc. in the pattern.

Here's an example:

```rust
let re = Regex::new(r"(\d{2})-\1").unwrap();
assert!(re.is_match("12-12"));
assert!(!re.is_match("12-34"));
```

In this example, `(\d{2})-\1` matches two digits followed by a `-` and the same two digits. `\1` is a backreference to the first capture group.

> For more information, you can refer to the [Rust regex crate documentation](https://docs.rs/regex/1.5.4/regex/).