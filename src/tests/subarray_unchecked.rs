use crate::subarray_unchecked;

// ============ Basic Functionality ============
#[test]
fn test_unchecked_extract_subarray_integers() {
    let data = [1, 2, 3, 4, 5];
    let result: &[i32; 3] = unsafe { subarray_unchecked::<3, i32>(&data, 0) };
    assert_eq!(result, &[1, 2, 3]);
}

#[test]
fn test_unchecked_extract_subarray_with_offset() {
    let data = [10, 20, 30, 40, 50];
    let result: &[i32; 2] = unsafe { subarray_unchecked::<2, i32>(&data, 2) };
    assert_eq!(result, &[30, 40]);
}

#[test]
fn test_unchecked_extract_single_element() {
    let data = [42];
    let result: &[i32; 1] = unsafe { subarray_unchecked::<1, i32>(&data, 0) };
    assert_eq!(result, &[42]);
}

#[test]
fn test_unchecked_extract_entire_slice() {
    let data = [1, 2, 3];
    let result: &[i32; 3] = unsafe { subarray_unchecked::<3, i32>(&data, 0) };
    assert_eq!(result, &[1, 2, 3]);
}

// ============ Different Types ============

#[test]
fn test_unchecked_with_strings() {
    let data = ["a", "b", "c", "d"];
    let result: &[&str; 2] = unsafe { subarray_unchecked::<2, &str>(&data, 1) };
    assert_eq!(result, &["b", "c"]);
}

#[test]
fn test_unchecked_with_floats() {
    let data = [1.1, 2.2, 3.3, 4.4];
    let result: &[f64; 2] = unsafe { subarray_unchecked::<2, f64>(&data, 1) };
    assert!((result[0] - 2.2).abs() < f64::EPSILON);
    assert!((result[1] - 3.3).abs() < f64::EPSILON);
}

#[test]
fn test_unchecked_with_custom_struct() {
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    let data = [
        Point { x: 0, y: 0 },
        Point { x: 1, y: 1 },
        Point { x: 2, y: 2 },
    ];
    let result: &[Point; 2] = unsafe { subarray_unchecked::<2, Point>(&data, 0) };
    assert_eq!(result, &[Point { x: 0, y: 0 }, Point { x: 1, y: 1 }]);
}

// ============ Edge Cases ============

#[test]
fn test_unchecked_end_of_slice() {
    let data = [1, 2, 3, 4, 5];
    let result: &[i32; 2] = unsafe { subarray_unchecked::<2, i32>(&data, 3) };
    assert_eq!(result, &[4, 5]);
}

#[test]
fn test_unchecked_large_size() {
    let data: Vec<i32> = (0..100).collect();
    let result: &[i32; 50] = unsafe { subarray_unchecked::<50, i32>(&data, 25) };
    let expected: Vec<i32> = (25..75).collect();
    assert_eq!(result.to_vec(), expected);
}

#[test]
fn test_unchecked_zero_index() {
    let data = [5, 10, 15];
    let result: &[i32; 2] = unsafe { subarray_unchecked::<2, i32>(&data, 0) };
    assert_eq!(result, &[5, 10]);
}

// ============ Memory Aliasing ============

#[test]
fn test_unchecked_shares_memory() {
    let mut data = [1, 2, 3, 4, 5];
    let subarray_ref: &[i32; 3] = unsafe { subarray_unchecked::<3, i32>(&data, 1) };
    assert_eq!(subarray_ref, &[2, 3, 4]);

    data[2] = 99;
    let subarray_ref2: &[i32; 3] = unsafe { subarray_unchecked::<3, i32>(&data, 1) };
    assert_eq!(subarray_ref2[1], 99);
}

// ============ Array Reference Properties ============

#[test]
fn test_unchecked_result_is_proper_array_reference() {
    let data = [1, 2, 3, 4];
    let result: &[i32; 2] = unsafe { subarray_unchecked::<2, i32>(&data, 1) };

    assert_eq!(result.len(), 2);
    assert_eq!(result.first(), Some(&2));
    assert_eq!(result.last(), Some(&3));
}

#[test]
fn test_unchecked_indexing_into_subarray() {
    let data = [10, 20, 30, 40];
    let result: &[i32; 3] = unsafe { subarray_unchecked::<3, i32>(&data, 0) };

    assert_eq!(result[0], 10);
    assert_eq!(result[1], 20);
    assert_eq!(result[2], 30);
}
