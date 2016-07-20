// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/Neko
//
// This file may not be copied, modified, or distributed
// except according to those terms.

mod inventory;

use self::inventory::part::Part;

const MAX_X: usize = 7;
const MAX_Y: usize = 10;

struct Body([[Part; MAX_X]; MAX_Y]);
