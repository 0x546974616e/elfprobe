use libc; // extern crate libc ??

use std::fs::File;
use std::ops::Deref;
use std::os::fd::{AsRawFd, RawFd};
use std::path::Path;
use std::{io, ptr, slice};

// https://github.com/danburkert/memmap-rs
// https://github.com/RazrFalcon/memmap2-rs/tree/master

///
/// Create a read-only memory-mapped file.
///
/// # Develope notes
///
/// Mutable references `&mut T` are not `Copy` hence the automatic
/// [reborrowing][reborrowing] `$*` when [inlay hints][inlay_hints] are
/// enabled.
///
/// [reborrowing]: https://haibane-tenshi.github.io/rust-reborrowing/
/// [inlay_hints]: https://rust-analyzer.github.io/manual.html#inlay-hints
///
// TODO: Better error message.
pub struct MappedFile {
  data: *const libc::c_void,
  length: libc::size_t,
}

// ╔═╗┬─┐┌─┐┌┬┐
// ╠╣ ├┬┘│ ││││
// ╚  ┴└─└─┘┴ ┴

impl TryFrom<&Path> for MappedFile {
  // There is a blanket implementation which prevent from doing:
  // impl<P: AsRef<Path>> TryFrom<P> for MappedFile { ... }
  type Error = io::Error;

  fn try_from(path: &Path) -> io::Result<Self> {
    // TODO:
    // Does `File` performs extra unnecessary operations and it is preferable to
    // use the native `open()` function instead to get a file descriptor?

    // According to the mmap(2) manual:
    // After the mmap() call has returned, the file descriptor, fd, can be
    // closed immediately without invalidating the mapping.
    MappedFile::try_from(&File::options().read(true).open(path)?)
  }
}

impl TryFrom<&File> for MappedFile {
  type Error = io::Error;

  fn try_from(file: &File) -> io::Result<Self> {
    // TODO:
    // is_file() but breaks with process substitution "<()" (in other words
    // with character and block device?), is !is_dir() enough?.
    match file.metadata()?.len().try_into() {
      // TryInto::<libc::size_t>::try_into(length)
      Err(error) => Err(io::Error::new(io::ErrorKind::InvalidData, error)),
      Ok(length) => MappedFile::new(file.as_raw_fd(), length),
    }
  }
}

// ╔═╗┬  ┬┌─┐┌─┐
// ╚═╗│  ││  ├┤
// ╚═╝┴─┘┴└─┘└─┘

impl Deref for MappedFile {
  type Target = [u8];

  #[inline]
  #[allow(clippy::needless_lifetimes)]
  // The lifetime is optional here but acts as a reminder that the output slice
  // must not outlive the mapped file.
  fn deref<'data>(&'data self) -> &'data [u8] {
    unsafe { slice::from_raw_parts(self.data as *const u8, self.length) }
  }
}

impl AsRef<[u8]> for MappedFile {
  #[inline]
  #[allow(clippy::needless_lifetimes)]
  // Same as Deref, the lifetime acts as a reminder for the developer.
  fn as_ref<'data>(&'data self) -> &'data [u8] {
    self.deref()
  }
}

// ╔╦╗┌─┐┌┬┐┬ ┬┌─┐┌┬┐
// ║║║├┤  │ ├─┤│ │ ││
// ╩ ╩└─┘ ┴ ┴ ┴└─┘╶┴┘

impl MappedFile {
  fn new(fd: RawFd, length: libc::size_t) -> io::Result<Self> {
    if length == 0 {
      return Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Memory map must have a non-zero length",
      ));
    }

    let data = unsafe {
      libc::mmap(
        // Let the kernel choose the mapping address.
        ptr::null_mut(),
        // Must be greater than zero and not not necessarily page-aligned.
        length,
        // Protection read-only.
        libc::PROT_READ,
        // Create a COW mapping. I am used to seeing PROT_READ and MAP_PRIVATE
        // together but is it really relevant? A COW is useless when it is
        // read-only, does it allocate additional resources? Or is it useful so
        // that changes made to the original file are not applied to the mapped
        // region? Although the mmap(2) manual explicitly defines this behavior
        // as unspecified.
        libc::MAP_PRIVATE,
        // Existing file descriptor otherwise EBADF.
        fd,
        // Start at the beginning of the file.
        0 as libc::off_t,
      )
    };

    if data == libc::MAP_FAILED {
      return Err(io::Error::last_os_error());
    }

    Ok(Self { data, length })
  }
}

// ╔╦╗┬─┐┌─┐┌─┐
//  ║║├┬┘│ │├─┘
// ═╩╝┴└─└─┘┴

impl Drop for MappedFile {
  fn drop(&mut self) {
    if !self.data.is_null() && self.length != 0 {
      let result = unsafe {
        libc::munmap(
          // Address must be page-aligned.
          // See libc::sysconf(libc::_SC_PAGESIZE).
          self.data.cast_mut(),
          self.length,
        )
      };

      if result == -1 {
        // We could also use panicking() and panic!().
        eprintln!("{:?}", io::Error::last_os_error());
      }
    }
  }
}
