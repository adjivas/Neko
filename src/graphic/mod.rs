mod sprite;
mod emotion;
mod position;

use self::sprite::{Sprite, SpriteError};
use self::sprite::texel::{Texel, TexelError};
use self::sprite::texel::part::{Part, PartError};
use self::position::{Position, PositionError};
use self::emotion::{Emotion, EmotionError};

use std::collections::HashMap;
use std::ffi::OsStr;
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
use std::io::prelude::*;
use std::io;
use std::ops::Not;

/// The default capacity of texel dictionary.
const SPEC_CAPACITY_TEXEL:  usize = 4095;
/// The default capacity of sprite dictionary.
const SPEC_CAPACITY_SPRITE: usize = 1024;

#[derive(Clone, Debug)]
pub struct Manager {
  /// Dictionary of texel.
  texel: HashMap<(Position, Part, Emotion), Texel>,
  /// Dictionary of sprite.
  sprite: HashMap<String, Sprite>, //String is the `Expression \n Representation of the sprite ibn the Dictionary of Sprite`
}


//     println!("{:?}", line.split(|c| "('): [,]".contains(c) ).filter(|x| !x.is_empty()).collect::<Vec<&str>>() );


impl Manager {

  /// The function `insert_texel` insert a texel.
  fn insert_texel(
    &mut self,
    key: (Position, Part, Emotion),
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

  /// (1) The function `from_file_texel` insert a texel from a file.
  pub fn insert_from_texelfile<S: AsRef<OsStr> >(
    &mut self,
    filename: S,
  ) {

    //open texel config file
    match fs::OpenOptions::new().read(true).open(filename.as_ref()) {
      Err(why) => panic!("couldn't create {:?}: {}",
                        filename.as_ref(),
                        why.description()),
      Ok(file) => {
        
        //get the sprite with parser from configfile to be tokenized
        let mut reader = io::BufReader::new(file).lines();     // iterator

        reader.all(|line: io::Result<String>| {
          if let Some(buf) = line.ok() {
            let tokens: Vec<&str> = buf.split(|c|
              "('): [,]".contains(c)
            ).filter(|x|
              x.is_empty().not()
            ).collect::<Vec<&str>>();
            match &tokens[..] {
              &[pt, character, emotion, ref position..] => {
                position.iter().all(|content|
                  if let (Some(position),
                          Some(part),
                          Some(emotion),
                          Some(glyph)) = (Position::new(content).ok(),
                                          Part::new(pt).ok(),
                                          Emotion::new(emotion).ok(),
                                          character.as_bytes().first()) {
                    if let Ok(texel) = Texel::new(pt, *glyph) {
                      self.insert_texel(
                        (position, part, emotion),
                        texel
                      );
                      true
                    } else {
                      false
                    }
                  } else {
                    false
                  }
                )
              },
              _ => false
            }
          } else {
            false
          }
        });
      },
    };
  }

  /// The function `from_file_sprite` insert a sprite from a file.
  pub fn insert_from_spritefile<S: AsRef<OsStr> >(
    &mut self,
    filename: S,
  ) {

    // TODO: pretty much the same as above but for sprite

    //open sprite config file
    //get the sprite with parser from configfile
    //push the relevant sprite to sprite hashmap
    unimplemented!()
  }

} // End of impl Manager

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
