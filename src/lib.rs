// Trying to emulate a chip-8 computer!
extern crate rand;
extern crate sdl2;
#[macro_use]
extern crate log;
extern crate env_logger;


pub mod boot;
pub mod vm;
pub mod instructions;
pub mod keypad;
pub mod gfx;

/// Returns the version of this crate in the format `MAJOR.MINOR.PATCH`.
pub fn version() -> &'static str {
    concat!(
        env!("CARGO_PKG_VERSION_MAJOR"), ".",
        env!("CARGO_PKG_VERSION_MINOR"), ".",
        env!("CARGO_PKG_VERSION_PATCH"),
    )
}
