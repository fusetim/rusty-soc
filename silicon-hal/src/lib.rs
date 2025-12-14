#![no_std]

pub use silicon_pac as pac;

pub mod dac;
pub mod delay;
pub mod display;
pub mod gpio;
pub mod spi;
pub mod timer;
pub mod typesafe;

#[cfg(feature = "rt")]
pub use riscv_rt::entry;

pub struct Peripheral {
    pub gpio: gpio::Gpio,
    pub spi0: spi::Spi0,
    pub dac: dac::AudioDac,
}

pub fn init() -> Peripheral {
    Peripheral {
        gpio: gpio::Gpio::new(),
        spi0: spi::Spi0::new(),
        dac: dac::AudioDac::new(),
    }
}
