use std::iter;

// When `index` == `vector.len()`, it means that the `iterator` has reached its end.

///
/// Wraps an iterator to make it bidirectional.
///
/// Not all iterators are [`DoubleEndedIterator`] (see also [`Iterator::rev()`]).
///
/// The cursor works by placing the items returned by the iterator in a
/// vector (lazily) and then moving within it on demand (see [`next()`] and
/// [`prev()`]).
///
/// [`next()`]: `Cursor::next()`
/// [`prev()`]: `Cursor::prev()`
///
pub struct Cursor<Iterator: iter::Iterator> {
  /// The actual iterator from which items are taken.
  iterator: Iterator,

  /// Whether or not the iterator has reached its end. In this case, no more
  /// calls to [`Iterator::next()`] will be made. By preventing further calls to
  /// `next()`, we preventing the iterator from potentially iterating again at
  /// some point (whose behavior is left up to implentations, see
  /// [`next()`][`Iterator::next()`] description).
  iterator_done: bool,

  /// Each element retrieved from the iterator is stored in this vector so that
  /// the cursor can move forward ([`Cursor::next()`]) and backward
  /// ([`Cursor::prev()`]), what a simple iterator cannot do.
  vector: Vec<Iterator::Item>,

  /// The cursor position in the vector. `None` indicates the beginning of the
  /// vector and `Some(vector.len())` the end, not to be confused with `Some(0)`
  /// and `Some(vector.len() - 1)` which are the first and last elements
  /// respectively.
  index: Option<usize>,
}

// I did not manage to implement `FromIterator<Iteraror::Item>`.
impl<Iteraror, IntoIter> From<IntoIter> for Cursor<Iteraror>
where
  Iteraror: iter::Iterator,
  IntoIter: iter::IntoIterator<IntoIter = Iteraror>,
{
  #[inline]
  fn from(value: IntoIter) -> Self {
    Cursor {
      iterator_done: false,
      iterator: value.into_iter(),
      vector: Vec::new(),
      index: None,
    }
  }
}

impl<Iterator: iter::Iterator> Cursor<Iterator> {
  #[allow(unused)]
  /// Moves cursor back and returns previous value.
  pub fn prev(&mut self) -> Option<&Iterator::Item> {
    // Returns None when the index is None.
    let mut index = self.index?;
    if index == 0_usize {
      self.index = None;
      return None;
    }

    index -= 1;
    // Previous elements are necessarily in the vector.
    let item = self.vector.get(index).unwrap();
    self.index = Some(index);
    Some(item)
  }

  ///
  /// Advances the cursor and returns the next value.
  ///
  /// **Reminder**:
  ///
  /// In [Rust][E0499] it is not possible to borrow a mutable variable twice
  /// (or more) at a time. The following code fails at compile time:
  ///
  /// ```should_panic
  /// # NOTE: It panics, but not for the reasons stated.
  /// # NOTE2: This is a comment and will not be in the final doc.
  /// let vec = vec![1, 2, 3, 4];
  /// let mut cursor = Cursor::from(vec);
  /// let item = cursor.next(); // First mutable borrow occurs here.
  /// cursor.next();            // Second mutable borrow occurs here.
  /// println!("{:?}", item);   // Error: First borrow later used here.
  /// ```
  ///
  /// [E0499]: https://doc.rust-lang.org/error_codes/E0499.html
  ///
  #[allow(unused)]
  pub fn next(&mut self) -> Option<&Iterator::Item> {
    // NOTE:
    // Cannot borrow `self.vector` as mutable because it is also borrowed as immutable.
    //
    // | if let Some(item) = self.vector.get(0) {
    // |   return Some(item);
    // | }
    // |
    // | if let Some(item) = self.iterator.next() {
    // |   self.vector.push(item);
    // |   ^^^^^^^^^^^^^^^^^^^^^^ Mutable borrow occurs here.
    // | }
    //
    // https://doc.rust-lang.org/error_codes/E0502.html
    // https://doc.rust-lang.org/error_codes/E0506.html

    let index = self.index.map_or(0, |i| i + 1);
    if index < self.vector.len() {
      let item = self.vector.get(index).unwrap();
      self.index = Some(index);
      return Some(item);
    }

    // From `iter::Iterator::next()` documentation:
    // > next() returns None when iteration is finished. Individual iterator
    // > implementations may choose to resume iteration, and so calling next()
    // > again may or may not eventually start returning Some(Item) again at
    // > some point.
    //
    // The "iterator_done" boolean attribute prevents this behavior and avoids
    // potential duplicates in the vector.

    if !self.iterator_done {
      if let Some(item) = self.iterator.next() {
        self.vector.push(item);
        let last = self.vector.last().unwrap();
        self.index = Some(self.vector.len() - 1);
        return Some(last);
      }
    }

    self.iterator_done = true;
    // By positioning the index outside the vector, the next call to prev() will
    // decrement the index and take the last element.
    self.index = Some(self.vector.len());
    #[allow(clippy::needless_return)]
    return None;
  }
}

#[test]
fn test_cursor() {
  let vec = vec![1, 2, 3, 4];
  let mut c = Cursor::from(vec);
  assert_eq!(c.next(), Some(&1));

  assert_eq!(c.next(), Some(&2));
  assert_eq!(c.prev(), Some(&1));
  assert_eq!(c.next(), Some(&2));
  assert_eq!(c.next(), Some(&3));

  assert_eq!(c.prev(), Some(&2));
  assert_eq!(c.prev(), Some(&1));
  assert_eq!(c.prev(), None);
  assert_eq!(c.prev(), None);
  assert_eq!(c.prev(), None);

  assert_eq!(c.next(), Some(&1));
  assert_eq!(c.next(), Some(&2));
  assert_eq!(c.next(), Some(&3));
  assert_eq!(c.next(), Some(&4));

  assert_eq!(c.next(), None);
  assert_eq!(c.next(), None);

  assert_eq!(c.prev(), Some(&4));
  assert_eq!(c.prev(), Some(&3));
  assert_eq!(c.prev(), Some(&2));
  assert_eq!(c.prev(), Some(&1));

  assert_eq!(c.prev(), None);
  assert_eq!(c.prev(), None);
  assert_eq!(c.prev(), None);
  assert_eq!(c.prev(), None);
  assert_eq!(c.prev(), None);
}
