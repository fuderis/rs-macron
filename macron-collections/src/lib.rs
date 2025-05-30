#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[cfg(feature = "test")]
pub use macron_map;

pub mod vec_deque;

pub mod hash_map;
pub mod hash_set;

pub mod btree_map;
pub mod btree_set;

pub mod binary_heap;

pub mod linked_list;
