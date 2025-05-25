[![github]](https://github.com/fuderis/rs-macron/tree/main/macron-regex)&ensp;
[![crates-io]](https://crates.io/crates/macron-regex)&ensp;
[![docs-rs]](https://docs.rs/macron-regex)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Regex Macro

## Introduction:

Creates a new instance of [Regex](https://docs.rs/regex/latest/regex/struct.Regex.html).

P.s.: More useful macros you can find [here](https://docs.rs/macron).


## Examples:

```rust
let re = re!(r"^Hello, \w+!$");

assert!(re.is_match("Hello, World!"));

// with formatting:
let re = re!(r"^Hello, {}!$", "World");

assert!(re.is_match("Hello, World!"));

```

## Licensing:

Distributed under the MIT license.


## Feedback:

You can contact me via GitHub or send a message to my Telegram [@fuderis](https://t.me/fuderis).

This library is constantly evolving, and I welcome your suggestions and feedback.
