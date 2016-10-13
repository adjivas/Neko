pub mod err;
pub mod part;

use std::fmt;

use self::part::Part;
pub use self::err::{TexelError, Result};

#[derive(Clone, Copy, Debug)]
pub struct Texel(Part, u8);

impl Texel {
  pub fn new(part: &'static str, glyph: u8) -> Result<Self> {
    if let 57344 ... 63743 = glyph {
      match Part::new(part) {
        Ok(part) => Ok(Texel(part, glyph)),
        Err(why) => Err(TexelError::PartFail(why)),
      }
    } else {
      Err(TexelError::ForbiddenGlyph(glyph))
    }
  }
  
  pub fn as_part(&self) -> Part {
    self.0
  }
}

impl PartialEq for Texel {
  fn eq(&self, rhs: &Texel) -> bool {
    self.0.eq(&rhs.0)
  }
}

impl fmt::Display for Texel {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.1)
  }
}
