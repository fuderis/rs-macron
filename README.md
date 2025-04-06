[![github]](https://github.com/fuderis/rs-macron)&ensp;
[![crates-io]](https://crates.io/crates/macron)&ensp;
[![docs-rs]](https://docs.rs/macron)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Introduction

🔧 Comprehensive Rust macros toolkit for everyday development tasks. Includes convenient string formatting (str!), regex pattern matching (re!), streamlined collection creation, and custom derive macros for Display, Error, From, and Into traits. Boost your productivity with intuitive syntax and simplify your codebase while maintaining performance and safety. Perfect for both small projects and large-scale applications.

🚀 Empower your Rust development with this versatile set of macros, designed to make common tasks easier and more enjoyable.

# Examples

## String operations:

**String**
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

**Regex** (writed for crate [regex](https://docs.rs/regex/))
```rust
let re = re!(r"^Hello, \w+!$");

assert!(re.is_match("Hello, World!"));

// with formatting:
let re = re!(r"^Hello, {}!$", "World");

assert!(re.is_match("Hello, World!"));

```

## Collections:

**VecDeque**
```rust
let mut deque = vec_deque![1, 2, 3];

assert_eq!(deque.len(), 3);
assert_eq!(deque.pop_front(), Some(1));
assert_eq!(deque.pop_back(), Some(3));
```

**HashMap**
```rust
let key = "one";
let val = 1;

let map = hash_map! {
    key => val,
    "two": 2,
    "three" => 3,
    "four": 4
};

assert_eq!(map.get("one"), Some(&1));
assert_eq!(map.get("two"), Some(&2));
```

**HashSet**
```rust
let set = hash_set![1, 2, 3];

assert!(set.contains(&1));
assert!(set.contains(&2));
```

**BTreeMap**
```rust
let key = "one";
let val = 1;

let map = btree_map! {
    key => val,
    "two": 2,
    "three" => 3,
    "four": 4
};

assert_eq!(map.get("one"), Some(&1));
assert_eq!(map.get("two"), Some(&2));
```

**BTreeSet**
```rust
let set = btree_set![4, 5, 6];

assert!(set.contains(&4));
assert!(set.contains(&5));
```

**BinaryHeap**
```rust
let mut heap = binary_heap![3, 1, 2];

assert_eq!(heap.pop(), Some(3));
assert_eq!(heap.pop(), Some(2));
assert_eq!(heap.pop(), Some(1));
```

**LinkedList**
```rust
let mut list = linked_list![10, 20, 30];

assert_eq!(list.len(), 3);
assert_eq!(list.pop_front(), Some(10));
assert_eq!(list.pop_back(), Some(30));
```

## Derive macros:

**impl Display**
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

**impl Error**
```rust
#[derive(Debug, Error)]
struct Error {
    source: Option<Box<dyn std::error::Error>>
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "it's my custom error")
    }
}

let err = Error { source: None };

assert_eq!(format!("{err}"), "it's my custom error");
```
```rust
#[derive(Debug, Error)]
enum Error {
    WithAnySource { source: Option<Box<dyn std::error::Error>>, msg: String },

    WithCertainSource(#[source] SourceError),

    WithoutSource
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match &self {
            Self::WithAnySource { source, msg } => write!(f, "the error with a message: '{msg}' and a source: '{src}'", src = if let Some(src) = source.as_deref() { src.to_string() }else { "no source".to_owned() }),
            Self::WithCertainSource(src) => write!(f, "the error with the source: '{src}'"),
            Self::WithoutSource => write!(f, "the error without a source"),
        }
    }
}

#[derive(Debug, Error)]
enum SourceError {
    Error
}

impl ::std::fmt::Display for SourceError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match &self {
            Self::Error => write!(f, "the source of error"),
        }
    }
}

let err = Error::WithAnySource { source: Some(Box::new(SourceError::Error)), msg: "the message of error".to_owned() };
let err2 = Error::WithCertainSource( SourceError::Error );
let err3 = Error::WithoutSource;

assert_eq!(format!("{err}"), "the error with a message: 'the message of error' and a source: 'the source of error'");
assert_eq!(format!("{err2}"), "the error with the source: 'the source of error'");
assert_eq!(format!("{err3}"), "the error without a source");
```

**impl From**
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

**impl Into**
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

**Map**
```rust
let (k, v) = ("one", 1);
    
let map = map! {
    k => v,
    "two": 2,
    "three" => 3,
    "four": 4,
};

assert_eq!(map, [("one", 1), ("two", 2), ("three", 3), ("four", 4)])
```

# Licensing
Distributed under the MIT license.

# Feedback
You can contact me via GitHub or send a message to my Telegram [@fuderis](https://t.me/fuderis).

This library is constantly evolving, and I welcome your suggestions and feedback.
