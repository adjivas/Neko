pub mod err;

pub use self::err::{PartError, Result};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Part {
  EyeLeft,
}

impl Part {
  pub fn new(part: &'static str) -> Result<Self> {
    match part {
      "EyeLeft" => Ok(Part::EyeLeft),
      _ => Err(PartError::UnknownPart),
    }
  }
}
