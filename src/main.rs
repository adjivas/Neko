// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/Neko
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate neko;

use neko::part::{Part, PartError};

fn main() {
  let _: Result<Part, PartError> = Part::new("EyeLeft", '0');
}
