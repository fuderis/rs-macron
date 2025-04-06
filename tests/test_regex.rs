extern crate macron;  #[cfg(feature = "test_regex")] use macron::re;

#[cfg(feature = "test_regex")]
#[test]
fn test_regex() {
    let re = re!(r"^Hello, \w+!$");

    assert!(re.is_match("Hello, World!"));
}

#[cfg(feature = "test_regex")]
#[test]
fn test_regex_format() {
    let re = re!(r"^Hello, {}!$", "World");

    assert!(re.is_match("Hello, World!"));
}
