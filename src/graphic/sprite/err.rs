use std::error::Error;
use std::fmt;
use std::io;

pub type Result<T> = ::std::result::Result<T, SpriteError>;


/// The enum `SpriteError` defines the possible errors
/// from constructor Sprite.
#[derive(Clone, Copy, Debug)]
pub enum SpriteError {
}

impl fmt::Display for SpriteError {

  /// The function `fmt` formats the value using
  /// the given formatter.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl Error for SpriteError {
  /// The function `description` returns a short description of
  /// the error.
  fn description(&self) -> &str {
    match *self {
    }
  }

  /// The function `cause` returns the lower-level cause of
  /// this error if any.
  fn cause(&self) -> Option<&Error> {
    None
  }
}
