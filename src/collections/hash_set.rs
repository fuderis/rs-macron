/// Creates a collection [HashSet](std::collections::HashSet)
#[macro_export]
macro_rules! hash_set {
    ($($v:expr),* $(,)?) => {{
        let mut set = ::std::collections::HashSet::new();
        $(set.insert($v);)*
        set
    }};
}
