use std::error::Error;
use std::fmt;

use super::part::PartError;

pub type Result<T> = ::std::result::Result<T, TexelError>;

/// The enum `TexelError` defines the possible errors
/// from constructor Texel.
#[derive(Clone, Copy, Debug)]
pub enum TexelError {
  PartFail(PartError),
  ForbiddenGlyph(u32),
}

impl fmt::Display for TexelError {
  /// The function `fmt` formats the value using
  /// the given formatter.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl Error for TexelError {
  /// The function `description` returns a short description of
  /// the error.
  fn description(&self) -> &str {
    match *self {
      TexelError::PartFail(_) => "The part constructor as occured an error.",
      TexelError::ForbiddenGlyph(_) => "Is out of the private unicode range.",
    }
  }

  /// The function `cause` returns the lower-level cause of
  /// this error if any.
  fn cause(&self) -> Option<&Error> {
    match *self {
      TexelError::PartFail(ref err) => Some(err),
      _ => None,
    }
  }
}
