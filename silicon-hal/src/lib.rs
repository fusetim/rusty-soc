#![no_std]

pub use silicon_pac as pac;

pub mod dac;
pub mod delay;
pub mod gpio;
pub mod timer;
pub mod typesafe;

pub struct Peripheral {
    pub gpio: gpio::Gpio,
}

pub fn init() -> Peripheral {
    Peripheral {
        gpio: gpio::Gpio::new(),
    }
}
