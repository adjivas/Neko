use std::error::Error;
use std::fmt;
use std::io;

pub type Result<T> = ::std::result::Result<T, LibraryError>;

/// The enum `LibraryError` defines the possible errors
/// from constructor Library.
#[derive(Debug)]
pub enum LibraryError {
    /// Can't found any methods.
    EmptyEvent,
    /// Can't get the symbole from dynamic library.
    BadGet(io::Error),
    /// DynamicLibrary as occured an error.
    BadDyLib(String),
}

impl fmt::Display for LibraryError {

   /// The function `fmt` formats the value using
   /// the given formatter.
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{}", self)
   }
}

impl Error for LibraryError {

    /// The function `description` returns a short description of
    /// the error.
    fn description(&self) -> &str {
        match *self {
            LibraryError::EmptyEvent => "Can't found any methods.",
            LibraryError::BadGet(_) => "Can't get the symbole from dy library.",
            LibraryError::BadDyLib(_) => "DynamicLibrary as occured an error.",
        }
    }

    /// The function `cause` returns the lower-level cause of
    /// this error if any.
    fn cause(&self) -> Option<&Error> {
        match *self {
            LibraryError::BadGet(ref why) => Some(why),
            _ => None,
        }
    }
}
