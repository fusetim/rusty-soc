#![no_std]

use crate::peripherals::Peripherals;

pub mod config;
pub(crate) mod peripherals;
pub mod gpio;
pub mod delay;

pub fn init() -> Peripherals {
    return Peripherals::new();
} 