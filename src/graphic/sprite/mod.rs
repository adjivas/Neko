pub mod texel;
pub mod err;

use std::fmt;

pub use self::err::{SpriteError, Result};
use self::texel::Texel;
use super::position::{Position, PositionError};
use super::emotion::{Emotion, EmotionError};

pub const SPEC_MAX_X: usize       = 7;
pub const SPEC_MAX_Y: usize       = 10;
pub const SPEC_MAX_XY: usize      = 70;
const SPEC_CAPACITY_SHEET: usize  = 5;

/// Position is like the Posture of the drawned persona.
#[derive(Copy)]
pub struct Draw(Position, [(Emotion, Texel); SPEC_MAX_XY]);

impl fmt::Debug for Draw {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "(Position: {:?}, sprite: {:?})", self.0,
      self.1.iter().collect::<Vec<&(Emotion, Texel)>>()
    )
  }
}

impl Clone for Draw {
    fn clone(&self) -> Draw { *self }
}


#[derive(Clone, Debug)]
pub struct Sprite {
  sheet: Vec<Draw>,
}

impl Sprite {
  fn insert(
    &mut self,
    draw: Draw,
  ) {
    self.sheet.push(draw);
  }

  /// The function `insert_from_expression` insert a draw
  /// from `cns file` expression.
  fn insert_from_expression(&mut self, content: &'static str) -> Result<()> {
    unimplemented!();
  }
}

impl Default for Sprite {
  fn default() -> Sprite {
    Sprite {
      sheet: Vec::with_capacity(SPEC_CAPACITY_SHEET),
    }
  }
}
