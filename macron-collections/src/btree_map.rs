/// Creates a collection [BTreeMap](std::collections::BTreeMap)
#[macro_export]
macro_rules! btree_map {
    ($($tt:tt)*) => {{
        ::std::collections::BTreeMap::from( ::macron_map::map!( $($tt)* ) )
    }};
}

/// Creates a collection [BTreeMap](std::collections::BTreeMap) with auto type converting by [Into::into()](std::convert::Into)
#[macro_export]
macro_rules! auto_btree_map {
    ($($tt:tt)*) => {{
        let fields = ::macron_map::map!($($tt)*);
        let map: ::std::collections::BTreeMap<String, _> = fields
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        map
    }};
}

/// Creates a stringify collection [BTreeMap](std::collections::BTreeMap)
#[macro_export]
macro_rules! str_btree_map {
    ($($tt:tt)*) => {{
        let fields = ::macron_map::map!($($tt)*);
        let map: ::std::collections::BTreeMap<String, _> = fields
            .into_iter()
            .map(|(k, v)| (String::from(k), String::from(v)))
            .collect();
        map
    }};
}
