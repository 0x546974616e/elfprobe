use std::num::ParseIntError;
use std::{error, fmt};

// https://stackoverflow.com/a/38343355
fn to_capitalized(string: &str) -> String {
  let mut result = String::with_capacity(string.len());
  let mut chars = string.chars();

  result.extend(chars.by_ref().take(1).flat_map(|c| c.to_uppercase()));
  result.extend(chars);

  result
}

// ╔═╗┬─┐┬─┐┌─┐┬─┐
// ║╣ ├┬┘├┬┘│ │├┬┘
// ╚═╝┴└─┴└─└─┘┴└─

// NOTE:
// Wrapping errors has been deliberately chosen instead boxing them (Box<dyn>)
// in order to try out other ways of playing with errors in Rust. It may
// therefore be a clumsy way of using them.

// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html

#[derive(Debug, Clone, PartialEq, Eq)]
// Are errors allowed to have internal references?
// How errors reference the cause/source of the error then?
pub enum ChunkError<'guilty> {
  /// Chunk size must not be less than or equal to zero.
  InvalidChunkSize(),

  /// Word length must be a multiple of the chunk size.
  InvalidWordLength(&'guilty str, usize),
}

impl<'guilty> error::Error for ChunkError<'guilty> {}

impl<'guilty> fmt::Display for ChunkError<'guilty> {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::InvalidChunkSize() => {
        write!(formatter, "Chunk size must be greater than zero.")
      }
      Self::InvalidWordLength(guilty, size) => {
        write!(
          formatter,
          "Word length must be a multiple of {size:} (\"{guilty:}\").",
        )
      }
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseHexError<'guilty> {
  /// Chunk-related errors.
  ChunkError(ChunkError<'guilty>),

  /// Integer parsing errors.
  ParseError(ParseIntError, &'guilty str),

  /// So far only even word lengths are supported.
  InvalidWordLength(&'guilty str),
}

impl<'guilty> error::Error for ParseHexError<'guilty> {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match self {
      // Self::ChunkError(ref error) => Some(error),
      Self::ParseError(ref error, _) => Some(error),
      _ => None,
    }
  }
}

impl<'guilty> fmt::Display for ParseHexError<'guilty> {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::ChunkError(error) => error.fmt(formatter),
      Self::ParseError(error, guilty) => {
        write!(
          formatter,
          "{} (\"{guilty:}\").",
          // ParseIntError message are not capitalized.
          to_capitalized(&error.to_string()) // Optional
        )
      }
      Self::InvalidWordLength(guilty) => {
        write!(formatter, "Word length must be even (\"{guilty:}\").")
      }
    }
  }
}

// ╦┌┬┐┌─┐┬─┐┌─┐┌┬┐┌─┐┬─┐
// ║ │ ├┤ ├┬┘├─┤ │ │ │├┬┘
// ╩ ┴ └─┘┴└─┴ ┴ ┴ └─┘┴└─

struct ChunksIterator<'data> {
  slice: &'data str,
  size: usize,
}

impl<'data> Iterator for ChunksIterator<'data> {
  type Item = &'data str;

  fn next(&mut self) -> Option<Self::Item> {
    let chunk = self.slice.get(..self.size)?;
    self.slice = self.slice.get(self.size..)?;
    Some(chunk)
  }
}

type ChunksResult<'data> = Result<ChunksIterator<'data>, ChunkError<'data>>;

trait IntoChunks<'data> {
  fn into_chunks(self, size: usize) -> ChunksResult<'data>;
}

impl<'data> IntoChunks<'data> for &'data str {
  fn into_chunks(self, size: usize) -> ChunksResult<'data> {
    if size == 0 {
      return Err(ChunkError::InvalidChunkSize());
    }

    if self.len() % size != 0 {
      return Err(ChunkError::InvalidWordLength(self, size));
    }

    Ok(ChunksIterator { slice: self, size })
  }
}

// ╔═╗┌─┐┬─┐┌─┐┌─┐┬─┐
// ╠═╝├─┤├┬┘└─┐├┤ ├┬┘
// ╩  ┴ ┴┴└─└─┘└─┘┴└─

#[inline]
#[allow(unused)]
fn remove_comment(string: &str) -> &str {
  match string.split_once(';') {
    Some((string, _comment)) => string,
    None => string,
  }
}

#[inline]
#[allow(unused)]
fn find_words(string: &str) -> impl Iterator<Item = &str> {
  string.lines().map(remove_comment).flat_map(str::split_whitespace)
}

///
/// Parse all hexadecimal representations and character sequences of the given
/// string into a vector of bytes (`Vec<u8>`).
///
/// Here are the few simple syntax rules:
/// - the input string can be of any number of lines, empty or not;
/// - whitespaces are reserved to separate words (add their hexadecimal literal
///   counterpart to keep them, `20` for space for instance);
/// - single quotes (`'`) indicate the beginning of a literal string up to the
///   first whitespace (there is no closing quote). The first simple quotes are
///   removed from the output (`''` to escape them);
/// - dots (`.`) work like single quotes but are preserved in the output;
/// - semicolons (`;`) are reserved characters that mark the beginning of a
///   comment (therefore completely removed from the output);
/// - all other sequences must be even to be interpreted as valid hexadecimal
///   integers (`12`, `BEEF`...).
///
/// Example:
/// ```rust
/// let bytes = parse_hex("1A2B FF 'Ab 20 .cd ; Comment");
/// let expected = vec![0x1A, 0x2B, 0xFF, 0x41, 0x62, 0x20, 0x2E, 0x63, 0x64];
/// assert_eq!(bytes.unwrap(), expected);
/// ```
///
#[allow(unused)]
pub fn parse_hex(string: &str) -> Result<Vec<u8>, ParseHexError> {
  // I find this function still amateurish in its use of Rust,
  // how to improve it? Is it necessary? (readability first)
  let mut buffer: Vec<u8> = Vec::new();

  for word in find_words(string) {
    if word.is_empty() {
      continue;
    }

    // TODO: % 2
    if word.len() == 1 {
      return Err(ParseHexError::InvalidWordLength(word));
    }

    if word.starts_with('.') {
      buffer.extend_from_slice(word.as_bytes());
      continue;
    }

    if let Some(stripped) = word.strip_prefix('\'') {
      buffer.extend_from_slice(stripped.as_bytes());
      continue;
    }

    match word.into_chunks(2) {
      Ok(chunks) => {
        for digits in chunks {
          match u8::from_str_radix(digits, 16) {
            Ok(hex) => buffer.push(hex),
            Err(error) => {
              return Err(ParseHexError::ParseError(error, digits));
            }
          }
        }
      }
      Err(error) => return Err(ParseHexError::ChunkError(error)),
    }
  }

  Ok(buffer)
}

// ╔╦╗┌─┐┌─┐┌┬┐┌─┐
//  ║ ├┤ └─┐ │ └─┐
//  ╩ └─┘└─┘ ┴ └─┘

#[cfg(test)]
mod tests {
  use super::*;

  macro_rules! test_equality {
    ($name: ident, $string: literal, $($bytes: literal),+ $(,)?) => {
      #[test]
      fn $name() {
        assert_eq!(parse_hex($string).unwrap(), vec![$($bytes),+])
      }
    };
  }

  macro_rules! test_error {
    ($name: ident, $string: literal, $error: literal) => {
      #[test]
      fn $name() {
        // Extract error to string because ParseIntError is not buildable.
        assert_eq!(parse_hex($string).unwrap_err().to_string(), $error);
      }
    };
  }

  // https://stackoverflow.com/q/12063840
  // -  1 byte  (  8 bit):  byte, DB, RESB
  // -  2 bytes ( 16 bit):  word, DW, RESW
  // -  4 bytes ( 32 bit): dword, DD, RESD
  // -  8 bytes ( 64 bit): qword, DQ, RESQ
  // - 10 bytes ( 80 bit): tword, DT, REST
  // - 16 bytes (128 bit): oword, DO, RESO, DDQ, RESDQ
  // - 32 bytes (256 bit): yword, DY, RESY
  // - 64 bytes (512 bit): zword, DZ, RESZ

  test_equality!(one_byte, "11", 0x11);
  test_equality!(one_word, "1122", 0x11, 0x22);
  test_equality!(one_dword, "11223344", 0x11, 0x22, 0x33, 0x44);

  test_equality!(simple_quote, "'Hello", 0x48, 0x65, 0x6c, 0x6c, 0x6f);
  test_equality!(section_name, ".data", 0x2E, 0x64, 0x61, 0x74, 0x61);

  test_equality!(
    multilines,
    r"
    b612 1a 1b 'Hello 'XY ; Short comment
    FF   00   616263      ; This is a long comment
    ; Empty line

    .data ''
    ",
    0xB6,
    0x12,
    0x1A,
    0x1B,
    0x48, // H
    0x65, // e
    0x6c, // l
    0x6c, // l
    0x6f, // o
    0x58, // X
    0x59, // Y
    0xFF,
    0x00,
    0x61, // a
    0x62, // b
    0x63, // c
    0x2E, // .
    0x64, // d
    0x61, // a
    0x74, // t
    0x61, // a
    0x27, // '
  );

  test_error!(
    invalid_word_length,
    "1234 ABCD 'Hello 'World! 5 6 7 8",
    "Word length must be even (\"5\")."
  );

  test_error!(
    invalid_chunk_size,
    "'Elf 'Dwarf .bss 123 ; This is a comment.",
    "Word length must be a multiple of 2 (\"123\")."
  );

  test_error!(
    parse_int_error,
    "DEADBEEF IS GREAT",
    "Invalid digit found in string (\"IS\")."
  );
}
