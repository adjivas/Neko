mod err;


use ::dylib;
pub use self::err::{LibraryError, Result};
use std::cmp::{Eq, Ordering};
use std::fmt;
use std::mem;
use std::path::PathBuf;

/// The struct `Library` is a table of callback.
pub struct Library {
  /// `start` interface.
  start: Option<fn()>,
  /// Address of the library.
  path: PathBuf,
  /// dynamic library interface.
  dylib: dylib::DynamicLibrary,
  /// priority queue.
  index: usize,
}

impl Library {
  /// The constructor method `new` returns a interface for a extern library.
  pub fn new(path: PathBuf, index: usize) -> Result<Self> {
    match dylib::DynamicLibrary::open(Some(&path)) {
      Err(why) => Err(LibraryError::BadDyLib(why)),
      Ok(lib) => unsafe {
        Ok(Library {
          start: if let Some(start) = lib.symbol::<*mut u8>("start")
            .ok() {
            Some(mem::transmute::<*mut *mut u8, fn()>(start))
          } else {
            None
          },
          path: path,
          dylib: lib,
          index: index,
        })
      },
    }
  }

  /// The accessor method `as_path_buf` return address of library.
  pub fn as_path_buf(&self) -> &PathBuf {
    &self.path
  }

  /// The method `start` call the extern function if defined.
  pub fn start(&self) {
    if let Some(start) = self.start {
      start();
    }
  }
}

/// Trait for equality comparisons which are equivalence relations.
impl Eq for Library {}

/// Trait for equality comparisons which are partial equivalence relations.
impl PartialEq for Library {
  /// This method tests for `self` and `other` values to be equal, and
  /// is used by `eq`.
  fn eq(&self, rhs: &Library) -> bool {
    self.index.eq(&rhs.index)
  }
}

/// Trait for values that can be compared for a sort-order.
impl PartialOrd for Library {
  /// This method returns an `Ordering` between `self` and `other` values
  /// if one exists.
  fn partial_cmp(&self, rhs: &Library) -> Option<Ordering> {
    self.index.partial_cmp(&rhs.index)
  }
}

/// Trait for equality comparisons which are equivalence relations.
impl Ord for Library {
  /// This method returns an `Ordering` between `self` and `other`.
  fn cmp(&self, rhs: &Library) -> Ordering {
    self.index.cmp(&rhs.index)
  }
}

/// Format trait for the `?` character.
impl fmt::Debug for Library {
  /// Formats the value using the given formatter.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "library({}): start:{}", self.index, self.start.is_some())
  }
}
