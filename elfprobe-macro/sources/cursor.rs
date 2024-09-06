//! TODO: Remove `#[allow(unused)]` when done.
use std::cell::RefCell;
use std::cell::RefMut;
use std::iter;
use std::rc::Rc;

/// The result of [`Cursor::peek_at()`].
#[allow(clippy::needless_pub_self)]
pub(self) enum PeekResult<Item> {
  /// Everything is fine, an existing item is returned.
  Item(Item),

  /// The [end][`Cursor::iterator_done`] of the [iterator][`Cursor::iterator`]
  /// has been reached, the length of the [vector][`Cursor::vector`] is
  /// returned.
  EndReached(usize),
}

impl<Item> PeekResult<Item> {
  pub(self) fn unwrap_item(self) -> Item {
    match self {
      Self::Item(item) => item,
      Self::EndReached(_index) => {
        panic!("Called PeekResult::unwrap_item() on a EndReached value.")
      }
    }
  }
}

// ╔═╗┬ ┬┬─┐┌─┐┌─┐┬─┐
// ║  │ │├┬┘└─┐│ │├┬┘
// ╚═╝└─┘┴└─└─┘└─┘┴└─

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

// I did not manage to implement `FromIterator<Iterator::Item>`.
impl<Iterator, IntoIterator> From<IntoIterator> for Cursor<Iterator>
where
  Iterator: iter::Iterator,
  IntoIterator: iter::IntoIterator<IntoIter = Iterator>,
{
  #[inline]
  fn from(value: IntoIterator) -> Self {
    Cursor::new(value.into_iter())
  }
}

impl<Iterator: iter::Iterator> Cursor<Iterator> {
  #[inline]
  pub fn new(iterator: Iterator) -> Self {
    Cursor {
      iterator,
      iterator_done: false,
      vector: Vec::new(),
      index: None,
    }
  }

  #[allow(unused)]
  /// Moves cursor back and returns previous value.
  pub fn prev(&mut self) -> Option<&mut Iterator::Item> {
    // Returns None when the index is None.
    let mut index = self.index?;
    if index == 0_usize {
      self.index = None;
      return None;
    }

    index -= 1;
    // Previous elements are necessarily in the vector.
    let item = self.vector.get_mut(index).unwrap();
    self.index = Some(index);
    Some(item)
  }

  ///
  /// Advances the cursor and returns the next value.
  ///
  /// **Reminder**:
  ///
  /// In Rust it is [not possible][E0499] to borrow a mutable variable twice
  /// (or more) at a time. The following code fails at compile time:
  ///
  /// ```compile_fail
  /// # // NOTE: It fails, but not for the reasons stated.
  /// # // NOTE2: This comment will not be in the generated doc.
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
  pub fn next(&mut self) -> Option<&mut Iterator::Item> {
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
      let item = self.vector.get_mut(index).unwrap();
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
        let length = self.vector.len(); // Because of E0502...
        let last = self.vector.last_mut().unwrap();
        self.index = Some(length - 1);
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

  /// Returns the `index`-th item without moving the cursor.
  pub(self) fn nth(&mut self, index: usize) -> PeekResult<&mut Iterator::Item> {
    if index < self.vector.len() {
      let item = self.vector.get_mut(index).unwrap();
      return PeekResult::Item(item);
    }

    if !self.iterator_done {
      while self.vector.len() <= index && !self.iterator_done {
        match self.iterator.next() {
          None => self.iterator_done = true,
          Some(item) => self.vector.push(item),
        }
      }

      if index == self.vector.len() - 1 {
        let item = self.vector.last_mut().unwrap();
        return PeekResult::Item(item);
      }
    }

    let index = self.vector.len();
    PeekResult::EndReached(index)
  }
}

// ╔═╗┬ ┬┌─┐┬─┐┌─┐┌┬┐
// ╚═╗├─┤├─┤├┬┘├┤  ││
// ╚═╝┴ ┴┴ ┴┴└─└─┘╶┴┘

///
/// A [`Cursor`] that can be [forked] and [merged].
///
/// [forked]: `SharedCursor::fork()`
/// [merged]: `SharedCursor::merge()`
///
pub struct SharedCursor<Iterator: iter::Iterator> {
  inner: Rc<RefCell<Cursor<Iterator>>>,
  index: Option<usize>,
}

// I did not manage to implement `FromIterator<Iterator::Item>`.
impl<Iterator, IntoIterator> From<IntoIterator> for SharedCursor<Iterator>
where
  Iterator: iter::Iterator,
  IntoIterator: iter::IntoIterator<IntoIter = Iterator>,
{
  #[inline]
  fn from(value: IntoIterator) -> Self {
    SharedCursor {
      inner: Rc::new(RefCell::new(Cursor::new(value.into_iter()))),
      index: None,
    }
  }
}

impl<Iterator: iter::Iterator> SharedCursor<Iterator> {
  #[allow(unused)]
  /// Clones the cursor.
  pub fn fork(&self) -> Self {
    SharedCursor {
      inner: self.inner.clone(),
      index: self.index,
    }
  }

  #[allow(unused)]
  /// Merge the given cursor into this one.
  pub fn merge(&mut self, other: Self) {
    // The other's inner Rc<_> can be safely dropped.
    self.index = other.index;
  }

  #[allow(unused)]
  /// Moves cursor back and returns previous value.
  pub fn prev(&mut self) -> Option<RefMut<Iterator::Item>> {
    // Returns None when the index is None.
    let mut index = self.index?;
    if index == 0_usize {
      self.index = None;
      return None;
    }

    index -= 1;
    self.index = Some(index);
    let reference = self.inner.borrow_mut();
    Some(RefMut::map(reference, |inner| {
      // Previous element necessarily exists.
      inner.nth(index).unwrap_item()
    }))
  }

  #[allow(unused)]
  /// Advances the cursor and returns the next value.
  pub fn next(&mut self) -> Option<RefMut<Iterator::Item>> {
    let index = self.index.map_or(0, |index| index + 1);
    let mut reference = self.inner.borrow_mut();

    match reference.nth(index) {
      PeekResult::Item(_item) => {
        self.index = Some(index);
        Some(RefMut::map(reference, |inner| {
          // This should not panic, peek_at(index) already returned an item.
          // A RefMut::map() must return a &mut.
          inner.nth(index).unwrap_item()
        }))
      }

      PeekResult::EndReached(index) => {
        self.index = Some(index);
        None
      }
    }
  }
}

// ╔╦╗┌─┐┌─┐┌┬┐┌─┐
//  ║ ├┤ └─┐ │ └─┐
//  ╩ └─┘└─┘ ┴ └─┘

#[cfg(test)]
mod tests {
  use super::*;
  use std::ops::DerefMut;

  macro_rules! assert_none {
    ($option: expr) => {
      assert!($option.is_none())
    };
  }

  macro_rules! assert_some {
    ($option: expr, $value: expr) => {{
      let item = $option; // To prevent calling next() twice.
      assert!(item.is_some() && item.unwrap().deref_mut() == $value);
    }};
  }

  #[test]
  fn test_fork() {
    let vec = vec![1, 2, 3, 4];
    let mut c1 = SharedCursor::from(vec);
    assert_none!(c1.prev());
    assert_some!(c1.next(), &mut 1);
    assert_some!(c1.next(), &mut 2);

    let mut c2 = c1.fork();
    assert_some!(c1.next(), &mut 3);
    assert_some!(c1.next(), &mut 4);
    assert_none!(c1.next());

    assert_some!(c2.next(), &mut 3);
    assert_some!(c2.next(), &mut 4);
    assert_none!(c2.next());
    assert_none!(c2.next());
    assert_none!(c2.next());
    assert_none!(c2.next());

    let mut c3 = c2.fork();
    assert_some!(c2.prev(), &mut 4);
    assert_some!(c2.prev(), &mut 3);
    assert_some!(c2.prev(), &mut 2);
    assert_some!(c2.prev(), &mut 1);
    assert_none!(c2.prev());
    assert_none!(c2.prev());

    assert_none!(c3.next());
    assert_none!(c3.next());
    assert_some!(c3.prev(), &mut 4);
    assert_some!(c3.prev(), &mut 3);
    assert_some!(c3.prev(), &mut 2);

    assert_some!(c1.prev(), &mut 4);
    assert_some!(c1.prev(), &mut 3);

    c1.merge(c2);
    assert_some!(c1.next(), &mut 1);
    assert_some!(c1.next(), &mut 2);

    c3.merge(c1);
    assert_some!(c3.next(), &mut 3);
    assert_some!(c3.next(), &mut 4);
    assert_none!(c3.next());
  }

  #[test]
  fn test_cursor() {
    let vec = vec![1, 2, 3, 4];
    let mut c = Cursor::from(vec);
    assert_eq!(c.next(), Some(&mut 1));

    assert_eq!(c.next(), Some(&mut 2));
    assert_eq!(c.prev(), Some(&mut 1));
    assert_eq!(c.next(), Some(&mut 2));
    assert_eq!(c.next(), Some(&mut 3));

    assert_eq!(c.prev(), Some(&mut 2));
    assert_eq!(c.prev(), Some(&mut 1));
    assert_eq!(c.prev(), None);
    assert_eq!(c.prev(), None);
    assert_eq!(c.prev(), None);

    assert_eq!(c.next(), Some(&mut 1));
    assert_eq!(c.next(), Some(&mut 2));
    assert_eq!(c.next(), Some(&mut 3));
    assert_eq!(c.next(), Some(&mut 4));

    assert_eq!(c.next(), None);
    assert_eq!(c.next(), None);

    assert_eq!(c.prev(), Some(&mut 4));
    assert_eq!(c.prev(), Some(&mut 3));
    assert_eq!(c.prev(), Some(&mut 2));
    assert_eq!(c.prev(), Some(&mut 1));

    assert_eq!(c.prev(), None);
    assert_eq!(c.prev(), None);
    assert_eq!(c.prev(), None);
    assert_eq!(c.prev(), None);
    assert_eq!(c.prev(), None);
  }
}
