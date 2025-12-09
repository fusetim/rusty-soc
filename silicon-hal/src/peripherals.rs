use crate::{delay::SocDelay, fb::FbPeripheral, gpio::GpioPeripheral};

/// Peripherals
#[derive(Debug)]
pub struct Peripherals {
    pub gpio: GpioPeripheral,
    pub delay: SocDelay,
    pub fb: FbPeripheral,
}

impl Peripherals {
    #[inline(always)]
    pub(crate) fn new() -> Self {
        Peripherals {
            gpio: GpioPeripheral::new(),
            delay: SocDelay,
            fb: FbPeripheral,
        }
    }
}
