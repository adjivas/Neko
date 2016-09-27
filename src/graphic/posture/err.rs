use std::error::Error;
use std::fmt;
use std::io;

pub type Result<T> = ::std::result::Result<T, PostureError>;


/// The enum `PostureError` defines the possible errors
/// from constructor Posture.
#[derive(Clone, Copy, Debug)]
pub enum PostureError {
  UnknownPosture,
}

impl fmt::Display for PostureError {

  /// The function `fmt` formats the value using
  /// the given formatter.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl Error for PostureError {
  /// The function `description` returns a short description of
  /// the error.
  fn description(&self) -> &str {
    match *self {
      PostureError::UnknownPosture => "The posture value is unknown.",
    }
  }

  /// The function `cause` returns the lower-level cause of
  /// this error if any.
  fn cause(&self) -> Option<&Error> {
    None
  }
}
