use std::mem::size_of;

use super::error::BytesError;
use super::pod::Pod;

///
/// Declare a trait to abstract the reading of data or data blocks. In this way,
/// `Reader` implementors can load data from heap-allocated memory or
/// memory-mapped file in order to return references to data (instead of copying
/// them) and minimize I/O calls.
///
/// All methods take `self` by value as `Self` is intended to be a reference and
/// the `Copy` and `Clone` bounds are to enforce this requirement (all shared
/// references `&T` implement `Copy`).
///
/// (Inspiref from [`object::read::ReadRef`][read_ref] trait crate.)
///
/// [read_ref]: https://tidelabs.github.io/tidext/object/read/trait.ReadRef.html
///
//
// # Developer notes
//
// ## `Copy` trait
//
// The `Copy` trait allows for simple bitwise copies of data and is a
// `std::marker` trait with no methods. The `Copy` trait can only be
// implemented by types whose fields also implement `Copy`. Certain types like
// `String` and `Vec<T>` are not `Copy` because they contain heap-allocated
// data, which would duplicate the pointer causing a double-free, or like
// `&mut T` which would create multiple mutable shared references (multiple
// non-mutable shared references `&T` are fine though). Additionally, types
// cannot implement `Copy` if they have a destructor (`Drop`), as this
// indicates a need for more complex memory management. By default, variable
// bindings have *move semantics*. However, if a type implements `Copy`, it
// instead has *copy semantics*.
//
// ## `Clone` trait
//
// On the other hand, the `Clone` trait is more flexible and allows for deeper
// duplication, which can accommodate heap-allocated types. `Clone` is
// considered a super-trait of `Copy`, meaning that any type implementing
// `Copy` must also implement `Clone`. Unlike `Copy`, where the duplication is
// implicit and efficient, cloning is always an explicit action and can be more
// resource-intensive.
//
// ## Conclusion
//
// Use `Copy` trait on reasonably small data that live entirely on the stack
// and, au contraire, do not use `Copy` when data does not fully live on the
// stack and when it violate Rust's owership model (multiple shared references,
// double free potential, etc.).
//
// ## Sources
//
// - <https://oswalt.dev/2023/12/copy-and-clone-in-rust/>
// - <https://doc.rust-lang.org/std/marker/trait.Copy.html>
// - <https://doc.rust-lang.org/std/clone/trait.Clone.html>
//
pub trait Reader<'data>: Copy + Clone {
  #[allow(unused)]
  /// Returns the total length, this description is useless because it's obvious.
  fn length(self) -> usize;

  #[allow(unused)]
  fn read_bytes(self, size: usize, offset: usize) -> Option<&'data [u8]>;

  #[allow(unused)]
  fn read_pod<Type: Pod>(self, offset: usize) -> Result<&'data Type, BytesError> {
    match self.read_bytes(size_of::<Type>(), offset) {
      // TODO: Technically, it is not empty because it does not exist.
      None => Err(BytesError::Empty),
      Some(bytes) => Type::from_bytes(bytes),
    }
  }
}

///
/// The Reader implementaion for a `&[u8]` should cover all usecase, namely read
/// bytes from heap-allocated memory like `Vec<u8>` or read data from
/// memory-mapped file.
///
impl<'data> Reader<'data> for &'data [u8] {
  #[inline]
  fn length(self) -> usize {
    self.len()
  }

  fn read_bytes(self, size: usize, offset: usize) -> Option<&'data [u8]> {
    self.get(offset..offset + size)
  }
}

// ╔╦╗┌─┐┌─┐┌┬┐┌─┐
//  ║ ├┤ └─┐ │ └─┐
//  ╩ └─┘└─┘ ┴ └─┘

#[cfg(test)]
mod tests {
  use super::*;
  use elfprobe_macro::Pod;
  use std::mem::{align_of, offset_of};

  #[test]
  fn read_bytes() {
    let slice: &[u8] = &[1, 2, 3, 4, 5, 6];
    let bytes = <&[u8] as Reader<'_>>::read_bytes(slice, 3, 2);
    assert_eq!(bytes, Some(&[3u8, 4u8, 5u8] as &[u8]));
  }

  #[test]
  fn read_pod() {
    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Pod)]
    struct Dada {
      a: u8,
      b: u64,
    }

    impl Default for Dada {
      fn default() -> Self {
        Self {
          a: 0xA0_u8,
          // The byte order has no effect this way.
          b: 0xB0_B0_B0_B0_B0_B0_B0_B0_u64,
        }
      }
    }

    assert_eq!(size_of::<Dada>(), 16);
    assert_eq!(align_of::<Dada>(), 8);
    assert_eq!(offset_of!(Dada, a), 0);
    assert_eq!(offset_of!(Dada, b), 8);

    let bytes: [u8; 8 + 16] = [
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // offset
      0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // a + align
      0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, 0xB0, // b
    ];

    let dada = bytes.read_pod::<Dada>(8);
    assert_eq!(Ok(&Dada::default()), dada);
  }
}
