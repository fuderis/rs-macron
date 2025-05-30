/// Creates a collection [HashMap](std::collections::HashMap)
#[cfg(not(feature = "from_macron"))]
#[macro_export]
macro_rules! hash_map {
    ($($tokens:tt)*) => {{
        ::std::collections::HashMap::from( ::macron_collections::map!( $($tokens)* ) )
    }};
}
/// Creates a collection [HashMap](std::collections::HashMap)
#[cfg(feature = "from_macron")]
#[macro_export]
macro_rules! hash_map {
    ($($tokens:tt)*) => {{
        ::std::collections::HashMap::from( ::macron::map!( $($tokens)* ) )
    }};
}

/// Creates a collection [HashMap](std::collections::HashMap) with auto type converting by [Into::into()](std::convert::Into)
#[cfg(not(feature = "from_macron"))]
#[macro_export]
macro_rules! auto_hash_map {
    ($($tokens:tt)*) => {{
        let fields = ::macron_collections::map!($($tokens)*);
        let map: ::std::collections::HashMap<String, _> = fields
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        map
    }};
}
/// Creates a collection [HashMap](std::collections::HashMap) with auto type converting by [Into::into()](std::convert::Into)
#[cfg(feature = "from_macron")]
#[macro_export]
macro_rules! auto_hash_map {
    ($($tokens:tt)*) => {{
        let fields = ::macron::map!($($tokens)*);
        let map: ::std::collections::HashMap<String, _> = fields
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        map
    }};
}

/// Creates a stringify collection [HashMap](std::collections::HashMap)
#[cfg(not(feature = "from_macron"))]
#[macro_export]
macro_rules! str_hash_map {
    ($($tokens:tt)*) => {{
        let fields = ::macron_collections::map!($($tokens)*);
        let map: ::std::collections::HashMap<String, _> = fields
            .into_iter()
            .map(|(k, v)| (String::from(k), String::from(v)))
            .collect();
        map
    }};
}
/// Creates a stringify collection [HashMap](std::collections::HashMap)
#[cfg(feature = "from_macron")]
#[macro_export]
macro_rules! str_hash_map {
    ($($tokens:tt)*) => {{
        let fields = ::macron::map!($($tokens)*);
        let map: ::std::collections::HashMap<String, _> = fields
            .into_iter()
            .map(|(k, v)| (String::from(k), String::from(v)))
            .collect();
        map
    }};
}
