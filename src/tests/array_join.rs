use crate::Array;

#[test]
fn joins_integers_with_separator() {
    assert_eq!([1, 2, 3].join(", "), "1, 2, 3");
}

#[test]
fn joins_strings_with_separator() {
    assert_eq!(["a", "b", "c"].join(" - "), "a - b - c");
}

#[test]
fn single_element() {
    assert_eq!([42].join(", "), "42");
}

#[test]
fn empty_array() {
    let arr: [i32; 0] = [];
    assert_eq!(arr.join(", "), "");
}

#[test]
fn empty_separator() {
    assert_eq!([1, 2, 3].join(""), "123");
}

#[test]
fn multi_char_separator() {
    assert_eq!([1, 2, 3].join(" :: "), "1 :: 2 :: 3");
}

#[test]
fn vec_source() {
    let v = vec![10, 20, 30];
    assert_eq!(v.join(", "), "10, 20, 30");
}

#[test]
#[should_panic(expected = "a Display implementation returned an error unexpectedly")]
fn failing_display_panics() {
    use std::fmt;

    struct Bad;

    impl fmt::Display for Bad {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let _ = [Bad, Bad].join(", ");
}
