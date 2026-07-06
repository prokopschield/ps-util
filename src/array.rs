use std::{cmp::Ordering, fmt::Write};

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

    /// Tests whether all elements are equal to the comparator target.
    ///
    /// Returns `true` if the array is empty or every element compares as
    /// [`Ordering::Equal`].
    ///
    /// **Only the first and last elements are checked.**
    ///
    /// The slice must be sorted according to the same ordering used by
    /// `comparator`. **If it is not, the result is undefined.** See
    /// [`Array::find_equal_in_sorted_by`] for details on the comparator
    /// direction pitfall.
    ///
    /// Time complexity: `O(1)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    ///
    /// let arr = [2, 2, 2];
    /// assert!(arr.every_equal_in_sorted_by(|x| x.cmp(&2)));
    ///
    /// let arr = [1, 2, 2];
    /// assert!(!arr.every_equal_in_sorted_by(|x| x.cmp(&2)));
    /// ```
    fn every_equal_in_sorted_by(&self, comparator: impl FnMut(&T) -> Ordering) -> bool;

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

    /// Returns the first element equal to the comparator target.
    ///
    /// The slice must be sorted according to the same ordering used by
    /// `comparator`. **If the slice is not sorted according to the
    /// comparator, the result is undefined**: the method will not panic,
    /// but may return `None` even when a match exists. This includes the
    /// case where the comparator is written in the wrong direction
    /// relative to the actual sort order.
    ///
    /// Time complexity: `O(log n)`.
    ///
    /// # Comparator direction pitfall
    ///
    /// The comparator must produce an [`Ordering`] that is consistent with
    /// how the slice is sorted. Getting the direction wrong violates this
    /// precondition, and is particularly insidious because
    /// **the code compiles, runs without
    /// panicking, and silently returns incorrect results.**
    ///
    /// This happens because [`Ordering::Equal`] is symmetric:
    /// `a.cmp(&b) == Equal` ⟺ `b.cmp(&a) == Equal`.
    ///
    /// In practice:
    /// - **Ascending** slice => `|item| item.cmp(&target)`
    /// - **Descending** slice => `|item| target.cmp(item)`
    ///
    /// With simple integers, the mistake is easy to spot;
    /// with structs or compound keys, it becomes much harder to notice:
    ///
    /// ```
    /// use ps_util::Array;
    /// use std::cmp::Ordering;
    ///
    /// struct Event { timestamp: u64, payload: String }
    ///
    /// let log: Vec<Event> = vec![
    ///     Event { timestamp: 100, payload: "a".into() },
    ///     Event { timestamp: 200, payload: "b".into() },
    ///     Event { timestamp: 300, payload: "c".into() },
    /// ];
    ///
    /// let target_ts: u64 = 200;
    ///
    /// // CORRECT: comparator matches the ascending sort order:
    /// let found = log.find_equal_in_sorted_by(|e| e.timestamp.cmp(&target_ts));
    /// assert_eq!(found.map(|e| &*e.payload), Some("b"));
    ///
    /// // WRONG: comparator is reversed; result is undefined:
    /// let found = log.find_equal_in_sorted_by(|e| target_ts.cmp(&e.timestamp));
    /// assert!(found.is_none()); // silently misses the match
    /// ```
    ///
    /// This pitfall applies equally to all `*_equal_in_sorted_by` methods.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 2, 2, 3];
    /// assert_eq!(arr.find_equal_in_sorted_by(|x| x.cmp(&2)), Some(&2));
    /// assert_eq!(arr.find_equal_in_sorted_by(|x| x.cmp(&5)), None);
    /// ```
    fn find_equal_in_sorted_by(&self, comparator: impl FnMut(&T) -> Ordering) -> Option<&T>;

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

    /// Returns the index of the first element equal to the comparator target.
    ///
    /// The slice must be sorted according to the same ordering used by
    /// `comparator`. **If it is not, the result is undefined.** See
    /// [`Array::find_equal_in_sorted_by`] for details on the comparator
    /// direction pitfall.
    ///
    /// Time complexity: `O(log n)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 2, 2, 3];
    /// assert_eq!(arr.find_index_equal_in_sorted_by(|x| x.cmp(&2)), Some(1));
    /// assert_eq!(arr.find_index_equal_in_sorted_by(|x| x.cmp(&5)), None);
    /// ```
    fn find_index_equal_in_sorted_by(
        &self,
        comparator: impl FnMut(&T) -> Ordering,
    ) -> Option<usize>;

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

    /// Returns the last element equal to the comparator target.
    ///
    /// The slice must be sorted according to the same ordering used by
    /// `comparator`. **If it is not, the result is undefined.** See
    /// [`Array::find_equal_in_sorted_by`] for details on the comparator
    /// direction pitfall.
    ///
    /// Time complexity: `O(log n)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 2, 2, 3];
    /// assert_eq!(arr.find_last_equal_in_sorted_by(|x| x.cmp(&2)), Some(&2));
    /// assert_eq!(arr.find_last_equal_in_sorted_by(|x| x.cmp(&5)), None);
    /// ```
    fn find_last_equal_in_sorted_by(&self, comparator: impl FnMut(&T) -> Ordering) -> Option<&T>;

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

    /// Returns the index of the last element equal to the comparator target.
    ///
    /// The slice must be sorted according to the same ordering used by
    /// `comparator`. **If it is not, the result is undefined.** See
    /// [`Array::find_equal_in_sorted_by`] for details on the comparator
    /// direction pitfall.
    ///
    /// Time complexity: `O(log n)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 2, 2, 3];
    /// assert_eq!(arr.find_last_index_equal_in_sorted_by(|x| x.cmp(&2)), Some(3));
    /// assert_eq!(arr.find_last_index_equal_in_sorted_by(|x| x.cmp(&5)), None);
    /// ```
    fn find_last_index_equal_in_sorted_by(
        &self,
        comparator: impl FnMut(&T) -> Ordering,
    ) -> Option<usize>;

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

    /// Returns all elements equal to the comparator target.
    ///
    /// See [`Array::slice_equal_in_sorted_by`] for a zero-copy variant.
    ///
    /// The slice must be sorted according to the same ordering used by
    /// `comparator`. **If it is not, the result is undefined.** See
    /// [`Array::find_equal_in_sorted_by`] for details on the comparator
    /// direction pitfall.
    ///
    /// Time complexity: `O(log n + k)`, where `k` is the number of matches.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 2, 2, 3];
    /// assert_eq!(arr.filter_equal_in_sorted_by(|x| x.cmp(&2)), vec![2, 2, 2]);
    /// assert_eq!(arr.filter_equal_in_sorted_by(|x| x.cmp(&5)), Vec::<i32>::new());
    /// ```
    fn filter_equal_in_sorted_by(&self, comparator: impl FnMut(&T) -> Ordering) -> Vec<T>
    where
        T: Clone;

    /// Flattens a level of nesting in an array of iterables.
    ///
    /// Elements are iterated by reference; only the items are cloned.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [vec![1, 2], vec![3, 4]];
    /// assert_eq!(arr.flat(), vec![1, 2, 3, 4]);
    /// ```
    fn flat<'a, O>(&'a self) -> Vec<O>
    where
        T: 'a,
        &'a T: IntoIterator<Item = &'a O>,
        O: Clone + 'a;

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
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3];
    /// assert_eq!(arr.join(", "), "1, 2, 3");
    /// ```
    fn join<S>(&self, separator: &S) -> String
    where
        S: std::fmt::Display + ?Sized,
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

    /// Returns the subslice of all elements equal to the comparator target.
    ///
    /// Equal elements in a sorted slice are contiguous, so this is the
    /// zero-copy counterpart of [`Array::filter_equal_in_sorted_by`].
    /// Returns an empty slice if there is no match.
    ///
    /// The slice must be sorted according to the same ordering used by
    /// `comparator`. **If it is not, the result is undefined.** See
    /// [`Array::find_equal_in_sorted_by`] for details on the comparator
    /// direction pitfall.
    ///
    /// Time complexity: `O(log n)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 2, 2, 3];
    /// assert_eq!(arr.slice_equal_in_sorted_by(|x| x.cmp(&2)), &[2, 2, 2]);
    /// assert_eq!(arr.slice_equal_in_sorted_by(|x| x.cmp(&5)), &[]);
    /// ```
    fn slice_equal_in_sorted_by(&self, comparator: impl FnMut(&T) -> Ordering) -> &[T];

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

    /// Tests whether any element is equal to the comparator target.
    ///
    /// The slice must be sorted according to the same ordering used by
    /// `comparator`. **If it is not, the result is undefined.** See
    /// [`Array::find_equal_in_sorted_by`] for details on the comparator
    /// direction pitfall.
    ///
    /// Time complexity: `O(log n)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 2, 3];
    /// assert!(arr.some_equal_in_sorted_by(|x| x.cmp(&2)));
    /// assert!(!arr.some_equal_in_sorted_by(|x| x.cmp(&5)));
    /// ```
    fn some_equal_in_sorted_by(&self, comparator: impl FnMut(&T) -> Ordering) -> bool;

    /// Tests whether no elements match the predicate.
    ///
    /// Returns `true` if the predicate returns `false` for every element,
    /// or if the array is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 3];
    /// assert!(arr.none(|x| x > &10));
    /// assert!(!arr.none(|x| x > &2));
    /// ```
    fn none(&self, predicate: impl FnMut(&T) -> bool) -> bool;

    /// Tests whether no element is equal to the comparator target.
    ///
    /// The slice must be sorted according to the same ordering used by
    /// `comparator`. **If it is not, the result is undefined.** See
    /// [`Array::find_equal_in_sorted_by`] for details on the comparator
    /// direction pitfall.
    ///
    /// Time complexity: `O(log n)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ps_util::Array;
    /// let arr = [1, 2, 2, 3];
    /// assert!(arr.none_equal_in_sorted_by(|x| x.cmp(&5)));
    /// assert!(!arr.none_equal_in_sorted_by(|x| x.cmp(&2)));
    /// ```
    fn none_equal_in_sorted_by(&self, comparator: impl FnMut(&T) -> Ordering) -> bool;

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

    fn every_equal_in_sorted_by(&self, mut comparator: impl FnMut(&T) -> Ordering) -> bool {
        let slice = self.as_ref();

        match slice {
            [] => true,
            [only] => comparator(only).is_eq(),
            [first, .., last] => comparator(first).is_eq() && comparator(last).is_eq(),
        }
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

    fn filter_equal_in_sorted_by(&self, comparator: impl FnMut(&T) -> Ordering) -> Vec<T>
    where
        T: Clone,
    {
        self.slice_equal_in_sorted_by(comparator).to_vec()
    }

    fn find(&self, mut predicate: impl FnMut(&T) -> bool) -> Option<&T> {
        Iterator::find(&mut self.as_ref().iter(), |item| predicate(item))
    }

    fn find_equal_in_sorted_by(&self, mut comparator: impl FnMut(&T) -> Ordering) -> Option<&T> {
        let slice = self.as_ref();

        equal_index_by(slice, &mut comparator).map(|idx| &slice[idx])
    }

    fn find_index(&self, predicate: impl FnMut(&T) -> bool) -> Option<usize> {
        self.as_ref().iter().position(predicate)
    }

    fn find_index_equal_in_sorted_by(
        &self,
        mut comparator: impl FnMut(&T) -> Ordering,
    ) -> Option<usize> {
        equal_index_by(self.as_ref(), &mut comparator)
    }

    fn find_last(&self, mut predicate: impl FnMut(&T) -> bool) -> Option<&T> {
        self.as_ref().iter().rfind(|item| predicate(item))
    }

    fn find_last_equal_in_sorted_by(
        &self,
        mut comparator: impl FnMut(&T) -> Ordering,
    ) -> Option<&T> {
        let slice = self.as_ref();

        equal_last_index_by(slice, &mut comparator).map(|idx| &slice[idx])
    }

    fn find_last_index(&self, predicate: impl FnMut(&T) -> bool) -> Option<usize> {
        self.as_ref().iter().rposition(predicate)
    }

    fn find_last_index_equal_in_sorted_by(
        &self,
        mut comparator: impl FnMut(&T) -> Ordering,
    ) -> Option<usize> {
        equal_last_index_by(self.as_ref(), &mut comparator)
    }

    fn flat<'a, O>(&'a self) -> Vec<O>
    where
        T: 'a,
        &'a T: IntoIterator<Item = &'a O>,
        O: Clone + 'a,
    {
        self.as_ref().iter().flatten().cloned().collect()
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

    fn join<S>(&self, separator: &S) -> String
    where
        S: std::fmt::Display + ?Sized,
        T: std::fmt::Display,
    {
        let mut iter = self.as_ref().iter();
        let first = iter.next().map(ToString::to_string).unwrap_or_default();

        iter.fold(first, |mut out, item| {
            #[allow(clippy::expect_used)]
            write!(out, "{separator}{item}").expect("writing to String failed");
            out
        })
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

    fn slice_equal_in_sorted_by(&self, mut comparator: impl FnMut(&T) -> Ordering) -> &[T] {
        let slice = self.as_ref();

        let Some((start, end)) = equal_range_by(slice, &mut comparator) else {
            return &[];
        };

        &slice[start..end]
    }

    fn some(&self, predicate: impl FnMut(&T) -> bool) -> bool {
        self.as_ref().iter().any(predicate)
    }

    fn some_equal_in_sorted_by(&self, mut comparator: impl FnMut(&T) -> Ordering) -> bool {
        equal_index_by(self.as_ref(), &mut comparator).is_some()
    }

    fn none(&self, predicate: impl FnMut(&T) -> bool) -> bool {
        !self.some(predicate)
    }

    fn none_equal_in_sorted_by(&self, comparator: impl FnMut(&T) -> Ordering) -> bool {
        !self.some_equal_in_sorted_by(comparator)
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

fn lower_bound_by<T, F>(slice: &[T], comparator: &mut F) -> usize
where
    F: FnMut(&T) -> Ordering,
{
    slice.partition_point(|item| comparator(item).is_lt())
}

fn equal_index_by<T, F>(slice: &[T], comparator: &mut F) -> Option<usize>
where
    F: FnMut(&T) -> Ordering,
{
    let idx = lower_bound_by(slice, comparator);
    (idx < slice.len() && comparator(&slice[idx]).is_eq()).then_some(idx)
}

fn upper_bound_by<T, F>(slice: &[T], comparator: &mut F) -> usize
where
    F: FnMut(&T) -> Ordering,
{
    slice.partition_point(|item| !comparator(item).is_gt())
}

fn equal_last_index_by<T, F>(slice: &[T], comparator: &mut F) -> Option<usize>
where
    F: FnMut(&T) -> Ordering,
{
    let end = upper_bound_by(slice, comparator);
    (end > 0 && comparator(&slice[end - 1]).is_eq()).then(|| end - 1)
}

fn equal_range_by<T, F>(slice: &[T], comparator: &mut F) -> Option<(usize, usize)>
where
    F: FnMut(&T) -> Ordering,
{
    let start = equal_index_by(slice, comparator)?;
    let end = start + upper_bound_by(&slice[start..], comparator);

    (start < end).then_some((start, end))
}
