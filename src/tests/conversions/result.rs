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
    let a = "foobar";
    let b = a.ok();
    let c = a.some();
    let d = b.into_option();
    let e = c.into_result();
    let f = d.into_result();
    let o: Option<String> = f.into_option();

    assert_ne!(b, Err(()), "b should be Ok");
    assert_eq!(Some(a), d, "values should match");
    assert_eq!(c, d, "values should match");
    assert_ne!(e, Err(()), "e should be Ok");
    assert_eq!(Ok(a), e, "values should match");
    assert_eq!(e, f, "values should match");
    assert_eq!(o, a.to_owned().some(), "values should match");
}
