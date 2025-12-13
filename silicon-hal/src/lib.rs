#![no_std]

pub use silicon_pac as pac;

pub mod dac;
pub mod delay;
pub mod gpio;
pub mod spi;
pub mod timer;
pub mod typesafe;

pub struct Peripheral {
    pub gpio: gpio::Gpio,
    pub spi0: spi::Spi0,
}

pub fn init() -> Peripheral {
    Peripheral {
        gpio: gpio::Gpio::new(),
        spi0: spi::Spi0::new(),
    }
}
