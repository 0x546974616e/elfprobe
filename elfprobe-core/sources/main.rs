// #![feature(trait_alias)]
// #![feature(doc_cfg)]

// https://users.rust-lang.org/t/how-to-fail-a-build-with-warnings/2687/10
// https://rust-unofficial.github.io/patterns/anti_patterns/deny-warnings.html
// https://doc.rust-lang.org/rustc/lints/index.html
// #![cfg_attr(not(debug_assertions), deny(missing_docs))]
// #![cfg_attr(not(debug_assertions), deny(warnings))]

// https://man7.org/linux/man-pages/man2/posix_fadvise.2.html

/*
mmap + MAP_LOCKED + MAP_POPULATE

Possibly https://www.man7.org/linux/man-pages/man2/madvise.2.html

hugepages? No

- locked + pop will will suck in more than he needs and slow the start down.
- Totally, it's a tradeoff if he's planning on hitting it many times. If the goal is to just load it up fast as possible definitely do not use those.


-----------------------------------------------------------------------------

MAP_PRIVATE + RO + mlock



*/

/*
https://github.com/NixOS/patchelf/blob/master/src/elf.h
https://github.com/bminor/binutils-gdb/blob/master/elfcpp/elfcpp.h

Linear Sweep approach

libopcodes
- https://github.com/guedou/binutils-rs/blob/master/src/opcodes.rs
- https://sourceware.org/cgit/binutils/tree/

libbfd

*/

// IPO / LTO ?

/*
Identical Comdat Folding (ICF)
https://devblogs.microsoft.com/oldnewthing/20161024-00/?p=94575
https://stackoverflow.com/questions/57378828/whats-the-difference-between-weak-linkage-vs-using-a-comdat-section
https://maskray.me/blog/2021-07-25-comdat-and-section-group
https://maskray.me/blog/2021-01-31-metadata-sections-comdat-and-shf-link-order
*/

// e_shnum contains the total number of sections in an object file.
// Since it is a 16-bit integer field, it's not large enough to
// represent >65535 sections. If an object file contains more than 65535
// sections, the actual number is stored to sh_size field.

// https://rust-lang.github.io/unsafe-code-guidelines/layout/function-pointers.html

// Auxiliary version information. https://tidelabs.github.io/tidechain/src/object/elf.rs.html#1738
// https://crates.io/crates/object

// https://github.com/robgjansen/elf-loader/tree/master

// pub(crate) ?

// od -An -t x1 -j 4 -N 1 $(which ls) | tr -d [[:space:]]

use std::env;
use std::fs::File;

mod core;
mod elf;
mod utils;

// #[cfg(any(test, doc, clippy))]
// mod hex;

use std::io;

#[allow(unused)]
fn test_file() -> io::Result<()> {
  use std::io::IsTerminal;
  // assert!(1 == 0, "dada {}", "fafa");

  let path: String = env::args()
    .nth(1)
    .expect("supply a single path as the program argument");

  println!("{:?}", path);

  // let file = File::open(path).expect("failed to open the file");
  let file = File::options().read(true).open(path)?;
  // .expect("failed to open the file");

  let metadata = file.metadata()?;
  println!("is_file = {:?}", metadata.is_file());
  println!("is_symlink = {:?}", metadata.is_symlink());
  println!("is_terminal = {:?}", file.is_terminal());

  Ok(())
}

fn main() {
  use crate::utils::MappedFile;
  use crate::*;
  use std::path::Path;

  // test_file();
  // return;

  let path: String = env::args().nth(1).expect("File missing");
  let path: &Path = path.as_ref();

  // let mmap = <MappedFile as TryFrom<&Path>>::try_from(path.as_ref());
  let mmap = MappedFile::try_from(path).expect("expect MappedFile");
  let slice = mmap.as_ref();

  // println!("{:#04X?}", &slice[0..4]);

  use crate::elf::parse_elf;

  println!("{:#x?}", parse_elf(slice));

  // mmap.close().expect("MappedFile close");

  // println!("{:x?}", data); // lower case
  // println!("{:X?}", data); // upper case
  // println!("{:02X?}", data); // print the leading zero
  // println!("{:#04X?}", data); // pretty modifier
}

// Read a usize value from a byte buffer:
// use std::mem;

/* use r#type::ElfType64;
fn read_usize(x: &[u8]) -> usize {
  assert!(x.len() >= mem::size_of::<usize>());
  let ptr = x.as_ptr() as *const usize;
  unsafe { ptr.read_unaligned() }
}
 */
