pub mod err;

pub use self::err::{TexelError, Result};

#[derive(Clone, Copy, Debug)]
pub enum Texel {
  EyeLeft(u8),
}

impl Texel {
  fn new(part: &'static str, glyph: u8) -> Result<Self> {
    if let 57344 ... 63743 = glyph {
      match part {
        "EyeLeft" => Ok(Texel::EyeLeft(glyph)),
        _ => Err(TexelError::UnknownTexel),
      }
    } else {
      Err(TexelError::ForbiddenGlyph(glyph))
    }
  }
}
