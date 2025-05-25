[![github]](https://github.com/fuderis/rs-macron/tree/main/macron-impl-from)&ensp;
[![crates-io]](https://crates.io/crates/macron-impl-from)&ensp;
[![docs-rs]](https://docs.rs/macron-impl-from)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Impl From Macro

## Introduction:

The implementation of trait [From](std::convert::From).

P.s.: More useful macros you can find [here](https://docs.rs/macron).


## Examples:

```rust
#[derive(From, Debug, PartialEq)]
#[from(Insertion, "Self { insertion: value }")]
struct Test {
    insertion: Insertion,
}

#[derive(Debug, PartialEq)]
struct Insertion;

assert_eq!(Test { insertion: Insertion {} }, Test::from(Insertion {}));
```
```rust
#[derive(From, Debug, PartialEq)]
#[from(Insertion, "Self::Insertion(value)")]
enum Test {
    Insertion(Insertion),

    #[from(Insertion2, "value")]
    Insertion2(Insertion2),

    #[from(Insertion3, "insertion: value")]
    Insertion3 { insertion: Insertion3 },

    #[from]
    Insertion4(Insertion4),

    #[from]
    Insertion5 { insertion: Insertion5 },
}

#[derive(Debug, PartialEq)]
struct Insertion;

#[derive(Debug, PartialEq)]
struct Insertion2;

#[derive(Debug, PartialEq)]
struct Insertion3;

#[derive(Debug, PartialEq)]
struct Insertion4;

#[derive(Debug, PartialEq)]
struct Insertion5;

assert_eq!(Test::Insertion(Insertion {}), Test::from(Insertion {}));
assert_eq!(Test::Insertion2(Insertion2 {}), Test::from(Insertion2 {}));
assert_eq!(Test::Insertion3 { insertion: Insertion3 {} }, Test::from(Insertion3 {}));
assert_eq!(Test::Insertion4(Insertion4 {}), Test::from(Insertion4 {}));
assert_eq!(Test::Insertion5 { insertion: Insertion5 {} }, Test::from(Insertion5 {}));
```

## Licensing:

Distributed under the MIT license.


## Feedback:

You can contact me via GitHub or send a message to my Telegram [@fuderis](https://t.me/fuderis).

This library is constantly evolving, and I welcome your suggestions and feedback.
