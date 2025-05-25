[![github]](https://github.com/fuderis/rs-macron/tree/main/macron-impl-into)&ensp;
[![crates-io]](https://crates.io/crates/macron-impl-into)&ensp;
[![docs-rs]](https://docs.rs/macron-impl-into)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Impl Into Macro

## Introduction:

The implementation of trait [Into](std::convert::Into)


## Examples:

```rust
#[derive(Into, Debug, PartialEq)]
#[into(Insertion, "self.insertion")]
struct Test {
    insertion: Insertion,
}

#[derive(Debug, PartialEq)]
struct Insertion;

assert_eq!(Insertion {}, Test { insertion: Insertion {} }.into());
```
```rust
#[derive(Into, Debug, PartialEq)]
#[into(Insertion, "match self { Self::Insertion(ins) => ins }")]
enum Test {
    Insertion(Insertion),
}

#[derive(Debug, PartialEq)]
struct Insertion;

assert_eq!(Insertion {}, Test::Insertion(Insertion {}).into());
```

## Licensing:

Distributed under the MIT license.


## Feedback:

You can contact me via GitHub or send a message to my Telegram [@fuderis](https://t.me/fuderis).

This library is constantly evolving, and I welcome your suggestions and feedback.
