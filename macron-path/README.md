[![github]](https://github.com/fuderis/rs-macron/tree/main/macron-path)&ensp;
[![crates-io]](https://crates.io/crates/macron-path)&ensp;
[![docs-rs]](https://docs.rs/macron-path)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# PathBuf Macro

A macro for ergonomic and safe construction of file and directory paths in Rust, with context-aware resolution.

* Supports both string literals and formatted paths with arguments.
* Automatically determines whether to allow a path relative to the current working directory, the executable directory, or the user data directory.

> _P.S. More useful macros are available in the [macron](https://github.com/fuderis/rs-macron.git) package._


## Features:

- **Empty call**: `path!()` returns `PathBuf::new()`.
- **Executable prefix `$`**: `path!("$")` → `std::env::current_exe()`.
- **Executable directory `$/`**: `path!("$/foo/bar")` → `current_exe().parent().join("foo/bar")`.
- **Home directory `~/`**: `path!("~/foo/bar")` → `$HOME/foo/bar` (`USERPROFILE` on Windows).
- **User data directory `%/`**:
  | OS       | Default Directory                      |
  |----------|----------------------------------------|
  | Windows  | `C:/Users/User/AppData/Roaming/`       |
  | macOS    | `~/Library/Application Support/`       |
  | Linux    | `~/.local/share`                       |
- **Formatting**: `path!("foo/{arg1}/{}", arg2)`.
- **Expressions**: `path!(some_path_var)`.
- **Plain paths**: `path!("foo/bar")`.


## Examples:

```rust
use macron_path::path;

// empty PathBuf:
let empty = path!();

// plain path (relative to CWD):
let relative = path!("foo/bar");

// executable path:
let exe_path = path!("$");

// executable directory:
let exe_dir_path = path!("$/config.toml");

// home directory:
let home_path = path!("~/Documents/file.txt");

// user data directory:
let data_path = path!("%/myapp/settings.json");

// formatting:
let user_path = path!("users/{}/docs", "alice");
let dynamic = path!("dir/{}", some_var.to_string());
```

## Licensing:

Distributed under the _MIT license_.


## Feedback:

You can contact me via GitHub or send a message to my _Telegram_ [@fuderis](https://t.me/fuderis).
This library is actively evolving, and your suggestions and feedback are always welcome!
<br>
> _P.S. More useful macros are available in the [macron](https://github.com/fuderis/rs-macron.git) package._
