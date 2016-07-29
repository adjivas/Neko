// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/Neko
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_type= "lib"]
#![cfg_attr(feature = "nightly", feature(plugin))]
#![cfg_attr(feature = "lints", plugin(clippy))]
#![cfg_attr(feature = "lints", deny(warnings))]
#![cfg_attr(not(any(feature = "lints", feature = "nightly")), deny(unstable_features))]
#![deny(
        missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unused_import_braces,
        unused_qualifications)]

mod body;
