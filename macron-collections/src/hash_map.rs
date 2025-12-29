/// Creates a collection [HashMap](std::collections::HashMap)
#[macro_export]
macro_rules! hash_map {
    () => { ::std::collections::HashMap::new() };

    ($($tokens:tt)*) => {{
        #[cfg(not(feature = "from_macron"))]
        use ::macron_collections::map;
        #[cfg(feature = "from_macron")]
        use ::macron::map;
        
        ::std::collections::HashMap::from( map!($($tokens)*) )
    }};
}
