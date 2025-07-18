[![github]](https://github.com/fuderis/rs-macron/tree/main/macron-path)&ensp;
[![crates-io]](https://crates.io/crates/macron-path)&ensp;
[![docs-rs]](https://docs.rs/macron-path)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# File Path Macro

A macro for ergonomic and safe construction of file and directory paths in Rust, with context-aware resolution.

* Supports both string literals and formatted paths with arguments.
* Automatically determines whether to allow a path relative to the current working directory, the executable directory, or the user data directory.

> _P.S. More useful macros are available in the [macron](https://github.com/fuderis/rs-macron.git) package._


## Features

* __Relative to current working directory__: If the path does not start with / or \, it is resolved relative to the current working directory (std::env::current_dir()).
* __Relative to executable's directory__: If the path starts with / or \, it is resolved relative to the directory where the executable resides (std::env::current_exe()).
* __Supports formatting__: You can use string formatting just like with the standard format! macro.


## Examples:

```rust
use macron_path::path;

// Path relative to the current working directory
let relative_path = path!("foo/bar");

// Path relative to the executable's directory
let root_path = path!("/foo/bar");

// Path relative to the user data directory for saving applications data
let root_path = path!("$/foo/bar");

// Supports string formatting
let fmt_path = path!("foo/{}", "bar");
```

## Notes:

* __Windows__: Both / and \ are supported as path separators.

* __Linux/macOS__: Only / is treated as a directory separator. For best results, use / or build paths using the macro or standard library methods.

* __Always use the path!__: macro to avoid issues with separators and path resolution differences across platforms.


## Licensing:

Distributed under the _MIT license_.


## Feedback:

You can contact me via GitHub or send a message to my _Telegram_ [@fuderis](https://t.me/fuderis).

This library is actively evolving, and your suggestions and feedback are always welcome!

> _P.S. More useful macros are available in the [macron](https://github.com/fuderis/rs-macron.git) package._
