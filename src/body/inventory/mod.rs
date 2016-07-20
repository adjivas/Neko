// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/Neko
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

pub mod part;
mod err;

use std::collections::HashMap;

use self::part::{Part, PartError};
pub use self::err::InventoryError;

pub type InventoryResult<T> = Result<T, InventoryError>;

struct Inventory(HashMap<String, Part>);

impl Inventory {
    pub fn insert(
        &mut self,
        limb: &str,
        name: &str,
        glyph: char
    ) -> InventoryResult<()> {
        match Part::new(limb, glyph) {
            Ok(part) => {
                self.0.insert(String::from(limb) + name, part);
                Ok(())
            },
            Err(why) => Err(InventoryError::BadPart(why)),
        }
    }
}
