#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[cfg(feature = "string")]
pub use macron_str::str;
#[cfg(feature = "string")]
pub use macron_regex::re;

#[cfg(feature = "input")]
pub use macron_input::input;
#[cfg(feature = "input")]
pub use macron_inputln::inputln;

#[cfg(feature = "collections")]
pub use macron_collections::{ map, binary_heap, btree_map, btree_set, hash_map, hash_set, linked_list, vec_deque };

#[cfg(feature = "derive")]
pub use macron_impl_error::Error;
#[cfg(feature = "derive")]
pub use macron_impl_display::Display;
#[cfg(feature = "derive")]
pub use macron_impl_from::From;
#[cfg(feature = "derive")]
pub use macron_impl_into::Into;
