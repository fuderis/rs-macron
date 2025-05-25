[![github]](https://github.com/fuderis/rs-macron)&ensp;
[![crates-io]](https://crates.io/crates/macron)&ensp;
[![docs-rs]](https://docs.rs/macron)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Macros Toolkit

## Introduction:

Comprehensive Rust macros toolkit for everyday development tasks. Includes convenient string formatting (str!), regex pattern matching (re!), streamlined collection creation, and custom derive macros for Display, Error, From, and Into traits. Boost your productivity with intuitive syntax and simplify your codebase while maintaining performance and safety. Perfect for both small projects and large-scale applications.

Empower your Rust development with this versatile set of macros, designed to make common tasks easier and more enjoyable.


## Navigation:

#### String operations (feature 'string'):

* [macron-str](https://docs.rs/macron-str) - Creates a new instance of [String](https://doc.rust-lang.org/stable/std/string/struct.String.html)
* [macron-regex](https://docs.rs/macron-regex) - Creates a new instance of [Regex](https://docs.rs/regex/latest/regex/struct.Regex.html)

#### IO Input (feature 'input'):

* [macron-input](https://docs.rs/macron-input) - Reads user input lines from the console
* [macron-inputln](https://docs.rs/macron-inputln) - Reads user input line from the console

#### Collections (feature 'collections'):

* [macron-collections](https://docs.rs/macron-collections) - Creates a new instance of std collections: HashMap, HashSet, BTreeMap, BTreeSet, VecDeque, LinkedList and BinaryHeap

#### Derive macros (feature 'derive'):

* [macron-impl-display](https://docs.rs/macron-impl-display) - The implementation of trait [Display](std::fmt::Display)
* [macron-impl-error](https://docs.rs/macron-impl-error) - The implementation of trait [Error](std::error::Error)
* [macron-impl-from](https://docs.rs/macron-impl-from) - The implementation of trait [From](std::convert::From)
* [macron-impl-into](https://docs.rs/macron-impl-into) - The implementation of trait [Into](std::convert::Into)

### Note:
If you need all of this macros, then use feature 'full'.


## Licensing:

Distributed under the MIT license.


## Feedback:

You can contact me via GitHub or send a message to my Telegram [@fuderis](https://t.me/fuderis).

This library is constantly evolving, and I welcome your suggestions and feedback.
