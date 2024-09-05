use std::iter;

// When `index` == `vector.len()`, it means that the `iterator` has reached its end.
pub struct Cursor<Iterator: iter::Iterator> {
  vector: Vec<Box<Iterator::Item>>,
  index: Option<usize>,
  iterator: Iterator,
}

impl<Iterator: iter::Iterator> Cursor<Iterator> {
  #[inline]
  #[allow(unused)]
  pub fn new(iterator: Iterator) -> Cursor<Iterator> {
    Cursor {
      vector: Vec::new(),
      index: None,
      iterator,
    }
  }

  #[allow(unused)]
  pub fn prev(&mut self) -> Option<&Iterator::Item> {
    let mut index = self.index?;

    if index == 0_usize {
      self.index = None;
      return None;
    }

    index -= 1;
    let item = self.vector.get(index)?;
    self.index = Some(index);
    Some(item.as_ref())
  }

  #[allow(unused)]
  pub fn next(&mut self) -> Option<&Iterator::Item> {
    if self.index == Some(self.vector.len()) {
      return None; // TODO: Comment why.
    }

    let index = self.index.map_or(0, |i| i + 1);
    if index < self.vector.len() {
      // Is it better to unwrap() instead?
      let item = self.vector.get(index)?;
      self.index = Some(index);
      return Some(item.as_ref());
    }

    if let Some(item) = self.iterator.next() {
      self.vector.push(Box::new(item));
      let last = self.vector.last().unwrap();
      self.index = Some(self.vector.len() - 1);
      return Some(last.as_ref());
    }

    // Set the index outside the vector.
    self.index = Some(self.vector.len());
    #[allow(clippy::needless_return)]
    return None;
  }
}

#[test]
fn test_cursor() {
  let vec = vec![1, 2, 3, 4];
  let mut c = Cursor::new(vec.into_iter());
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
