use std::error::Error;
use std::fmt;

use ::dynamic::CompositerError;
use ::graphic::ManagerError;
use ::pty_proc::shell::ShellError;

pub type Result<T> = ::std::result::Result<T, NekoError>;

/// The enum `NekoError` defines the possible errors
/// from constructor Neko.
#[derive(Debug)]
pub enum NekoError {
    /// The dynamic library interface has occured an error.
    DynamicFail(CompositerError),
    /// The graphic interface has occured an error.
    GraphicFail(ManagerError),
    /// The shell interface has occured an error.
    ShellFail(ShellError),
}

impl fmt::Display for NekoError {
  /// The function `fmt` formats the value using
  /// the given formatter.
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
       Ok(())
    }
}

impl Error for NekoError {
  /// The function `description` returns a short description of
  /// the error.
  fn description(&self) -> &str {
      match *self {
          NekoError::DynamicFail(_) => "The dynamic library interface has\
                                        occured an error.",
          NekoError::GraphicFail(_) => "The graphic interface has\
                                        occured an error.",
          NekoError::ShellFail(_) => "The shell interface has occured an error",
    }
  }

  /// The function `cause` returns the lower-level cause of
  /// this error if any.
  fn cause(&self) -> Option<&Error> {
      match *self {
          NekoError::DynamicFail(ref why) => Some(why),
          NekoError::GraphicFail(ref why) => Some(why),
          NekoError::ShellFail(ref why) => Some(why),
      }
  }
}
