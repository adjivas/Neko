use std::error::Error;
use std::fmt;

pub type Result<T> = ::std::result::Result<T, EmotionError>;


/// The enum `EmotionError` defines the possible errors
/// from constructor Emotion.
#[derive(Clone, Copy, Debug)]
pub enum EmotionError {
  UnknownEmotion,
}

impl fmt::Display for EmotionError {
  /// The function `fmt` formats the value using
  /// the given formatter.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl Error for EmotionError {
  /// The function `description` returns a short description of
  /// the error.
  fn description(&self) -> &str {
    match *self {
      EmotionError::UnknownEmotion => "The emotion value is unknown.",
    }
  }

  /// The function `cause` returns the lower-level cause of
  /// this error if any.
  fn cause(&self) -> Option<&Error> {
    None
  }
}
