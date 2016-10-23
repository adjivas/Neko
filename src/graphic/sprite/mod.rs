pub mod texel;
pub mod draw;
mod err;

use std::fmt;

use self::draw::Draw;
pub use self::err::{SpriteError, Result};
pub use self::texel::Texel;
pub use super::position::{Position, PositionError};
pub use super::emotion::{Emotion, EmotionError};

const SPEC_CAPACITY_SHEET: usize  = 5;

#[derive(Clone, Debug)]
pub struct Sprite {
  sheet: Vec<Draw>,
}

impl Sprite {
  pub fn insert(
    &mut self,
    draw: Draw,
  ) {
    self.sheet.push(draw);
  }
}

impl Default for Sprite {
  fn default() -> Sprite {
    Sprite {
      sheet: Vec::with_capacity(SPEC_CAPACITY_SHEET),
    }
  }
}
