#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

pub use macron_impl_error::Error;
pub use macron_impl_display::Display;
pub use macron_impl_from::From;
pub use macron_impl_into::Into;

pub use macron_map::map;
pub mod collections;

pub use macron_str::str;
pub mod string;
