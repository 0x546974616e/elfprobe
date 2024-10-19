mod adapter;
mod constant;
mod file;
mod hex;
mod table;

#[cfg(any(test, doc, clippy))]
pub use hex::parse_hex;

pub(crate) use constant::define_constants;
pub(crate) use table::display_table;

pub use adapter::{Bytes, Hex, Magic, FileOffset};
pub use constant::Constant;
pub use file::MappedFile;
pub use table::DisplayTable;
