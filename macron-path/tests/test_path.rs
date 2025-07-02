extern crate macron_path;  use macron_path::path;

#[test]
fn test_empty_path() {
    path!();
}

#[test]
fn test_path() {
    path!("foo/bar");
}

#[test]
fn test_root_path() {
    path!("/foo/bar");
}

#[test]
fn test_drive_path() {
    path!("C:/foo/bar");
}

#[test]
fn test_format_path() {
    path!("foo/{}", "bar");
}
