use std::{error, fmt, result};

#[allow(unused)]
// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

// ╔╗ ┬ ┬┌┬┐┌─┐┌─┐
// ╠╩╗└┬┘ │ ├┤ └─┐
// ╚═╝ ┴  ┴ └─┘└─┘

#[derive(Debug, PartialEq, Eq)]
pub enum BytesError {
  Empty,
  SizeOfMismatch {
    length: usize,
    size_of: usize,
  },
  #[allow(unused)] // Only used when cfg(not(feature = "unaligned"))
  AlignOfMismatch {
    pointer: usize,
    align_of: usize,
  },
}

impl fmt::Display for BytesError {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Empty => write!(formatter, "bytes.len() != 0"),

      Self::SizeOfMismatch { length, size_of } => {
        write!(
          formatter,
          "bytes.len() != size_of::<Pod>(), {} != {}",
          length, size_of,
        )
      }

      Self::AlignOfMismatch { pointer, align_of } => {
        write!(
          formatter,
          "bytes.as_ptr() % align_of::<Pod>() != 0, {:p} % {} == {}",
          pointer,
          align_of,
          pointer % align_of,
        )
      }
    }
  }
}

impl error::Error for BytesError {}
