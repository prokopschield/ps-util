//! Generally helpful utility functions and traits.
//!
//! This crate provides three groups of utilities:
//!
//! - [`Array`]: a trait implemented for every `AsRef<[T]>` type that offers
//!   JavaScript-inspired array methods ([`map`](Array::map),
//!   [`filter`](Array::filter), [`find`](Array::find),
//!   [`reduce`](Array::reduce), [`join`](Array::join), and many more),
//!   along with binary-search-based `*_equal_in_sorted_by` methods
//!   for sorted slices.
//! - [`subarray`], [`subarray_checked`], and [`subarray_unchecked`]:
//!   functions returning a fixed-size array reference `&[T; S]` into a slice.
//! - [`ToResult`] and [`ResConv`]: ergonomic conversions for `Result`
//!   and `Option` values.
//!
//! # Examples
//!
//! ```
//! use ps_util::{Array, ToResult};
//!
//! let arr = [1, 2, 3, 4];
//!
//! assert_eq!(arr.filter(|x| x % 2 == 0), vec![2, 4]);
//! assert_eq!(arr.join(" + "), "1 + 2 + 3 + 4");
//! assert_eq!(arr.find_index_equal_in_sorted_by(|x| x.cmp(&3)), Some(2));
//!
//! assert_eq!(7.ok::<()>(), Ok(7));
//! ```

mod array;
mod conversions;
mod subarray;
mod subarray_checked;
mod subarray_unchecked;

#[cfg(test)]
mod tests;

pub use array::*;
pub use conversions::*;
pub use subarray::*;
pub use subarray_checked::*;
pub use subarray_unchecked::*;
