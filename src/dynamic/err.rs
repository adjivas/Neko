use std::error::Error;
use std::fmt;

use super::library::LibraryError;

pub type Result<T> = ::std::result::Result<T, CompositerError>;

/// The enum `CompositerError` defines the possible errors
/// from constructor Compositer.
#[derive(Debug)]
pub enum CompositerError {
    /// Can't mount the dynamic library.
    BadMount(LibraryError),
    /// Path not exist.
    BadPath,
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
            CompositerError::BadMount(_) => "Can't mount the dynamic library",
            CompositerError::BadPath => "Path not exist",
        }
    }

    /// The function `cause` returns the lower-level cause of
    /// this error if any.
    fn cause(&self) -> Option<&Error> {
        match *self {
            CompositerError::BadMount(ref why) => Some(why),
            _ => None,
        }
    }
}
