//! Mehr Informationen über die Termion-Bibliothek https://github.com/ticki/termion

#![warn(missing_docs)]

#[cfg(not(target_os = "redox"))]
extern crate libc;

#[cfg(not(target_os = "redox"))]
mod termios;

mod async;
pub use async::{AsyncReader, async_stdin};

mod size;
pub use size::terminal_size;

mod tty;
pub use tty::is_tty;

#[macro_use]
mod macros;
pub mod clear;
pub mod color;
pub mod cursor;
pub mod event;
pub mod input;
pub mod raw;
pub mod scroll;
pub mod style;
