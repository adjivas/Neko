use std::error::Error;
use std::fmt;
use std::io;

use super::library::LibraryError;

pub type Result<T> = ::std::result::Result<T, CompositerError>;

/// The enum `CompositerError` defines the possible errors
/// from constructor Compositer.
#[derive(Debug)]
pub enum CompositerError {
  /// Can't clone the repository.
  BadPath,
  /// The command can't be run.
  BadCommand(io::Error),
  /// The command haven't a success return.
  BadReturnCommand(i32),
  /// Can't mount the dynamic library.
  NotMounted(LibraryError),
  /// The Makefile isn't accessible.
  NotMakeFound,
  /// The library wasn't found.
  NotUnmounted,
  /// The library can't be removed.
  NotUninstalled(io::Error),
  /// The repository can't be pulled.
  NotGitPull,
  /// The repository can't be cloned.
  NotGitClone,
  /// The repository can't be opened.
  NotGitOpen,
}

impl fmt::Display for CompositerError {
  /// The function `fmt` formats the value using
  /// the given formatter.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl Error for CompositerError {
  /// The function `description` returns a short description of
  /// the error.
  fn description(&self) -> &str {
    match *self {
      CompositerError::BadCommand(_) => "The command can't be run.",
      CompositerError::BadReturnCommand(_) => "The command haven't a success return.",
      CompositerError::NotMounted(_) => "Can't mount the dynamic library.",
      CompositerError::BadPath => "Path not exist.",
      CompositerError::NotMakeFound => "The Makefile isn't accessible.",
      CompositerError::NotUnmounted => "The library wasn't found.",
      CompositerError::NotUninstalled(_) => "The library can't be removed",
      CompositerError::NotGitPull => "The repository can't be pulled.",
      CompositerError::NotGitClone => "The repository can't be cloned.",
      CompositerError::NotGitOpen => "The repository can't be opened.",
    }
  }

  /// The function `cause` returns the lower-level cause of
  /// this error if any.
  fn cause(&self) -> Option<&Error> {
    match *self {
      CompositerError::NotMounted(ref why) => Some(why),
      CompositerError::BadCommand(ref why) => Some(why),
      CompositerError::NotUninstalled(ref why) => Some(why),
      _ => None,
    }
  }
}
