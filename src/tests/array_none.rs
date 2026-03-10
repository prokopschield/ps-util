use crate::Array;

#[test]
fn none_returns_true_when_no_items_match() {
    let arr = [1, 2, 3];
    assert!(arr.none(|x| x > &10));
}

#[test]
fn none_returns_false_when_any_item_matches() {
    let arr = [1, 2, 3];
    assert!(!arr.none(|x| x > &2));
}

#[test]
fn none_returns_true_for_empty_array() {
    let arr: [i32; 0] = [];
    assert!(arr.none(|_| true));
}
