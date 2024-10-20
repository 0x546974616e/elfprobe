mod adapter;
mod constant;
mod file;
mod flags;
mod hex;
mod table;

#[cfg(any(test, doc, clippy))]
pub use hex::parse_hex;

pub(crate) use constant::define_constants;
pub(crate) use flags::define_flags;
pub(crate) use table::display_table;
pub(crate) use table::display_row;

pub use adapter::{Bytes, FileOffset, Hex, Magic};
pub use constant::Constant;
pub use file::MappedFile;
pub use table::DisplayTable;
