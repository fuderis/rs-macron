#![allow(dead_code)]

extern crate macron;  use macron::From;

#[test]
fn test_impl_from_struct() {
    #[derive(From, Debug, PartialEq)]
    #[from(Insertion, "Self { insertion: value }")]
    struct Test {
        insertion: Insertion,
    }

    #[derive(Debug, PartialEq)]
    struct Insertion;

    assert_eq!(Test { insertion: Insertion {} }, Test::from(Insertion {}));
}

#[test]
fn test_impl_from_enum() {
    #[derive(From, Debug, PartialEq)]
    #[from(Insertion, "Self::Insertion(value)")]
    enum Test {
        Insertion(Insertion),

        #[from(Insertion2, "value")]
        Insertion2(Insertion2),

        #[from(Insertion3, "insertion: value")]
        Insertion3 { insertion: Insertion3 },

        #[from]
        Insertion4(Insertion4),

        #[from]
        Insertion5 { insertion: Insertion5 },
    }

    #[derive(Debug, PartialEq)]
    struct Insertion;

    #[derive(Debug, PartialEq)]
    struct Insertion2;

    #[derive(Debug, PartialEq)]
    struct Insertion3;

    #[derive(Debug, PartialEq)]
    struct Insertion4;

    #[derive(Debug, PartialEq)]
    struct Insertion5;

    assert_eq!(Test::Insertion(Insertion {}), Test::from(Insertion {}));
    assert_eq!(Test::Insertion2(Insertion2 {}), Test::from(Insertion2 {}));
    assert_eq!(Test::Insertion3 { insertion: Insertion3 {} }, Test::from(Insertion3 {}));
    assert_eq!(Test::Insertion4(Insertion4 {}), Test::from(Insertion4 {}));
    assert_eq!(Test::Insertion5 { insertion: Insertion5 {} }, Test::from(Insertion5 {}));
}
