use crate::conversions::result::ResConv;

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
