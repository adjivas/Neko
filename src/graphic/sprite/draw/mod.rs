mod err;


pub use self::err::{DrawError, Result};
use std::fmt;
use std::mem;

pub const SPEC_MAX_X: usize = 7;
pub const SPEC_MAX_Y: usize = 10;
pub const SPEC_MAX_XY: usize = SPEC_MAX_X * SPEC_MAX_Y;

pub use super::{Emotion, EmotionError};
pub use super::{Position, PositionError};
pub use super::Texel;

/// Position is like the Posture of the drawned persona.
#[derive(Copy)]
pub struct Draw(Position, [(Emotion, Texel); SPEC_MAX_XY]);    //tuple structs

impl Draw {
  pub fn new(position: Position, buf: &[(Emotion, Texel)]) -> Result<Self> {
    if SPEC_MAX_XY.eq(&buf.len()) {
      unsafe {
        let mut line: [(Emotion, Texel); SPEC_MAX_XY] = mem::uninitialized();

        line.copy_from_slice(buf);
        Ok(Draw(position, line))
      }
    } else {
      Err(DrawError::OutOfSize)
    }
  }
}

impl fmt::Debug for Draw {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,
           "(Position: {:?}, sprite: {:?})",
           self.0,
           self.1.iter().collect::<Vec<&(Emotion, Texel)>>())
  }
}

impl Clone for Draw {
  fn clone(&self) -> Draw {
    *self
  }
}
