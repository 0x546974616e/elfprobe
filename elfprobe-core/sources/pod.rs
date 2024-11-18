use std::slice;

#[allow(unused)]
use std::mem::align_of;
use std::mem::size_of;

use crate::reader::Reader;

#[derive(Debug, PartialEq, Eq)]
pub enum PodError {
  /// Multiplication overflow detected while computing slice size.
  CountOverflow { count: usize, size: usize },

  /// Bytes array pointer is not aligned with the output type.
  #[allow(unused)] // Only used when cfg(not(feature = "unaligned"))
  NotAligned { alignment: usize, pointer: usize },

  /// Bytes array size is not equal to the output type size.
  NotEnoughSpace {
    expected: usize,
    offset: usize,
    found: usize,
  },
}

///
/// TLDR: A POD type is a bag of bits with no magic.
///
/// A POD (Plain Old Data) type is all primitive types (`u8`, `i32`, `i64`...)
/// and all aggregations of POD types (`struct`, `union`...). A POD structure
/// contains only POD types as members and does not have any constructors,
/// destructors and virtual members functions.
///
/// The following trait bounds are here to enforce [the idea of POD type in
/// Rust][rust_pod]:
///
/// - [`'static`][static] as a trait bound means that the type does not contain
///   any internal non-static references (`&T` and `&mut T`). Type that are
///   `'static` have therefore no lifetime restrictions and will be basically
///   ignored by the borrow checker.
///
/// - [`Copy`] trait allows values to be duplicated simply by copying its bits
///   (no move semantics). `Copy` trait is then implemented by types that do not
///   have complex memory management with for example heap allocation (pointers)
///   or shared mutable references (`&mut T`, note that `&T` is `Copy` though).
///
/// - [`Sized`] trait requires that the type has a size known at compile time
///   and can thus be stored on the stack.
///
/// - [`Send`] and [`Sync`] require that the type can be sent to other threads
///   and can shared via immutable reference (`&T`) across threads. These traits
///   are not implemented if the type contains some kind of magic (interior
///   mutability, references without a lifetime...).
///
/// [rust_pod]: https://stackoverflow.com/questions/45634083/is-there-a-concept-of-pod-types-in-rust
/// [static]: https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html#trait-bound
///
#[allow(unused)]
// TODO: Add Send + Sync
pub trait Pod: 'static + Copy + Sized {
  #[inline(always)]
  fn from_reader_at<'data>(reader: impl Reader<'data>, offset: usize) -> Result<&'data Self, PodError> {
    reader.read_pod_at(offset)
  }

  #[allow(clippy::needless_lifetimes)] // For readability.
  fn from_bytes_at<'data>(bytes: &'data [u8], offset: usize) -> Result<&'data Self, PodError> {
    match bytes.get(offset..size_of::<Self>()) {
      None => Err(PodError::NotEnoughSpace {
        expected: size_of::<Self>(),
        found: bytes.len(),
        offset,
      }),

      // What about std::ptr::read*() methods?
      // What kind of security do they provide?
      // https://doc.rust-lang.org/std/ptr/fn.read.html
      // https://doc.rust-lang.org/std/ptr/fn.read_volatile.html
      // https://doc.rust-lang.org/std/ptr/fn.read_unaligned.html

      // From read_volatile()
      // Rust does not currently have a rigorously and formally defined memory
      // model, so the precise semantics of what “volatile” means here is subject
      // to change over time. That being said, the semantics will almost always
      // end up pretty similar to C11’s definition of volatile.

      // From INTERNATIONAL STANDARD - Programming languages - C11
      // https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf
      // Page 122, §6.7.3, footnote 134
      // A **volatile** declaration may be used to describe an object
      // corresponding to a memory-mapped input/output port or an object accessed
      // by an asynchronously interrupting function. Actions on objects so
      // declared shall not be ‘‘optimized out’’ by an implementation or reordered
      // except as permitted by the rules for evaluating expressions.

      // From https://stackoverflow.com/a/29102709
      // This is one of the two situations where volatile is mandatory (and it
      // would be nice if compilers could know that).
      //
      // Any memory location which can change either without your code initiating
      // it (I.e. a memory mapped device register) or without your thread
      // initiating it (i.e. it is changed by another thread or by an interrupt
      // handler) absolutely must be declared as volatile to prevent the compiler
      // optimizing away memory-fetch operations.
      //
      // Your answer is incomplete, as it only focuses on the fetch aspect.
      // There's a complimentary requirement for store.
      Some(data) => {
        let pointer = data.as_ptr();
        #[cfg(any(clippy, not(feature = "unaligned")))]
        if (pointer as usize) % align_of::<Self>() != 0 {
          return Err(PodError::NotAligned {
            alignment: align_of::<Self>(),
            pointer: pointer as usize,
          });
        }

        Ok(unsafe { &*pointer.cast::<Self>() })
      }
    }
  }

  #[inline(always)]
  fn slice_from_reader_at<'data>(
    reader: impl Reader<'data>,
    offset: usize,
    count: usize,
  ) -> Result<&'data [Self], PodError> {
    reader.read_pod_slice_at(count, offset)
  }

  fn slice_from_bytes_at<'data>(
    bytes: &'data [u8],
    offset: usize,
    count: usize,
  ) -> Result<&'data [Self], PodError> {
    match count.checked_mul(size_of::<Self>()) {
      None => Err(PodError::CountOverflow {
        size: size_of::<Self>(),
        count,
      }),

      Some(size) => match bytes.get(offset..size) {
        None => Err(PodError::NotEnoughSpace {
          found: bytes.len(),
          expected: size,
          offset,
        }),

        Some(data) => {
          let pointer = data.as_ptr();
          #[cfg(any(clippy, not(feature = "unaligned")))]
          if (pointer as usize) % align_of::<Self>() != 0 {
            return Err(PodError::NotAligned {
              alignment: align_of::<Self>(),
              pointer: pointer as usize,
            });
          }

          Ok(unsafe { slice::from_raw_parts(pointer.cast(), count) })
        }
      },
    }
  }
}

macro_rules! impl_pod {
  ($($bytes: literal),+, $($type: ident),+) => {
    $(impl Pod for [u8; $bytes] {})+
    $(impl Pod for $type {})+
  };
}

// Implement POD trait for primitive types in order to be used by POD aggregates.
impl_pod!(2, 4, 8, i8, u8, i16, u16, i32, u32, i64, u64);

// ╔╦╗┌─┐┌─┐┌┬┐┌─┐
//  ║ ├┤ └─┐ │ └─┐
//  ╩ └─┘└─┘ ┴ └─┘

#[cfg(test)]
mod tests {
  use super::*;
  use elfprobe_macro::Pod;
  use std::mem::offset_of;

  #[repr(C)]
  #[derive(Pod, Debug, Copy, Clone, PartialEq, Eq)]
  struct Dada {
    a: u64,
    b: u32,
    c: u16,
    d: u8,
  }

  #[test]
  fn from_bytes_ok() {
    assert_eq!(size_of::<Dada>(), 16);
    assert_eq!(align_of::<Dada>(), 8);
    assert_eq!(offset_of!(Dada, a), 0);
    assert_eq!(offset_of!(Dada, b), 8);
    assert_eq!(offset_of!(Dada, c), 12);
    assert_eq!(offset_of!(Dada, d), 14);

    let bytes: [u8; 8 + 16] = [
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // offset
      // Bytes order has no effect on endianness this way.
      0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, // a
      0x03, 0x03, 0x03, 0x03, // b
      0x02, 0x02, // c
      0x01, // d
      0x00,
    ];

    assert_eq!(
      Dada::from_bytes_at(&bytes, 8),
      Ok(&Dada {
        a: 0x04_04_04_04_04_04_04_04_u64,
        b: 0x03_03_03_03_u32,
        c: 0x02_02_u16,
        d: 0x01_u8,
      }),
    );
  }

  #[test]
  fn from_bytes_size_of_error() {
    assert_eq!(size_of::<Dada>(), 16);
    assert_eq!(align_of::<Dada>(), 8);

    assert_eq!(
      Dada::from_bytes_at(&[1, 2, 3], 0),
      Err(PodError::NotEnoughSpace {
        expected: 16,
        offset: 0,
        found: 3,
      }),
    )
  }

  #[test]
  #[cfg(any(clippy, not(feature = "unaligned")))]
  fn from_bytes_align_of_error() {
    assert_eq!(size_of::<Dada>(), 16);
    assert_eq!(align_of::<Dada>(), 8);

    let bytes: &[u8; 1 + 16] = &[
      0x00, // To make sure it is unaligned.
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
    ];

    let slice = &bytes[1..];
    assert_eq!(
      Dada::from_bytes_at(slice, 0),
      Err(PodError::NotAligned {
        pointer: slice.as_ptr() as usize,
        alignment: 8,
      }),
    )
  }

  #[test]
  fn slice_from_bytes_ok() {
    assert_eq!(size_of::<Dada>(), 16);
    assert_eq!(align_of::<Dada>(), 8);
    assert_eq!(offset_of!(Dada, a), 0);
    assert_eq!(offset_of!(Dada, b), 8);
    assert_eq!(offset_of!(Dada, c), 12);
    assert_eq!(offset_of!(Dada, d), 14);

    let bytes: [u8; 8 + 16 * 3] = [
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // offset
      // Bytes order has no effect on endianness this way.
      0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, // a
      0x22, 0x22, 0x22, 0x22, // b
      0x33, 0x33, // c
      0x44, // d
      0x00, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, // a
      0xBB, 0xBB, 0xBB, 0xBB, // b
      0xCC, 0xCC, // c
      0xDD, // d
      0x00, 0xA1, 0xA1, 0xA1, 0xA1, 0xA1, 0xA1, 0xA1, 0xA1, // a
      0xB2, 0xB2, 0xB2, 0xB2, // b
      0xC3, 0xC3, // c
      0xD4, // d
      0x00,
    ];

    assert_eq!(
      Dada::slice_from_bytes_at(&bytes, 8, 3),
      Ok(&[
        Dada {
          a: 0x11_11_11_11_11_11_11_11_u64,
          b: 0x22_22_22_22_u32,
          c: 0x33_33_u16,
          d: 0x44_u8,
        },
        Dada {
          a: 0xAA_AA_AA_AA_AA_AA_AA_AA_u64,
          b: 0xBB_BB_BB_BB_u32,
          c: 0xCC_CC_u16,
          d: 0xDD_u8,
        },
        Dada {
          a: 0xA1_A1_A1_A1_A1_A1_A1_A1_u64,
          b: 0xB2_B2_B2_B2_u32,
          c: 0xC3_C3_u16,
          d: 0xD4_u8,
        },
      ] as &[Dada]),
    );
  }
}
