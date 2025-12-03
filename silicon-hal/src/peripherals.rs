use crate::{delay::SocDelay, gpio::GpioPeripheral};

/// Peripherals
#[derive(Debug)]
pub struct Peripherals {
    pub gpio: GpioPeripheral,
    pub delay: SocDelay,
}

impl Peripherals {
    pub(crate) fn new() -> Self {
        Peripherals {
            gpio: GpioPeripheral::new(),
            delay: SocDelay,
        }
    }
}