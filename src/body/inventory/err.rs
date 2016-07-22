// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/Neko
//
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::error::Error;

use super::part::PartError;

pub type Result<T> = ::std::result::Result<T, InventoryError>;

#[derive(Debug)]
pub enum InventoryError {
    Duplicate,
    BadPart(PartError),
}

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for InventoryError {
    fn description(&self) -> &str {
        match *self {
            InventoryError::Duplicate => "Key already added",
            InventoryError::BadPart(_) => "There is something wrong with `Part`",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            InventoryError::Duplicate => None,
            InventoryError::BadPart(ref err) => Some(err),
        }
    }
}
