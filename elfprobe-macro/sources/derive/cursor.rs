use std::cell::Cell;
use std::fmt;
use std::marker::PhantomData;
use std::ops::Range;

use super::buffer::Buffer;
use super::entry::Entry;
use super::entry::Group;
use super::entry::Identifier;
use super::entry::Literal;
use super::entry::Punctuation;
use super::parser::Parse;
use super::parser::Peek;

// ╦ ╦┌─┐┌─┐┌┬┐
// ╠═╣├┤ ├─┤ ││
// ╩ ╩└─┘┴ ┴╶┴┘

#[derive(Copy, Clone, Debug)]
pub(self) struct Head {
  pub current: *const Entry,
  // pub start: *const Entry,
  pub stop: *const Entry,
}

impl Head {
  pub(self) fn new(range: Range<*const Entry>) -> Self {
    let Range { start, end: stop } = range;
    Head {
      current: start,
      // start: start,
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
  pub(crate) fn entry(&self) -> &'buffer Entry {
    unsafe { &*self.head.get().current }
  }

  #[inline(always)]
  pub(self) fn step(&self) -> Cursor<'buffer> {
    Cursor::from_head(unsafe { self.head.get().step() })
  }
}

// ╔═╗┌─┐┬─┐┌─┐┌─┐┬─┐
// ╠═╝├─┤├┬┘└─┐├┤ ├┬┘
// ╩  ┴ ┴┴└─└─┘└─┘┴└─

// For convenientness.
impl<'buffer> Cursor<'buffer> {
  #[allow(unused)]
  #[inline(always)]
  // Parses and moves the cursor.
  pub(crate) fn parse<Type: Parse>(&'buffer self) -> Option<Type> {
    Type::parse(self)
  }

  #[allow(unused)]
  #[inline(always)]
  // Peek and does not move the cursor.
  pub(crate) fn peek<Type: Peek>(&'buffer self) -> bool {
    Type::peek(self)
  }
}

// ╔╦╗┌─┐┬┌─┌─┐
//  ║ ├─┤├┴┐├┤
//  ╩ ┴ ┴┴ ┴└─┘

pub(crate) type TakeResult<'buffer, Type> = (&'buffer Type, Cursor<'buffer>);

pub(crate) trait Take<'buffer, Type> {
  fn take(&self) -> Option<TakeResult<'buffer, Type>>;
}

macro_rules! implement_take {
  ($($token: ident),*) => {
    $(
      impl<'buffer> Take<'buffer, $token> for Cursor<'buffer> {
        fn take(&self) -> Option<TakeResult<'buffer, $token>> {
          match self.entry() {
            Entry::$token(token) => Some((token, self.step())),
            _ => None,
          }
        }
      }
    )*
  };
}

implement_take!(Identifier, Group, Literal, Punctuation);

// For ease of use.
impl<'buffer> Cursor<'buffer> {
  pub(crate) fn is_end(&self) -> bool {
    matches!(self.entry(), Entry::End())
  }

  pub(crate) fn take<Type>(&self) -> Option<TakeResult<'buffer, Type>>
  where
    Self: Take<'buffer, Type>,
  {
    Take::take(self)
  }
}
