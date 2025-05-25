[![github]](https://github.com/fuderis/rs-macron/tree/main/macron-input)&ensp;
[![crates-io]](https://crates.io/crates/macron-input)&ensp;
[![docs-rs]](https://docs.rs/macron-input)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Read User Inputs

## Introduction:

Reads user input lines from the console


## Examples:

```rust
let mut input = input!("Enter in order '0', '1', '2': ");

for i in 0..=2 {
    if let Ok(text) = input.next().unwrap() {
        assert_eq!(text, format!("{i}"));
    }
}
```

## Licensing:

Distributed under the MIT license.


## Feedback:

You can contact me via GitHub or send a message to my Telegram [@fuderis](https://t.me/fuderis).

This library is constantly evolving, and I welcome your suggestions and feedback.
