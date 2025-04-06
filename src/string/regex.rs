/// Creates a new instance of [Regex](https://docs.rs/regex/latest/regex/struct.Regex.html)
#[macro_export]
macro_rules! re {
    ($($tokens:tt)*) => {{
        ::regex::Regex::new(&::macron::str!($($tokens)*)).unwrap()
    }};

    ($expr:expr) => {{
        ::regex::Regex::new(&::macron::str!($expr)).unwrap()
    }};
}
