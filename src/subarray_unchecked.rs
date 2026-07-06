/// Get a subarray of length `S` at `index` without bounds checking.
///
/// This is the unchecked variant of [`crate::subarray`]. See that function for a safe alternative.
///
/// # Safety
///
/// The caller must ensure that `index + S <= slice.len()`. Violating this is undefined behavior.
///
/// In debug builds, this precondition is checked with a `debug_assert!`.
///
/// # Examples
///
/// ```
/// use ps_util::subarray_unchecked;
/// let data = [1, 2, 3, 4, 5];
/// // SAFETY: index=1, S=2, slice.len()=5, so 1+2 <= 5
/// let chunk: &[i32; 2] = unsafe { subarray_unchecked::<2, i32>(&data, 1) };
/// assert_eq!(chunk, &[2, 3]);
/// ```
///
/// Works with vectors:
///
/// ```
/// use ps_util::subarray_unchecked;
/// let vec = vec!["a", "b", "c", "d"];
/// // SAFETY: index=0, S=3, vec.len()=4, so 0+3 <= 4
/// let chunk: &[&str; 3] = unsafe { subarray_unchecked::<3, &str>(&vec, 0) };
/// assert_eq!(chunk, &["a", "b", "c"]);
/// ```
pub const unsafe fn subarray_unchecked<const S: usize, T>(slice: &[T], index: usize) -> &[T; S] {
    debug_assert!(S <= slice.len() && index <= slice.len() - S);

    &*slice.as_ptr().add(index).cast::<[T; S]>()
}
