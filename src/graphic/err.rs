use std::error::Error;
use std::fmt;
use std::io;

pub type Result<T> = ::std::result::Result<T, ManagerError>;

/// The enum `ManagerError` defines the possible errors
/// from constructor Manager.
#[derive(Debug)]
pub enum ManagerError {
    /// Can't read the `texel` sub-directory.
    ReadDirTexel(io::Error),
    /// Can't read the `sprite` sub-directory.
    ReadDirSprite(io::Error),
}

impl fmt::Display for ManagerError {
  /// The function `fmt` formats the value using
  /// the given formatter.
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
       Ok(())
    }
}

impl Error for ManagerError {
  /// The function `description` returns a short description of
  /// the error.
  fn description(&self) -> &str {
      match *self {
          ManagerError::ReadDirTexel(_) => "Can't read the `texel` sub-directory.",
          ManagerError::ReadDirSprite(_) => "Can't read the `sprite` sub-directory.",
      }
  }

  /// The function `cause` returns the lower-level cause of
  /// this error if any.
  fn cause(&self) -> Option<&Error> {
      match *self {
          ManagerError::ReadDirTexel(ref why) => Some(why),
          ManagerError::ReadDirSprite(ref why) => Some(why),
      }
  }
}
