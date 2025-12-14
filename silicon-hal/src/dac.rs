//! Digital-to-Analog Converter (DAC) Peripheral
//!
//! This peripheral provides audio output functionality using PWM.
//! The DAC accepts 8-bit audio samples.

use crate::pac::{self};
pub struct AudioDac {
    _inner: (),
}

impl AudioDac {
    /// Creates a new instance of the Audio DAC peripheral.
    pub(crate) fn new() -> Self {
        AudioDac { _inner: () }
    }

    #[inline(always)]
    pub fn write_left_sample(&mut self, sample: u8) {
        // Safety: We ensure exclusive access to the DAC peripheral.
        unsafe {
            let dac = pac::Dac::steal();
            dac.left_output().write(|w| w.value().bits(sample));
        }
    }

    #[inline(always)]
    pub fn write_right_sample(&mut self, sample: u8) {
        // Safety: We ensure exclusive access to the DAC peripheral.
        unsafe {
            let dac = pac::Dac::steal();
            dac.right_output().write(|w| w.value().bits(sample));
        }
    }

    #[inline(always)]
    pub fn write_stereo_sample(&mut self, left: u8, right: u8) {
        // Safety: We ensure exclusive access to the DAC peripheral.
        unsafe {
            let dac = pac::Dac::steal();
            dac.output().write(|w| {
                w.left_output().bits(left);
                w.right_output().bits(right)
            });
        }
    }
}
