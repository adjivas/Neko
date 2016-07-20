// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/Neko
//
// This file may not be copied, modified, or distributed
// except according to those terms.


extern crate std;

use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum PartError {
    UnknownPart,
    ForbiddenGlyph(char),
}

impl fmt::Display for PartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for PartError {
    fn description(&self) -> &str {
        match *self {
            PartError::UnknownPart => "the name of part is unknown",
            PartError::ForbiddenGlyph(_) => "the glyph is on a forbidden range",
        }
    }
}

