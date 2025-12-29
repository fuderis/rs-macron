/// Creates a collection [BTreeMap](std::collections::BTreeMap)
#[macro_export]
macro_rules! btree_map {
    () => { ::std::collections::BTreeMap::new() };

    ($($tt:tt)*) => {{
        #[cfg(not(feature = "from_macron"))]
        use ::macron_collections::map;
        #[cfg(feature = "from_macron")]
        use ::macron::map;
        
        ::std::collections::BTreeMap::from( map!( $($tt)* ) )
    }};
}
