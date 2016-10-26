pub mod err;

pub use self::err::{PartError, Result};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Part {
  EyeLeft,
  EyeRight,
  None,
}

impl Part {
  pub fn new(part: &str) -> Result<Self> {
    match part {
      "eyeLeft"  => Ok(Part::EyeLeft),
      "eyeRight" => Ok(Part::EyeRight),
      "none" => Ok(Part::None),
               _ => Err(PartError::UnknownPart),
    }
  }
}
