use crate::subarray_checked;

// Basic functionality
#[test]
fn basic_array() {
    let data = [1, 2, 3, 4, 5];
    let chunk: Option<&[i32; 2]> = subarray_checked::<2, i32>(&data, 1);
    assert_eq!(chunk, Some(&[2, 3]));
}

#[test]
fn basic_vector() {
    let vec = vec!["a", "b", "c", "d"];
    let chunk: Option<&[&str; 3]> = subarray_checked::<3, &str>(&vec, 0);
    assert_eq!(chunk, Some(&["a", "b", "c"]));
}

// Boundary conditions
#[test]
fn subarray_at_start() {
    let data = [10, 20, 30, 40, 50];
    let chunk: Option<&[i32; 3]> = subarray_checked::<3, i32>(&data, 0);
    assert_eq!(chunk, Some(&[10, 20, 30]));
}

#[test]
fn subarray_at_end() {
    let data = [10, 20, 30, 40, 50];
    let chunk: Option<&[i32; 2]> = subarray_checked::<2, i32>(&data, 3);
    assert_eq!(chunk, Some(&[40, 50]));
}

#[test]
fn entire_slice_as_subarray() {
    let data = [1, 2, 3];
    let chunk: Option<&[i32; 3]> = subarray_checked::<3, i32>(&data, 0);
    assert_eq!(chunk, Some(&[1, 2, 3]));
}

#[test]
fn single_element_subarray() {
    let data = [42];
    let chunk: Option<&[i32; 1]> = subarray_checked::<1, i32>(&data, 0);
    assert_eq!(chunk, Some(&[42]));
}

// Out of bounds cases
#[test]
fn index_at_length() {
    let data = [1, 2, 3, 4, 5];
    let chunk: Option<&[i32; 2]> = subarray_checked::<2, i32>(&data, 5);
    assert_eq!(chunk, None);
}

#[test]
fn index_far_beyond_length() {
    let data = [1, 2, 3, 4, 5];
    let chunk: Option<&[i32; 1]> = subarray_checked::<1, i32>(&data, 100);
    assert_eq!(chunk, None);
}

#[test]
fn range_exceeds_bounds() {
    let data = [1, 2, 3, 4, 5];
    let chunk: Option<&[i32; 3]> = subarray_checked::<3, i32>(&data, 3);
    assert_eq!(chunk, None);
}

#[test]
fn range_one_element_short() {
    let data = [1, 2, 3, 4, 5];
    let chunk: Option<&[i32; 2]> = subarray_checked::<2, i32>(&data, 4);
    assert_eq!(chunk, None);
}

// Zero-length subarrays
#[test]
fn zero_length_at_valid_index() {
    let data = [1, 2, 3];
    let chunk: Option<&[i32; 0]> = subarray_checked::<0, i32>(&data, 1);
    assert_eq!(chunk, Some(&[]));
}

#[test]
fn zero_length_at_slice_end() {
    let data = [1, 2, 3];
    let chunk: Option<&[i32; 0]> = subarray_checked::<0, i32>(&data, 3);
    assert_eq!(chunk, Some(&[]));
}

#[test]
fn zero_length_beyond_slice() {
    let data = [1, 2, 3];
    let chunk: Option<&[i32; 0]> = subarray_checked::<0, i32>(&data, 4);
    assert_eq!(chunk, None);
}

#[test]
fn empty_slice_zero_length() {
    let data: [i32; 0] = [];
    let chunk: Option<&[i32; 0]> = subarray_checked::<0, i32>(&data, 0);
    assert_eq!(chunk, Some(&[]));
}

#[test]
fn empty_slice_nonzero_length() {
    let data: [i32; 0] = [];
    let chunk: Option<&[i32; 1]> = subarray_checked::<1, i32>(&data, 0);
    assert_eq!(chunk, None);
}

// Different types
#[test]
fn string_slices() {
    let data = vec!["hello", "world", "rust"];
    let chunk: Option<&[&str; 2]> = subarray_checked::<2, &str>(&data, 1);
    assert_eq!(chunk, Some(&["world", "rust"]));
}

#[test]
fn tuple_elements() {
    let data = [(1, 'a'), (2, 'b'), (3, 'c')];
    let chunk: Option<&[(i32, char); 2]> = subarray_checked::<2, (i32, char)>(&data, 0);
    assert_eq!(chunk, Some(&[(1, 'a'), (2, 'b')]));
}

#[test]
fn custom_struct() {
    #[derive(Debug, PartialEq, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    let data = [
        Point { x: 0, y: 0 },
        Point { x: 1, y: 1 },
        Point { x: 2, y: 2 },
    ];
    let chunk: Option<&[Point; 2]> = subarray_checked::<2, Point>(&data, 1);
    assert_eq!(chunk, Some(&[Point { x: 1, y: 1 }, Point { x: 2, y: 2 }]));
}

#[test]
fn floating_point_numbers() {
    let data = vec![1.5, 2.5, 3.5, 4.5];
    let chunk: Option<&[f64; 2]> = subarray_checked::<2, f64>(&data, 1);
    assert_eq!(chunk, Some(&[2.5, 3.5]));
}

// Large data
#[test]
fn large_vector_middle() {
    let data: Vec<i32> = (0..1000).collect();
    let chunk: Option<&[i32; 10]> = subarray_checked::<10, i32>(&data, 500);
    let chunk = chunk.expect("expected a 10-element subarray at index 500");
    assert_eq!(chunk[0], 500);
    assert_eq!(chunk[9], 509);
}

#[test]
fn large_vector_exceeds_bounds() {
    let data: Vec<i32> = (0..1000).collect();
    let chunk: Option<&[i32; 10]> = subarray_checked::<10, i32>(&data, 995);
    assert_eq!(chunk, None);
}
