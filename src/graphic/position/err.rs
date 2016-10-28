use std::error::Error;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, PositionError>;


/// The enum `PositionError` defines the possible errors
/// from constructor Position.
#[derive(Clone, Copy, Debug)]
pub enum PositionError {
  UnknownPosition,
}

impl fmt::Display for PositionError {
  /// The function `fmt` formats the value using
  /// the given formatter.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl Error for PositionError {
  /// The function `description` returns a short description of
  /// the error.
  fn description(&self) -> &str {
    match *self {
      PositionError::UnknownPosition => "The posture value is unknown.",
    }
  }

  /// The function `cause` returns the lower-level cause of
  /// this error if any.
  fn cause(&self) -> Option<&Error> {
    None
  }
}
