pub mod texel;
pub mod err;

pub use self::err::{SpriteError, Result};
use self::texel::Texel;
use super::position::{Position, PositionError};
use super::emotion::{Emotion, EmotionError};

pub const SPEC_MAX_X: usize = 7;
pub const SPEC_MAX_Y: usize = 10;
const SPEC_CAPACITY_SHEET: usize = 70;

pub type Draw = (Position, Vec<(Emotion, Texel)>);

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
  /// from `cns` expression.
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
