#![allow(clippy::panic)] // This file only contains tests, so panics are expected.

use crate::*;

#[test]
pub fn usize_ok() {
    let n = 125usize;
    let ok_t = n.ok();
    let ok_w = Ok(n);

    assert_eq!(ok_t, ok_w, "Results should be equal.");
    assert_ne!(ok_t, Err(()), "Result shouldn't be an Err.");
}

#[test]
pub fn usize_err() {
    let n = 125usize;
    let ok_t = n.err();
    let ok_w = Err(n);

    assert_eq!(ok_t, ok_w, "Results should be equal.");
    assert_ne!(ok_t, Ok(()), "Result shouldn't be an Ok.");
}

#[test]
pub fn usize_some() {
    let n = 125usize;
    let ok_t = n.some();
    let ok_w = Some(n);

    assert_eq!(ok_t, ok_w, "Options should be equal.");
    assert_ne!(ok_t, None, "Option shouldn't be None.");
}

#[test]
pub fn result_conv() {
    let value = "foobar";
    let ok_value = value.ok();
    let some_value = value.some();
    let option_value = ok_value.into_option();
    let result_value = some_value.into_result();
    let result_option_value = option_value.into_result();
    let option_result_value: Option<String> = result_option_value.into_option();

    assert_ne!(ok_value, Err(()), "b should be Ok");
    assert_eq!(Some(value), option_value, "values should match");
    assert_eq!(some_value, option_value, "values should match");
    assert_ne!(result_value, Err(()), "e should be Ok");
    assert_eq!(Ok(value), result_value, "values should match");
    assert_eq!(result_value, result_option_value, "values should match");
    assert_eq!(
        option_result_value,
        value.to_owned().some(),
        "values should match"
    );
}
