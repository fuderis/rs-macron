#![allow(dead_code)]

extern crate macron;  use macron::Into;

#[test]
fn test_impl_into_struct() {
    #[derive(Into, Debug, PartialEq)]
    #[into(Insertion, "self.insertion")]
    struct Test {
        insertion: Insertion,
    }

    #[derive(Debug, PartialEq)]
    struct Insertion;

    assert_eq!(Insertion {}, Test { insertion: Insertion {} }.into());
}

#[test]
fn test_impl_into_enum() {
    #[derive(Into, Debug, PartialEq)]
    #[into(Insertion, "match self { Self::Insertion(ins) => ins }")]
    enum Test {
        Insertion(Insertion),
    }

    #[derive(Debug, PartialEq)]
    struct Insertion;

    assert_eq!(Insertion {}, Test::Insertion(Insertion {}).into());
}
