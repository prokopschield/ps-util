use std::fmt::Write;

use crate::{subarray, subarray_checked, subarray_unchecked};

pub trait Array<T> {
    /// Returns a reference to the element at the specified index,
    /// or `None` if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3];
    /// assert_eq!(arr.at(0), Some(&1));
    /// assert_eq!(arr.at(5), None);
    /// ```
    fn at(&self, index: usize) -> Option<&T>;

    /// Concatenates this array with another slice and returns a new vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2];
    /// let result = arr.concat(&[3, 4]);
    /// assert_eq!(result, vec![1, 2, 3, 4]);
    /// ```
    fn concat(&self, other: impl AsRef<[T]>) -> Vec<T>
    where
        T: Clone;

    /// Returns an iterator of (index, &T) tuples for each element.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = ['a', 'b'];
    /// let entries: Vec<_> = arr.entries().collect();
    /// assert_eq!(entries, vec![(0, &'a'), (1, &'b')]);
    /// ```
    fn entries<'a>(&'a self) -> impl Iterator<Item = (usize, &'a T)>
    where
        T: 'a;

    /// Tests whether all elements match the predicate.
    ///
    /// Returns `true` if the predicate returns `true` for every element,
    /// or if the array is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [2, 4, 6];
    /// assert!(arr.every(|x| x % 2 == 0));
    /// assert!(!arr.every(|x| x > &5));
    /// ```
    fn every(&self, predicate: impl FnMut(&T) -> bool) -> bool;

    /// Returns a reference to the first element that matches the predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3, 4];
    /// assert_eq!(arr.find(|x| x > &2), Some(&3));
    /// assert_eq!(arr.find(|x| x > &10), None);
    /// ```
    fn find(&self, predicate: impl FnMut(&T) -> bool) -> Option<&T>;

    /// Returns the index of the first element that matches the predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3, 4];
    /// assert_eq!(arr.find_index(|x| x > &2), Some(2));
    /// assert_eq!(arr.find_index(|x| x > &10), None);
    /// ```
    fn find_index(&self, predicate: impl FnMut(&T) -> bool) -> Option<usize>;

    /// Returns a reference to the last element that matches the predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3, 4];
    /// assert_eq!(arr.find_last(|x| x < &4), Some(&3));
    /// assert_eq!(arr.find_last(|x| x > &10), None);
    /// ```
    fn find_last(&self, predicate: impl FnMut(&T) -> bool) -> Option<&T>;

    /// Returns the index of the last element that matches the predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3, 4];
    /// assert_eq!(arr.find_last_index(|x| x < &4), Some(2));
    /// assert_eq!(arr.find_last_index(|x| x > &10), None);
    /// ```
    fn find_last_index(&self, predicate: impl FnMut(&T) -> bool) -> Option<usize>;

    /// Returns a vector containing all elements that match the predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3, 4];
    /// assert_eq!(arr.filter(|x| x % 2 == 0), vec![2, 4]);
    /// ```
    fn filter(&self, predicate: impl FnMut(&T) -> bool) -> Vec<T>
    where
        T: Clone;

    /// Flattens a level of nesting in an array of iterables.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [vec![1, 2], vec![3, 4]];
    /// assert_eq!(arr.flat(), vec![1, 2, 3, 4]);
    /// ```
    fn flat(&self) -> Vec<T::Item>
    where
        T: Clone + IntoIterator;

    /// Maps each element to an iterable and flattens the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3];
    /// let result = arr.flat_map(|x| vec![*x, *x * 2]);
    /// assert_eq!(result, vec![1, 2, 2, 4, 3, 6]);
    /// ```
    fn flat_map<O, I>(&self, mapper: impl FnMut(&T) -> I) -> Vec<O>
    where
        I: IntoIterator<Item = O>;

    /// Applies a closure to each element for side effects.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3];
    /// arr.for_each(|x| println!("{}", x));
    /// ```
    fn for_each(&self, cb: impl FnMut(&T));

    /// Checks whether the array contains the specified value.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3];
    /// assert!(arr.includes(&2));
    /// assert!(!arr.includes(&5));
    /// ```
    fn includes(&self, value: &T) -> bool
    where
        T: PartialEq;

    /// Returns the index of the first occurrence of the specified value,
    /// or `None` if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3, 2];
    /// assert_eq!(arr.index_of(&2), Some(1));
    /// assert_eq!(arr.index_of(&5), None);
    /// ```
    fn index_of(&self, value: &T) -> Option<usize>
    where
        T: PartialEq;

    /// Returns `true` if the array is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// assert!(Vec::<i32>::new().is_empty());
    /// assert!(![1].is_empty());
    /// ```
    fn is_empty(&self) -> bool;

    /// Concatenates all elements into a string, separated by the given separator.
    ///
    /// # Errors
    ///
    /// Errors are passed from the [`std::fmt::Display`] implementation.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3];
    /// assert_eq!(arr.join(", ").unwrap(), "1, 2, 3");
    /// ```
    fn join(&self, separator: &str) -> Result<String, std::fmt::Error>
    where
        T: std::fmt::Display;

    /// Returns an iterator of indices (0, 1, 2, ...).
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = ['a', 'b', 'c'];
    /// let keys: Vec<_> = arr.keys().collect();
    /// assert_eq!(keys, vec![0, 1, 2]);
    /// ```
    fn keys(&self) -> impl Iterator<Item = usize>;

    /// Returns the index of the last occurrence of the specified value,
    /// or `None` if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3, 2];
    /// assert_eq!(arr.last_index_of(&2), Some(3));
    /// assert_eq!(arr.last_index_of(&5), None);
    /// ```
    fn last_index_of(&self, value: &T) -> Option<usize>
    where
        T: PartialEq;

    /// Returns the number of elements in the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3];
    /// assert_eq!(arr.len(), 3);
    /// ```
    fn len(&self) -> usize;

    /// Transforms each element using the provided mapper function
    /// and returns a vector of the results.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3u8];
    /// assert_eq!(arr.as_slice().map(|x| x * 2), vec![2, 4, 6]);
    /// ```
    fn map<O>(&self, mapper: impl FnMut(&T) -> O) -> Vec<O>;

    /// Reduces the array to a single value by applying a callback
    /// with an accumulator, starting from the left.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3, 4];
    /// let sum = arr.reduce(|acc, x| acc + x, 0);
    /// assert_eq!(sum, 10);
    /// ```
    fn reduce<O>(&self, reducer: impl FnMut(O, &T) -> O, initial: O) -> O;

    /// Reduces the array to a single value by applying a callback
    /// with an accumulator, starting from the right.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3];
    /// let result = arr.reduce_right(
    ///     |acc, x| format!("{}{}", acc, x),
    ///     String::new()
    /// );
    /// assert_eq!(result, "321");
    /// ```
    fn reduce_right<O>(&self, reducer: impl FnMut(O, &T) -> O, initial: O) -> O;

    /// Returns a slice of the array from `start` to `end` (exclusive).
    ///
    /// If `end` is `None`, slices to the end of the array. Indices are clamped
    /// to valid bounds; if `start` exceeds the array length, an empty slice
    /// is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3, 4];
    /// assert_eq!(arr.slice(1, Some(3)), &[2, 3][..]);
    /// assert_eq!(arr.slice(2, None), &[3, 4][..]);
    /// assert_eq!(arr.slice(10, Some(20)), &[][..]);
    /// ```
    fn slice(&self, start: usize, end: Option<usize>) -> &[T];

    /// Tests whether any element matches the predicate.
    ///
    /// Returns `true` if the predicate returns `true` for at least one element.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3];
    /// assert!(arr.some(|x| x > &2));
    /// assert!(!arr.some(|x| x > &10));
    /// ```
    fn some(&self, predicate: impl FnMut(&T) -> bool) -> bool;

    /// Returns a fixed-size array reference starting at the given index.
    ///
    /// # Panics
    ///
    /// Panics if there are not enough elements remaining in the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3, 4];
    /// assert_eq!(arr.subarray::<2>(1), &[2, 3]);
    /// ```
    fn subarray<const S: usize>(&self, index: usize) -> &[T; S];

    /// Checked version of `subarray`. Returns `None` if bounds are exceeded.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3];
    /// assert_eq!(arr.subarray_checked::<2>(1), Some(&[2, 3]));
    /// assert_eq!(arr.subarray_checked::<2>(2), None);
    /// ```
    fn subarray_checked<const S: usize>(&self, index: usize) -> Option<&[T; S]>;

    /// Unchecked version of `subarray`. Undefined behavior if bounds are exceeded.
    ///
    /// # Safety
    ///
    /// Caller must ensure that `index + S <= self.len()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3, 4];
    /// unsafe {
    ///     assert_eq!(arr.subarray_unchecked::<2>(1), &[2, 3]);
    /// }
    /// ```
    unsafe fn subarray_unchecked<const S: usize>(&self, index: usize) -> &[T; S];

    /// Returns an iterator over references to the elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3];
    /// let values: Vec<_> = arr.values().collect();
    /// assert_eq!(values, vec![&1, &2, &3]);
    /// ```
    fn values<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a;
}

impl<A, T> Array<T> for A
where
    A: AsRef<[T]>,
{
    fn at(&self, index: usize) -> Option<&T> {
        self.as_ref().get(index)
    }

    fn concat(&self, other: impl AsRef<[T]>) -> Vec<T>
    where
        T: Clone,
    {
        let lhs = self.as_ref();
        let rhs = other.as_ref();

        let mut concatenated = Vec::with_capacity(lhs.len() + rhs.len());

        concatenated.extend_from_slice(lhs);
        concatenated.extend_from_slice(rhs);

        concatenated
    }

    fn entries<'a>(&'a self) -> impl Iterator<Item = (usize, &'a T)>
    where
        T: 'a,
    {
        self.as_ref().iter().enumerate()
    }

    fn every(&self, predicate: impl FnMut(&T) -> bool) -> bool {
        self.as_ref().iter().all(predicate)
    }

    fn filter(&self, mut predicate: impl FnMut(&T) -> bool) -> Vec<T>
    where
        T: Clone,
    {
        self.as_ref()
            .iter()
            .filter(|item| predicate(item))
            .cloned()
            .collect()
    }

    fn find(&self, mut predicate: impl FnMut(&T) -> bool) -> Option<&T> {
        Iterator::find(&mut self.as_ref().iter(), |item| predicate(item))
    }

    fn find_index(&self, predicate: impl FnMut(&T) -> bool) -> Option<usize> {
        self.as_ref().iter().position(predicate)
    }

    fn find_last(&self, mut predicate: impl FnMut(&T) -> bool) -> Option<&T> {
        self.as_ref().iter().rfind(|item| predicate(item))
    }

    fn find_last_index(&self, predicate: impl FnMut(&T) -> bool) -> Option<usize> {
        self.as_ref().iter().rposition(predicate)
    }

    fn flat(&self) -> Vec<<T>::Item>
    where
        T: Clone + IntoIterator,
    {
        self.as_ref().iter().cloned().flatten().collect()
    }

    fn flat_map<O, I>(&self, mapper: impl FnMut(&T) -> I) -> Vec<O>
    where
        I: IntoIterator<Item = O>,
    {
        self.as_ref().iter().flat_map(mapper).collect()
    }

    fn for_each(&self, cb: impl FnMut(&T)) {
        self.as_ref().iter().for_each(cb);
    }

    fn includes(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.as_ref().contains(value)
    }

    fn index_of(&self, value: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        self.as_ref().iter().position(|x| x == value)
    }

    fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    fn join(&self, separator: &str) -> Result<String, std::fmt::Error>
    where
        T: std::fmt::Display,
    {
        let mut a = String::new();
        let mut iterator = self.as_ref().iter();

        if let Some(first) = iterator.next() {
            write!(&mut a, "{first}")?;

            for item in iterator {
                write!(&mut a, "{separator}{item}")?;
            }
        }

        Ok(a)
    }

    fn keys(&self) -> impl Iterator<Item = usize> {
        0..self.as_ref().len()
    }

    fn last_index_of(&self, value: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        self.find_last_index(|item| item == value)
    }

    fn len(&self) -> usize {
        self.as_ref().len()
    }

    fn map<O>(&self, mapper: impl FnMut(&T) -> O) -> Vec<O> {
        self.as_ref().iter().map(mapper).collect()
    }

    fn reduce<O>(&self, reducer: impl FnMut(O, &T) -> O, initial: O) -> O {
        self.as_ref().iter().fold(initial, reducer)
    }

    fn reduce_right<O>(&self, reducer: impl FnMut(O, &T) -> O, initial: O) -> O {
        self.as_ref().iter().rev().fold(initial, reducer)
    }

    fn slice(&self, start: usize, end: Option<usize>) -> &[T] {
        let full = self.as_ref();
        let len = full.len();
        let start = usize::min(start, len);
        let end = end.unwrap_or(len).clamp(start, len);

        &full[start..end]
    }

    fn some(&self, predicate: impl FnMut(&T) -> bool) -> bool {
        self.as_ref().iter().any(predicate)
    }

    fn subarray<const S: usize>(&self, index: usize) -> &[T; S] {
        subarray(self.as_ref(), index)
    }

    fn subarray_checked<const S: usize>(&self, index: usize) -> Option<&[T; S]> {
        subarray_checked(self.as_ref(), index)
    }

    unsafe fn subarray_unchecked<const S: usize>(&self, index: usize) -> &[T; S] {
        subarray_unchecked(self.as_ref(), index)
    }

    fn values<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.as_ref().iter()
    }
}
