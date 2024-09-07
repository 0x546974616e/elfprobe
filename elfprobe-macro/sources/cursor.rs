use std::cell::Cell;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Range;

use crate::entry::Entry;
use crate::entry::Group;
use crate::entry::Identifier;
use crate::entry::Literal;
use crate::entry::Punctuation;

use crate::parser::Parse;
use crate::parser::Peek;
use crate::parser::Stream;

use crate::buffer::Buffer;

// ╦ ╦┌─┐┌─┐┌┬┐
// ╠═╣├┤ ├─┤ ││
// ╩ ╩└─┘┴ ┴╶┴┘

#[derive(Copy, Clone, Debug)]
pub(self) struct Head {
  pub current: *const Entry,
  pub start: *const Entry,
  pub stop: *const Entry,
}

impl Head {
  pub(self) fn new(range: Range<*const Entry>) -> Self {
    let Range { start, end: stop } = range;
    Head {
      current: start,
      start,
      stop,
    }
  }

  pub(self) unsafe fn step(mut self) -> Self {
    if let Entry::End() = &*self.current {
      panic!("Try to move the cursor after the end entry.");
    }

    if self.current >= self.stop {
      panic!("Try to move the cursor outside its allowed region.");
    }

    self.current = self.current.add(1);
    self
  }
}

// ╔═╗┬ ┬┬─┐┌─┐┌─┐┬─┐
// ║  │ │├┬┘└─┐│ │├┬┘
// ╚═╝└─┘┴└─└─┘└─┘┴└─

pub(crate) struct Cursor<'buffer> {
  _marker: PhantomData<&'buffer ()>,

  // https://doc.rust-lang.org/error_codes/E0502.html
  // https://doc.rust-lang.org/error_codes/E0597.html
  // For interior mutability...
  head: Cell<Head>,
}

impl<'buffer> fmt::Debug for Cursor<'buffer> {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter
      .debug_struct("Cursor")
      .field("head", &self.head)
      .finish()
  }
}

// ╔═╗┬─┐┌─┐┌┬┐
// ╠╣ ├┬┘│ ││││
// ╚  ┴└─└─┘┴ ┴

impl<'buffer> From<&Buffer> for Cursor<'buffer> {
  #[inline(always)]
  fn from(buffer: &Buffer) -> Self {
    Cursor::from_range(buffer.as_ptr_range())
  }
}

impl<'buffer> Cursor<'buffer> {
  pub(self) fn from_range(range: Range<*const Entry>) -> Self {
    Cursor {
      head: Cell::new(Head::new(range)),
      _marker: PhantomData,
    }
  }

  pub(self) fn from_head(head: Head) -> Self {
    Cursor {
      head: Cell::new(head),
      _marker: PhantomData,
    }
  }
}

// ╔╦╗┌─┐┌┬┐┬ ┬┌─┐┌┬┐┌─┐
// ║║║├┤  │ ├─┤│ │ ││└─┐
// ╩ ╩└─┘ ┴ ┴ ┴└─┘╶┴┘└─┘

impl<'buffer> Cursor<'buffer> {
  #[inline(always)]
  pub(crate) fn fork(&self) -> Self {
    Cursor::from_head(self.head.get())
  }

  #[inline(always)]
  pub(crate) fn merge(&self, other: Self) {
    self.head.set(other.head.get());
  }

  #[inline(always)]
  pub(self) fn entry(&self) -> &'buffer Entry {
    unsafe { &*self.head.get().current }
  }

  #[inline(always)]
  pub(self) fn step(&self) -> Cursor<'buffer> {
    Cursor::from_head(unsafe { self.head.get().step() })
  }
}

// ╔═╗┌┐┌┌┬┐┬─┐┬ ┬
// ║╣ │││ │ ├┬┘└┬┘
// ╚═╝┘└┘ ┴ ┴└─ ┴

pub(crate) type Take<'buffer, Type> = Option<(&'buffer Type, Cursor<'buffer>)>;

impl<'buffer> Cursor<'buffer> {
  // Returns an identifier, does not move the cursor.
  pub(crate) fn identifier(&self) -> Take<Identifier> {
    match self.entry() {
      Entry::Identifier(token) => Some((token, self.step())),
      _ => None,
    }
  }

  // Returns a literal, does not move the cursor.
  pub(crate) fn literal(&self) -> Take<Literal> {
    match self.entry() {
      Entry::Literal(token) => Some((token, self.step())),
      _ => None,
    }
  }

  // Returns a punctuation, does not move the cursor.
  pub(crate) fn punctuation(&self) -> Take<Punctuation> {
    match self.entry() {
      Entry::Punctuation(token) => Some((token, self.step())),
      _ => None,
    }
  }

  // Returns a group, does not move the cursor.
  pub(crate) fn group(&self) -> Take<Group> {
    match self.entry() {
      Entry::Group(token, _offset) => Some((token, self.step())),
      _ => None,
    }
  }
}

// ╔═╗┌─┐┬─┐┌─┐┌─┐┬─┐
// ╠═╝├─┤├┬┘└─┐├┤ ├┬┘
// ╩  ┴ ┴┴└─└─┘└─┘┴└─

// For convenientness.
impl<'buffer> Cursor<'buffer> {
  #[inline(always)]
  // Parses and moves the cursor.
  pub(crate) fn parse<Type: Parse>(&'buffer self) -> Option<Type> {
    Type::parse(self)
  }

  #[inline(always)]
  // Peek and does not move the cursor.
  pub(crate) fn peek<Type: Peek>(&'buffer self) -> bool {
    Type::peek(self)
  }
}
