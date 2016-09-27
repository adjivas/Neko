mod sprite;
mod emotion;
mod posture;

use self::sprite::{Sprite, SpriteError};
use self::sprite::texel::{Texel, TexelError};
use self::posture::{Posture, PostureError};
use self::emotion::{Emotion, EmotionError};

use std::collections::HashMap;
use std::ffi::OsStr;

/// The default capacity of texel dictionary.
const SPEC_CAPACITY_TEXEL: usize = 4095;
/// The default capacity of sprite dictionary.
const SPEC_CAPACITY_SPRITE: usize = 1024;

pub type Key = (Posture, Emotion);

#[derive(Clone, Debug)]
pub struct Manager {
  /// Dictionary of texel.
  texel: HashMap<Key, Texel>,
  /// Dictionary of sprite.
  sprite: HashMap<String, Sprite>,
}

impl Manager {

  /// The function `insert_texel` insert a texel.
  fn insert_texel(
    &mut self,
    key: Key,
    val: Texel,
  ) -> Option<Texel> {
    self.texel.insert(key, val)
  }

  /// The function `insert_sprite` insert a sprite.
  fn insert_sprite(
    &mut self,
    key : String,
    val: Sprite,
  ) -> Option<Sprite> {
    self.sprite.insert(key, val)
  }

  /// The function `from_file_texel` insert a texel from a file.
  pub fn insert_from_texelfile<S: AsRef<OsStr> >(
    &mut self,
    filename: S,
  ) {
    unimplemented!()
  }

  /// The function `from_file_sprite` insert a sprite from a file.
  pub fn insert_from_spritefile<S: AsRef<OsStr> >(
    &mut self,
    filename: S,
  ) {
    unimplemented!()
  }
}

/// A trait for giving a type a useful default value.
impl Default for Manager {

  /// The constructor `default` returns a empty Manager.
  fn default() -> Manager {
    Manager {
      texel: HashMap::with_capacity(SPEC_CAPACITY_TEXEL),
      sprite: HashMap::with_capacity(SPEC_CAPACITY_SPRITE),
    }
  }
}
