#![no_std]

use crate::peripherals::Peripherals;

pub(crate) mod config;
pub(crate) mod peripherals;
pub mod led;

pub fn init() -> Peripherals {
    Peripherals {
        leds: led::LedPeripherals::new(),
    }
} 