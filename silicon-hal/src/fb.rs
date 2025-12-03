use crate::config::{FB_BASE, FB_CONFIG_BASE};
use core::ptr::{read_volatile, write_volatile};

/// Internal Framebuffer Peripheral
#[derive(Debug, Clone, Copy)]
pub struct FbPeripheral;
pub(crate) const INTERNAL_FB: FbPeripheral = FbPeripheral;

mod reg {
    use super::FB_CONFIG_BASE;

    /// Select Color Channel Register
    pub const REG_SELECT_COLOR_CHANNEL: usize = FB_CONFIG_BASE | (0x00 << 2);
}

/// Framebuffer Color Channels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FbColorChannel {
    Disabled,
    Grey,
    Red,
    Green,
    Blue,
}

impl FbColorChannel {
    /// Get the corresponding value for the color channel
    fn value(&self) -> u8 {
        match self {
            FbColorChannel::Grey => 0b111,
            FbColorChannel::Red  => 0b100,
            FbColorChannel::Green=> 0b010,
            FbColorChannel::Blue => 0b001,
            FbColorChannel::Disabled => 0b000,
        }
    }
}

impl FbPeripheral {
    /// Write a raw pixel component value at (x, y)
    /// # Arguments
    /// * `x` - The x coordinate of the pixel (0-127)
    /// * `y` - The y coordinate of the pixel (0-127)
    /// * `value` - The raw pixel component value (0-255)
    pub fn write_raw_pixel(&self, x: u8, y: u8, value: u8) {
        let offset = (y as usize) * 128 + (x as usize);
        unsafe {
            write_volatile((FB_BASE + offset) as *mut u8, value);
        }
    }

    /// Select a color channel for framebuffer operations
    /// # Arguments
    /// * `channel` - The color channel to select (Grey, Red, Green, Blue)
    pub fn select_color_channel(&self, channel: FbColorChannel) {
        let value = channel.value() as u32;
        unsafe {
            write_volatile(reg::REG_SELECT_COLOR_CHANNEL as *mut u32, value);
        }
    }
}