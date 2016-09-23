mod library;
pub mod err;

use std::collections::BinaryHeap;
use std::path::Path;
use std::usize;

use self::library::Library;
pub use self::err::{CompositerError, Result};

/// The default capacity of heap.
const SPEC_CAPACITY: usize = 10;
/// The default priority of call.
const SPEC_PRIORITY: usize = usize::MAX / 2;

/// The struct `Compositer` is a heap of a double tuple
/// of a dynamic libraries and a priority order.
#[derive(Debug)]
pub struct Compositer (BinaryHeap<Library>);

impl Compositer {

  /// The method `mount` adds a new library to the heap's compositer.
  pub fn mount (
    &mut self,
    libraryname: &str,
    priority: Option<usize>,
  ) -> Result<()> {
    let path: &Path = Path::new(libraryname);

    if path.exists() {
      match Library::new(path, priority.unwrap_or(SPEC_PRIORITY)) {
        Err(why) => Err(CompositerError::BadMount(why)),
        Ok(lib) => Ok(self.0.push(lib)),
      }
    } else {
      Err(CompositerError::BadPath)
    }
  }

  pub fn start(&self) {
    self.0.iter().all(|lib| {
        lib.start();
        true
    });
  }
}

/// A trait for giving a type a useful default value.
impl Default for Compositer {

  /// The constructor `default` returns a empty Compositer.
  fn default() -> Compositer {
    Compositer(BinaryHeap::with_capacity(SPEC_CAPACITY))
  }
}
