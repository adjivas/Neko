pub mod err;

pub use self::err::{PostureError, Result};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Posture {
  LotusHandsOnFloor,
  LyingOnSomething,
  None,
}

impl Posture {
  pub fn new(content: &'static str) -> Result<Self> {
    match content {
      "LotusHandsOnFloor" => Ok(Posture::LotusHandsOnFloor),
      "LyingOnSomething" => Ok(Posture::LyingOnSomething),
      "None" => Ok(Posture::None),
      _ => unimplemented!()
    }
  }
}

impl Default for Posture {
  fn default() -> Posture {
    Posture::None
  }
}
