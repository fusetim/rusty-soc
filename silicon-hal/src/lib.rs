#![no_std]

use crate::peripherals::Peripherals;

pub mod config;
pub mod dac;
pub mod delay;
pub mod fb;
pub mod gpio;
pub(crate) mod peripherals;

pub fn init() -> Peripherals {
    return Peripherals::new();
}
