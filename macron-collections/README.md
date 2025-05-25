[![github]](https://github.com/fuderis/rs-macron/tree/main/macron-collections)&ensp;
[![crates-io]](https://crates.io/crates/macron-collections)&ensp;
[![docs-rs]](https://docs.rs/macron-collections)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Std::Collections Declarative Macros

## Introduction:

Creates a new instance of std collections: HashMap, HashSet, BTreeMap, BTreeSet, VecDeque, LinkedList and BinaryHeap


## Examples:

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

## Licensing:

Distributed under the MIT license.


## Feedback:

You can contact me via GitHub or send a message to my Telegram [@fuderis](https://t.me/fuderis).

This library is constantly evolving, and I welcome your suggestions and feedback.
