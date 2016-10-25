pub mod err;

pub use self::err::{EmotionError, Result};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Emotion {
  Happy,
  Malicious,
  None,
}

impl Emotion {
  pub fn new(content: &str) -> Result<Self> {
    match content {
      "Happy"     => Ok(Emotion::Happy),
      "Malicious" => Ok(Emotion::Malicious),
      "None"      => Ok(Emotion::None),
                _ => Err(EmotionError::UnknownEmotion)
    }
  }
}

impl Default for Emotion {
  fn default() -> Emotion {
    Emotion::None
  }
}
