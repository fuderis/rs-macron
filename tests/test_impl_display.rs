#![allow(dead_code)]

extern crate macron;  use macron::Display;

#[test]
fn test_impl_display_struct() {
    // Without message
    #[derive(Display)]
    struct Test;

    assert_eq!(format!("{}", Test {}), "Test");

    // With message:
    #[derive(Display)]
    #[display = "Hello, {name}!"]
    struct Hello {
        pub name: &'static str,
    }

    let hello = Hello { name: "World" };

    assert_eq!(format!("{hello}"), "Hello, World!");
}

#[test]
fn test_impl_display_enum() {
    #[derive(Display)]
    enum Animals {
        Test,
        
        #[display = "it's a dog"]
        Dog,
        
        #[display = "cat '{0}'"]
        Cat(&'static str, u8),

        #[display = "bird '{name}'"]
        Bird { name: &'static str, age: u8 },
    }

    assert_eq!(format!("{}", Animals::Test), "Test");

    let dog = Animals::Dog;
    assert_eq!(format!("{dog}"), "it's a dog");

    let cat = Animals::Cat("Tomas", 1);
    assert_eq!(format!("{cat}"), "cat 'Tomas'");

    let bird = Animals::Bird { name: "Kesha", age: 1 };
    assert_eq!(format!("{bird}"), "bird 'Kesha'");
}
