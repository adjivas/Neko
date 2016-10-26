mod sprite;
mod emotion;
mod position;

use std::collections::VecDeque;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::io;
use std::ops::Not;
use std::mem;

use self::sprite::draw::{Draw, SPEC_MAX_XY};
use self::sprite::Sprite;
use self::sprite::texel::Texel;
use self::sprite::texel::part::Part;
use self::position::Position;
use self::emotion::Emotion;

/// The default capacity of texel dictionary.
const SPEC_CAPACITY_TEXEL:  usize = 4096;
/// The default capacity of sprite dictionary.
const SPEC_CAPACITY_SPRITE: usize = 1024;

#[derive(Clone, Debug)]
pub struct Manager {
  /// Dictionary of texel.
  texel: HashMap<(Position, Part, Emotion), Texel>,
  /// Dictionary of sprite.
  ///String is the `Expression \n Representation of the sprite ibn the Dictionary of Sprite`
  sprite: HashMap<String, Sprite>,
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
    key : String, //name of the the sprite config file
    val : Sprite,
  ) -> Option<Sprite> {
    self.sprite.insert(key, val)
  }

  /// The function `from_file_texel` insert a texel from a file.
  pub fn insert_from_texelfile<S: AsRef<OsStr> >(
    &mut self,
    filename: S,
  ) {
    match fs::OpenOptions::new().read(true).open(filename.as_ref()) {
      Err(why) => panic!("couldn't create {:?}: {}",
                        filename.as_ref(),
                        why.description()),
      Ok(buffer) => {
        let mut reader = io::BufReader::new(buffer).lines();
        reader.all(|line: io::Result<String>| {
          if let Some(line) = line.ok() {
            let words: Vec<&str> = line.split(|c|
              "('): [,]".contains(c)
            ).filter(|x|
              x.is_empty().not()
            ).collect::<Vec<&str>>();
            match &words[..] {
              &[pt, character, emotion, ref positions..] => {
                positions.iter().all(|content: &&str|
                  if let (Some(position),
                          Some(part),
                          Some(emotion),
                          glyph) = (Position::new(&content).ok(),
                                          Part::new(pt).ok(),
                                          Emotion::new(emotion).ok(),
                                          character.as_bytes()) {
                    if let Ok(texel) = Texel::new(pt, unsafe {
                      mem::transmute::<[u8; 4], u32>(
                        [glyph[0], glyph[1], glyph[2], glyph[3]]
                      )
                    }) {
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
  pub fn insert_from_spritefile(
    &mut self,
    filename: String,
  ) {
    let mut sprite: Sprite = Sprite::default();

    match fs::OpenOptions::new().read(true).open(&filename) {
      Err(why) => panic!("couldn't create {}: {}",
                        filename,
                        why.description()),
      Ok(mut file) => {
        let mut buffer = String::new();
        if file.read_to_string(&mut buffer).is_ok() {
          let mut words: VecDeque<&str> = buffer.split(|c|
            " \n:".contains(c)
          ).filter(|x|
            x.is_empty().not()
          ).collect::<VecDeque<&str>>();

          if let Some(position) = Position::new(words.pop_front().unwrap())
                                           .ok() {
            println!("{:?}", position );  //debug
            let mut potential_draw_chunks = words.as_slices().0.chunks(
              SPEC_MAX_XY*2
            );
            potential_draw_chunks.all(|chunck| {
              let pairs = chunck.chunks(2);

              /* filter_map vs match... Fight! */

              if let Ok(draw) = Draw::new(
                position,
                pairs.map(|pair: &[&str]| {
                  match pair {
                    &[part, emotion] => {
                      if let (Ok(part), Ok(emotion)) = (
                        Part::new(part),
                        Emotion::new(emotion)
                      ) {
                        if let Some(texel) = self.texel.get(&(position,
                          part,
                          emotion
                        )) {
                          Some((emotion, *texel))
                        } else {
                          None
                        }
                      } else {
                        None
                      }
                    },
                    _ => None,
                  }
                }).filter_map(|s| s)
                  .collect::<Vec<(Emotion, Texel)>>().as_slice()
              ) {
                sprite.insert(draw);
                true
              } else {
                false
              }
            });
            self.insert_sprite(filename, sprite);
          }
        }
      }
    }
  } // End fn insert_from_spritefile 

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
