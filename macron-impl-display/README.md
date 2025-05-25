[![github]](https://github.com/fuderis/rs-macron/tree/main/macron-impl-display)&ensp;
[![crates-io]](https://crates.io/crates/macron-impl-display)&ensp;
[![docs-rs]](https://docs.rs/macron-impl-display)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Impl Display Macro

## Introduction:

The implementation of trait [Display](std::fmt::Display)


## Examples:

```rust
#[derive(Display)]
#[display = "Hello, {name}!"]
struct Hello {
    pub name: &'static str,
}

let hello = Hello { name: "World" };

assert_eq!(format!("{hello}"), "Hello, World!");
```
```rust
#[derive(Display)]
enum Animals {
    #[display = "it's a dog"]
    Dog,
    
    #[display = "cat '{0}'"]
    Cat(&'static str, u8),

    #[display = "bird '{name}'"]
    Bird { name: &'static str, age: u8 },
}

let dog = Animals::Dog;
assert_eq!(format!("{dog}"), "it's a dog");

let cat = Animals::Cat("Tomas", 1);
assert_eq!(format!("{cat}"), "cat 'Tomas'");

let bird = Animals::Bird { name: "Kesha", age: 1 };
assert_eq!(format!("{bird}"), "bird 'Kesha'");
```

## Licensing:

Distributed under the MIT license.


## Feedback:

You can contact me via GitHub or send a message to my Telegram [@fuderis](https://t.me/fuderis).

This library is constantly evolving, and I welcome your suggestions and feedback.
