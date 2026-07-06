# ps-util

Generally helpful utility functions and traits.

## Overview

This crate provides three groups of utilities:

- **`Array`**: a trait implemented for every `AsRef<[T]>` type (slices, arrays, vectors, and more) that offers JavaScript-inspired array methods: `map`, `filter`, `find`, `reduce`, `join`, `flat`, `some`, `every`, and many more. It also includes binary-search-based `*_equal_in_sorted_by` methods for sorted slices, such as `find_equal_in_sorted_by` and `slice_equal_in_sorted_by`, which run in `O(log n)`.
- **`subarray`, `subarray_checked`, `subarray_unchecked`**: functions returning a fixed-size array reference `&[T; S]` into a slice.
- **`ToResult` and `ResConv`**: ergonomic conversions for `Result` and `Option` values.

## Usage

Add the dependency:

```sh
cargo add ps-util
```

Then:

```rust
use ps_util::{Array, ToResult};

let arr = [1, 2, 3, 4];

assert_eq!(arr.filter(|x| x % 2 == 0), vec![2, 4]);
assert_eq!(arr.join(" + "), "1 + 2 + 3 + 4");
assert_eq!(arr.find_index_equal_in_sorted_by(|x| x.cmp(&3)), Some(2));

assert_eq!(7.ok::<()>(), Ok(7));
```

## License

Licensed under the [GNU General Public License, version 3 or later](https://www.gnu.org/licenses/gpl-3.0.html).
