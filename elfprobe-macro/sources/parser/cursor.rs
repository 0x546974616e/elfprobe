use std::cell::Cell;
use std::marker::PhantomData;
use std::ops::Range;

use super::buffer::Buffer;
use super::entry::Entry;
use super::Parse;

#[derive(Debug, Clone)]
pub(crate) struct Cursor<'buffer> {
  _maker: PhantomData<&'buffer Entry>,
  head: Cell<*const Entry>, // Interior mutability
  end: *const Entry,
}

impl<'buffer> From<&Buffer> for Cursor<'buffer> {
  #[inline(always)]
  fn from(buffer: &Buffer) -> Self {
    let Range { start, end } = buffer.as_ptr_range();
    Cursor {
      _maker: PhantomData,
      head: Cell::new(start),
      end,
    }
  }
}

impl<'buffer> Cursor<'buffer> {
  #[inline(always)]
  pub(super) fn fork(&self) -> Self {
    self.clone()
  }

  #[inline(always)]
  pub(super) fn merge(&self, other: Self) {
    self.head.set(other.head.get());
  }

  #[inline(always)]
  pub(super) fn entry(&self) -> &'buffer Entry {
    unsafe { &*self.head.get() }
  }

  #[inline(always)]
  pub(crate) fn is_end(&self) -> bool {
    matches!(self.entry(), Entry::End())
  }

  #[inline(always)]
  // Parses and moves the cursor.
  pub(super) fn parse<Type: Parse>(&'buffer self) -> Option<Type> {
    Type::parse(self)
  }

  pub(super) fn step(&self) {
    let head = self.head.get();

    if let &Entry::End() = unsafe { &*head } {
      panic!("Try to move the cursor after the last entry.");
    }

    if head >= self.end {
      panic!("Try to move the cursor outside its scope.");
    }

    self.head.set(unsafe { head.add(1) });
  }
}
