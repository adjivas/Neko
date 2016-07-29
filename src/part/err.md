```rust
use std::fmt;
use std::error::Error;

/// The alias `Result` learns `PartError` possibility.

pub type Result<T> = ::std::result::Result<T, PartError>;

/// The enum `PartError` defines the possible errors from constructor Part.

#[derive(Clone, Copy, Debug)]
pub enum PartError {
    /// If the part dosn't exist.
    UnknownPart,
    /// If the glyph isn't allowed by the constructor.
    ForbiddenGlyph(char),
}

impl fmt::Display for PartError {

    /// The function `fmt` formats the value using the given formatter.

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for PartError {

    /// The function `description` returns a short description of the error.

    fn description(&self) -> &str {
        match *self {
            PartError::UnknownPart => "The name of part is unknown",
            PartError::ForbiddenGlyph(_) => "The glyph is on a forbidden range",
        }
    }

    /// The function `cause` returns the lower-level cause of this error, if any.

    fn cause(&self) -> Option<&Error> {
        None
    }
}
```
