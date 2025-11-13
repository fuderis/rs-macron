#![allow(dead_code)]

extern crate macron_impl_display;  use macron_impl_display::Display;

#[test]
fn display_struct() {
    // without message
    #[derive(Display)]
    struct Test;

    assert_eq!(format!("{}", Test {}), "Test");

    // with message:
    #[derive(Display)]
    #[display = "Hello, {name}!"]
    struct Hello {
        pub name: &'static str,
    }

    let hello = Hello { name: "World" };

    assert_eq!(format!("{hello}"), "Hello, World!");
}

#[test]
fn display_enum() {
    #[derive(Display)]
    enum Animals {
        Test,
        
        Tiger(u8),
        
        #[display = "it's a dog"]
        Dog,
        
        #[display = "cat '{0}'"]
        Cat(&'static str, u8),

        #[display = "bird '{name}'"]
        Bird { name: &'static str, age: u8 },
    }

    assert_eq!(format!("{}", Animals::Test), "Test");

    assert_eq!(format!("{}", Animals::Tiger(2)), "2");

    let dog = Animals::Dog;
    assert_eq!(format!("{dog}"), "it's a dog");

    let cat = Animals::Cat("Tomas", 1);
    assert_eq!(format!("{cat}"), "cat 'Tomas'");

    let bird = Animals::Bird { name: "Kesha", age: 1 };
    assert_eq!(format!("{bird}"), "bird 'Kesha'");
}
