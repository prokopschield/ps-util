// ============ Basic Functionality ============

use crate::subarray;

#[test]
fn test_extract_subarray_integers() {
    let data = [1, 2, 3, 4, 5];
    let result: &[i32; 3] = subarray::<3, i32>(&data, 0);
    assert_eq!(result, &[1, 2, 3]);
}

#[test]
fn test_extract_subarray_with_offset() {
    let data = [10, 20, 30, 40, 50];
    let result: &[i32; 2] = subarray::<2, i32>(&data, 2);
    assert_eq!(result, &[30, 40]);
}

#[test]
fn test_extract_single_element() {
    let data = [42];
    let result: &[i32; 1] = subarray::<1, i32>(&data, 0);
    assert_eq!(result, &[42]);
}

#[test]
fn test_extract_entire_slice() {
    let data = [1, 2, 3];
    let result: &[i32; 3] = subarray::<3, i32>(&data, 0);
    assert_eq!(result, &[1, 2, 3]);
}

// ============ Different Types ============

#[test]
fn test_subarray_with_strings() {
    let data = ["a", "b", "c", "d"];
    let result: &[&str; 2] = subarray::<2, &str>(&data, 1);
    assert_eq!(result, &["b", "c"]);
}

#[test]
fn test_subarray_with_floats() {
    let data = [1.1, 2.2, 3.3, 4.4];
    let result: &[f64; 2] = subarray::<2, f64>(&data, 1);
    assert_eq!(result, &[2.2, 3.3]);
}

#[test]
fn test_subarray_with_custom_struct() {
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
    let result: &[Point; 2] = subarray::<2, Point>(&data, 0);
    assert_eq!(result, &[Point { x: 0, y: 0 }, Point { x: 1, y: 1 }]);
}

#[test]
fn test_subarray_with_bools() {
    let data = [true, false, true, false, true];
    let result: &[bool; 3] = subarray::<3, bool>(&data, 1);
    assert_eq!(result, &[false, true, false]);
}

// ============ Edge Cases ============

#[test]
fn test_subarray_end_of_slice() {
    let data = [1, 2, 3, 4, 5];
    let result: &[i32; 2] = subarray::<2, i32>(&data, 3);
    assert_eq!(result, &[4, 5]);
}

#[test]
fn test_subarray_large_size() {
    let data: Vec<i32> = (0..100).collect();
    let result: &[i32; 50] = subarray::<50, i32>(&data, 25);
    let expected: Vec<i32> = (25..75).collect();
    assert_eq!(result.to_vec(), expected);
}

#[test]
fn test_subarray_with_zero_index() {
    let data = [5, 10, 15];
    let result: &[i32; 2] = subarray::<2, i32>(&data, 0);
    assert_eq!(result, &[5, 10]);
}

// ============ Mutation Through Original Slice ============

#[test]
fn test_subarray_shares_memory() {
    let mut data = [1, 2, 3, 4, 5];
    let subarray_ref: &[i32; 3] = subarray::<3, i32>(&data, 1);
    assert_eq!(subarray_ref, &[2, 3, 4]);

    data[2] = 99;
    // Verify the subarray reference still points to modified data
    let subarray_ref2: &[i32; 3] = subarray::<3, i32>(&data, 1);
    assert_eq!(subarray_ref2[1], 99);
}

// ============ Panic Cases ============

#[test]
#[should_panic]
fn test_panic_index_out_of_bounds() {
    let data = [1, 2, 3];
    let _result: &[i32; 2] = subarray::<2, i32>(&data, 3);
}

#[test]
#[should_panic]
fn test_panic_insufficient_elements() {
    let data = [1, 2];
    let _result: &[i32; 5] = subarray::<5, i32>(&data, 0);
}

#[test]
#[should_panic]
fn test_panic_zero_slice_length() {
    let data: [i32; 0] = [];
    let _result: &[i32; 1] = subarray::<1, i32>(&data, 0);
}

#[test]
#[should_panic]
fn test_panic_nonzero_index_short_slice() {
    let data = [1];
    let _result: &[i32; 1] = subarray::<1, i32>(&data, 1);
}

// ============ Type System Verification ============

#[test]
fn test_lifetime_validity() {
    let data = vec![1, 2, 3, 4];
    let result: &[i32; 2] = subarray::<2, i32>(&data, 1);
    assert_eq!(result, &[2, 3]);
}

#[test]
fn test_const_generic_parameter() {
    // Verify const generic parameter is correctly enforced
    let data = [1, 2, 3, 4, 5];

    let result2: &[i32; 2] = subarray::<2, i32>(&data, 0);
    let result3: &[i32; 3] = subarray::<3, i32>(&data, 0);
    let result4: &[i32; 4] = subarray::<4, i32>(&data, 1);

    assert_eq!(result2, &[1, 2]);
    assert_eq!(result3, &[1, 2, 3]);
    assert_eq!(result4, &[2, 3, 4, 5]);
}

// ============ Array Reference Properties ============

#[test]
fn test_result_is_proper_array_reference() {
    let data = [1, 2, 3, 4];
    let result: &[i32; 2] = subarray::<2, i32>(&data, 1);

    // Verify we can use array-specific methods
    assert_eq!(result.len(), 2);
    assert_eq!(result.first(), Some(&2));
    assert_eq!(result.last(), Some(&3));
}

#[test]
fn test_indexing_into_subarray() {
    let data = [10, 20, 30, 40];
    let result: &[i32; 3] = subarray::<3, i32>(&data, 0);

    assert_eq!(result[0], 10);
    assert_eq!(result[1], 20);
    assert_eq!(result[2], 30);
}
