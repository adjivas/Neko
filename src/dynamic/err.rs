use std::error::Error;
use std::fmt;
use std::io;

use super::library::LibraryError;

pub type Result<T> = ::std::result::Result<T, CompositerError>;

/// The enum `CompositerError` defines the possible errors
/// from constructor Compositer.
#[derive(Debug)]
pub enum CompositerError {
  /// Can't fount the repository.
  LibraryNotFound,
  /// The command haven't a success return.
  CommandFail(i32),
  /// The command can't be run.
  CallCommandFail(io::Error),
  /// The directory can't be created.
  MkdirFail(io::Error),
  /// The directory can't be moved.
  MvFail(io::Error),
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
      CompositerError::CallCommandFail(_) => "The command can't be run.",
      CompositerError::CommandFail(_) => "The command haven't a success return.",
      CompositerError::MkdirFail(_) => "The directory can't be created.",
      CompositerError::MvFail(_) => "The directory can't be moved.",
      CompositerError::NotMounted(_) => "Can't mount the dynamic library.",
      CompositerError::LibraryNotFound => "The library wasn't found.",
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
      CompositerError::CallCommandFail(ref why) => Some(why),
      CompositerError::MkdirFail(ref why) => Some(why),
      CompositerError::MvFail(ref why) => Some(why),
      CompositerError::NotMounted(ref why) => Some(why),
      CompositerError::NotUninstalled(ref why) => Some(why),
      _ => None,
    }
  }
}
