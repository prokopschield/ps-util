/// Returns a reference to a contiguous subarray of length `S` starting at `index`.
///
/// Returns `None` if the requested range exceeds the slice bounds.
/// Returns `None` if `index + S` overflows.
///
/// This is the bounds-checked variant. For an unchecked version, see `subarray_unchecked`.
///
/// # Examples
///
/// ```
/// use ps_util::subarray_checked;
/// let data = [1, 2, 3, 4, 5];
/// let chunk: Option<&[i32; 2]> = subarray_checked::<2, i32>(&data, 1);
/// assert_eq!(chunk, Some(&[2, 3]));
///
/// // Out of bounds returns None
/// assert_eq!(subarray_checked::<2, i32>(&data, 4), None);
/// ```
///
/// Works with vectors:
///
/// ```
/// use ps_util::subarray_checked;
/// let vec = vec!["a", "b", "c", "d"];
/// let chunk: Option<&[&str; 3]> = subarray_checked::<3, &str>(&vec, 0);
/// assert_eq!(chunk, Some(&["a", "b", "c"]));
///
/// // Out of bounds returns None
/// assert_eq!(subarray_checked::<3, &str>(&vec, 2), None);
/// ```
pub fn subarray_checked<const S: usize, T>(slice: &[T], index: usize) -> Option<&[T; S]> {
    let end = index.checked_add(S)?;
    slice.get(index..end)?.as_array()
}
