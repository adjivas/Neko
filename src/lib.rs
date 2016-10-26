// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/Neko
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//! # neko
//!
//! This library contains the module `graphic` and `dynamic`.

#![feature(slice_patterns)]
#![feature(advanced_slice_patterns)]

#![crate_type= "lib"]
#![cfg_attr(feature = "nightly", feature(plugin))]
#![cfg_attr(feature = "lints", plugin(clippy))]
#![cfg_attr(feature = "lints", deny(warnings))]
#![cfg_attr(not(any(feature = "lints", feature = "nightly")), deny())]
#![deny(
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unused_import_braces,
        unused_qualifications
)]

#![doc(html_logo_url = "https://arukana.github.io/Neko/images/neko.png")]

extern crate dylib;
extern crate git2;
extern crate toml;

#[macro_use]
/// The macros of crate.
mod macros;
/// The module `dynamic` is the compositer of extern libraries.
pub mod dynamic;
/// The module `graphic` is the manager of neko's sprites
pub mod graphic;
/// The module `prelude` is for public.
pub mod prelude;
