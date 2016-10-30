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

#[macro_use]
extern crate itertools;
extern crate pty_proc;
extern crate dylib;
extern crate git2;
extern crate toml;

#[macro_use]
/// The macros of crate.
mod macros;
/// The module `prelude` is for public.
pub mod prelude;
/// The module `dynamic` is the compositer of extern libraries.
pub mod dynamic;
/// The module `graphic` is the manager of neko's sprites
pub mod graphic;

mod err;

use std::fmt;

pub use self::err::{NekoError, Result};
use pty_proc::shell::{Shell, ShellState};

use dynamic::Compositer;
use graphic::Manager;

/// The first directory.
const SPEC_ROOT: &'static str = ".neko";

/// The module `neko` is the first interface level.
pub struct Neko {
    dynamic: Compositer,
    graphic: Manager,
    shell: Shell,
}

impl Neko {
    pub fn new(
        repeat: Option<i64>,
        interval: Option<i64>,
    ) -> Result<Self> {
        match (
            Shell::from_mode(repeat, interval, None, pty_proc::shell::mode::Mode::Character),
            Compositer::new(),
            Manager::new()
        ) {
            (Err(why), _, _) => Err(NekoError::ShellFail(why)),
            (_, Err(why), _) => Err(NekoError::DynamicFail(why)),
            (_, _, Err(why)) => Err(NekoError::GraphicFail(why)),
            (Ok(shell), Ok(dynamic), Ok(graphic)) => Ok(Neko {
                dynamic: dynamic,
                graphic: graphic,
                shell: shell,
            }),
        }
    }
}

impl Iterator for Neko {
    type Item = ShellState;

    fn next(&mut self) -> Option<ShellState> {
        if let Some(next) = self.shell.next() {
            self.dynamic.call(&next);
            Some(next)
        } else {
            None
        }
    }
}

impl fmt::Debug for Neko {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?}", self.dynamic, self.graphic)
    }
}
