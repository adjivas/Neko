// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/Neko
//
// This file may not be copied, modified, or distributed
// except according to those terms.

pub mod part;
mod err;

use std::collections::HashMap;

pub use self::err::{InventoryError, Result};
use self::part::Part;

struct Inventory(HashMap<String, Part>);

impl Inventory {
    pub fn insert(
        &mut self,
        limb: &str,
        name: &str,
        glyph: char
    ) -> Result<()> {
        match Part::new(limb, glyph) {
            Ok(part) => {
                if self.0.insert(String::from(limb) + name, part).is_none() {
                    Ok(())
                }
                else {
                    Err(InventoryError::Duplicate)
                }
            },
            Err(why) => Err(InventoryError::BadPart(why)),
        }
    }
}
