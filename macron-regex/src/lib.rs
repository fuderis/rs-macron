#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

/// Creates a new instance of [Regex](https://docs.rs/regex/latest/regex/struct.Regex.html)
#[macro_export]
macro_rules! re {
    ($($tokens:tt)*) => {{
        ::regex::Regex::new(&::std::format!($($tokens)*)).unwrap()
    }};

    ($expr:expr) => {{
        ::regex::Regex::new(&::std::format!($expr)).unwrap()
    }};
}
