use std::error::Error;
use std::fmt;
use std::io;

pub type Result<T> = ::std::result::Result<T, ManagerError>;

/// The enum `ManagerError` defines the possible errors
/// from constructor Manager.
#[derive(Debug)]
pub enum ManagerError {
    /// Can't read the sub-directory.
    ReadDir(io::Error),
    /// Can't create the texel sub-directory.
    MkDirTexel(io::Error),
    /// Can't create the sprite sub-directory.
    MkDirSprite(io::Error),
    /// Can't found the $HOME environement variable.
    Home,
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
          ManagerError::ReadDir(_) => "Can't read the sub-directory.",
          ManagerError::MkDirTexel(_) => "Can't create the texel sub-directory.",
          ManagerError::MkDirSprite(_) => "Can't create the sprite sub-directory.",
          ManagerError::Home => "Can't found the $HOME environement variable.",
      }
  }

  /// The function `cause` returns the lower-level cause of
  /// this error if any.
  fn cause(&self) -> Option<&Error> {
      match *self {
          ManagerError::ReadDir(ref why) |
          ManagerError::MkDirTexel(ref why) |
          ManagerError::MkDirSprite(ref why) => Some(why),
          _ => None,
      }
  }
}
