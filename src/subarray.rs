use crate::subarray_checked;

/// Get a subarray of length `S` at `index`.
///
/// # Examples
///
/// ```
/// use ps_util::subarray;
/// let data = [1, 2, 3, 4, 5];
/// let chunk: &[i32; 2] = subarray::<2, i32>(&data, 1);
/// assert_eq!(chunk, &[2, 3]);
/// ```
///
/// Works with vectors:
///
/// ```
/// use ps_util::subarray;
/// let vec = vec!["a", "b", "c", "d"];
/// let chunk: &[&str; 3] = subarray::<3, &str>(&vec, 0);
/// assert_eq!(chunk, &["a", "b", "c"]);
/// ```
///
/// # Panics
///
/// Panics if `index + S` exceeds the slice length:
///
/// ```should_panic
/// use ps_util::subarray;
/// let data = [1, 2, 3];
/// let _chunk: &[i32; 3] = subarray::<3, i32>(&data, 1);
/// ```
#[allow(clippy::expect_used)]
pub fn subarray<const S: usize, T>(slice: &[T], index: usize) -> &[T; S] {
    subarray_checked(slice, index).expect("range end index out of range for subarray")
}
