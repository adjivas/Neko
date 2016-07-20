// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/Neko
//
// This file may not be copied, modified, or distributed
// except according to those terms.

mod err;

pub use self::err::PartError;

pub type PartResult<T> = Result<T, PartError>;

pub enum Part {
    EyeLeft(char),
    EyeRight(char),
    EarLeft(char),
    EarRight(char),
    Nose(char),
    Mouth(char),
    Neck(char),
}

impl Part {
    pub fn new(limb: &str, glyph: char) -> PartResult<Self> {
        match limb {
            "EyeLeft" => Ok(Part::EyeLeft(glyph)),
            _ => Err(PartError::UnknownPart),
        }
    }
}

