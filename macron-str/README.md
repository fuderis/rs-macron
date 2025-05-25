[![github]](https://github.com/fuderis/rs-macron/tree/main/macron-str)&ensp;
[![crates-io]](https://crates.io/crates/macron-str)&ensp;
[![docs-rs]](https://docs.rs/macron-str)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# String Macro

## Introduction:

Creates a new instance of [String](https://doc.rust-lang.org/stable/std/string/struct.String.html)


## Examples:

```rust
// simple string:
let s = str!("Hello, World!");

assert_eq!(s, "Hello, World!");

// from integer:
let s = str!(10);

assert_eq!(s, "10");

// from refference:
let r = 10.2;
let s = str!(r);

assert_eq!(s, "10.2");

// string formatting with arguments:
let s = str!("Hello, {}!", "World");

assert_eq!(s, "Hello, World!");

// string formatting with named arguments:
let name = "World";
let s = str!("Hello, {name}!");

assert_eq!(s, "Hello, World!");
```

## Licensing:

Distributed under the MIT license.


## Feedback:

You can contact me via GitHub or send a message to my Telegram [@fuderis](https://t.me/fuderis).

This library is constantly evolving, and I welcome your suggestions and feedback.
