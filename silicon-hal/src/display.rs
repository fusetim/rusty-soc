//! Display module
//!
//! This module provides abstractions and utilities for managing
//! the onboard OLED SDD1351 display.

use core::fmt::Debug;

use embedded_hal::{
    digital::OutputPin,
    spi::{Operation, SpiDevice},
};

pub mod spidisplay;

use crate::{
    display::spidisplay::{
        Ssd1351Command,
        cmd::{
            ClockDivCommand, CommandLockCommand, ContrastAbcCommand, ContrastMasterCommand,
            DisplayGpioConfig, DisplayOffCommand, DisplayOffsetCommand, DisplayOnCommand,
            FunctionSelectCommand, FunctionSelectParallelOption, InvertDisplayCommand,
            MuxRatioCommand, NormalDisplayCommand, Precharge2Command, PrechargeCommand,
            SetColumnCommand, SetGpioCommand, SetRemapCommand, SetRowCommand, SetVslCommand,
            VcomhCommand, WriteRamCommand,
        },
    },
    typesafe::Sealed,
};

/// Marker trait for display states
pub trait DisplayState: Sealed {}
pub struct Uninitialized;
pub struct Initialized;
impl Sealed for Uninitialized {}
impl Sealed for Initialized {}
impl DisplayState for Uninitialized {}
impl DisplayState for Initialized {}

/// Errors that can occur in the DisplayPeripheral
pub enum DisplayError<PINERR, SPIERR> {
    /// Underlying pin error
    PinError(PINERR),
    /// Underlying SPI error
    SpiError(SPIERR),
}

impl<PINERR, SPIERR> Debug for DisplayError<PINERR, SPIERR>
where
    PINERR: Debug,
    SPIERR: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DisplayError::PinError(e) => f.debug_tuple("PinError").field(e).finish(),
            DisplayError::SpiError(e) => f.debug_tuple("SpiError").field(e).finish(),
        }
    }
}

pub struct DisplayPeripheral<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR, STATE>
where
    SPI: SpiDevice<u8, Error = SPIERR>,
    CS: OutputPin<Error = PINERR>,
    DC: OutputPin<Error = PINERR>,
    RST: OutputPin<Error = PINERR>,
    DELAYER: crate::delay::DelayNs + Clone,
    STATE: DisplayState,
{
    spi: SPI,
    cs: CS,
    dc: DC,
    rst: RST,
    delay: DELAYER,
    state: STATE,
}

impl<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR>
    DisplayPeripheral<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR, Uninitialized>
where
    SPI: SpiDevice<u8, Error = SPIERR>,
    CS: OutputPin<Error = PINERR>,
    DC: OutputPin<Error = PINERR>,
    RST: OutputPin<Error = PINERR>,
    DELAYER: crate::delay::DelayNs + Clone,
{
    /// Creates a new DisplayPeripheral instance
    ///
    /// Unfortuantely, if you want to use the Chip Select pin (CS),
    /// you must provide it here and not as part of the SPI device.
    /// This is because the display requires manual control of the CS pin
    /// for each command/data transfer.
    #[inline(always)]
    pub fn new(spi: SPI, cs: CS, dc: DC, rst: RST, delay: DELAYER) -> Self {
        Self {
            state: Uninitialized,
            spi,
            cs,
            dc,
            rst,
            delay,
        }
    }

    /// Bring down the display peripheral into its components
    #[inline(always)]
    pub fn bring_down(self) -> (SPI, CS, DC, RST, DELAYER) {
        (self.spi, self.cs, self.dc, self.rst, self.delay)
    }

    fn send_command<const N: usize>(
        &mut self,
        command: &dyn Ssd1351Command<N>,
    ) -> Result<(), DisplayError<PINERR, SPIERR>> {
        // Begin transation
        self.cs.set_low().map_err(|e| DisplayError::PinError(e))?;

        // Send command ID
        self.dc.set_low().map_err(|e| DisplayError::PinError(e))?;
        let cmd_id = command.command_id() as u8;
        self.spi
            .write(&[cmd_id])
            .map_err(|e| DisplayError::SpiError(e))?;

        // Send command data
        let data = command.command_data();
        if N > 0 {
            self.dc.set_high().map_err(|e| DisplayError::PinError(e))?;
            self.spi
                .write(&data)
                .map_err(|e| DisplayError::SpiError(e))?;
        }

        // End transaction
        self.cs.set_high().map_err(|e| DisplayError::PinError(e))?;
        Ok(())
    }

    /// Initialize the display peripheral and bring it to Initialized state
    pub fn initialize(
        mut self,
    ) -> Result<
        DisplayPeripheral<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR, Initialized>,
        DisplayError<PINERR, SPIERR>,
    > {
        // Desassert the display
        self.cs.set_high().map_err(|e| DisplayError::PinError(e))?;
        // Reset the display for 1 ms
        self.rst.set_high().map_err(|e| DisplayError::PinError(e))?;
        self.delay.delay_ms(100);
        self.rst.set_low().map_err(|e| DisplayError::PinError(e))?;
        self.delay.delay_ms(100);
        self.rst.set_high().map_err(|e| DisplayError::PinError(e))?;
        self.delay.delay_ms(100);

        // Send initialization commands to the display
        {
            // Sequence based on Adafruit example
            self.send_command(&CommandLockCommand::UnlockIC)?;
            self.send_command(&CommandLockCommand::UnlockRestricted)?;
            self.send_command(&DisplayOffCommand {})?;
            self.send_command(&ClockDivCommand::new(1, 15))?;
            self.send_command(&MuxRatioCommand::new(127).unwrap())?;
            self.send_command(&DisplayOffsetCommand::new(0).unwrap())?;
            self.send_command(&SetGpioCommand::new(
                DisplayGpioConfig::InputDisabled,
                DisplayGpioConfig::InputDisabled,
            ))?;
            self.send_command(&FunctionSelectCommand::new(
                FunctionSelectParallelOption::Parallel8Bit,
                false,
            ))?;
            self.send_command(&PrechargeCommand::new(2, 3))?;
            self.send_command(&VcomhCommand::new(5))?;
            self.send_command(&ContrastAbcCommand::new(0xC8, 0x80, 0xC8))?;
            self.send_command(&ContrastMasterCommand::new(0x0F))?;
            self.send_command(&SetVslCommand {})?;
            self.send_command(&Precharge2Command::new(1))?;
            self.send_command(&DisplayOnCommand {})?;

            // Remap: none + color
            let mut remap = SetRemapCommand::default();
            remap.set_color_depth(0b00); // 565
            remap.set_color_seq(true); // RGB
            remap.set_col_addr_map(true); // Column address mapped
            remap.set_com_split_odd_even(true);
            self.send_command(&remap)?;
        }

        Ok(DisplayPeripheral {
            spi: self.spi,
            cs: self.cs,
            dc: self.dc,
            rst: self.rst,
            delay: self.delay,
            state: Initialized,
        })
    }
}

impl<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR>
    DisplayPeripheral<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR, Initialized>
where
    SPI: SpiDevice<u8, Error = SPIERR>,
    CS: OutputPin<Error = PINERR>,
    DC: OutputPin<Error = PINERR>,
    RST: OutputPin<Error = PINERR>,
    DELAYER: crate::delay::DelayNs + Clone,
{
    pub fn send_command<const N: usize>(
        &mut self,
        command: &dyn Ssd1351Command<N>,
    ) -> Result<(), DisplayError<PINERR, SPIERR>> {
        // Begin transation
        self.cs.set_low().map_err(|e| DisplayError::PinError(e))?;

        // Send command ID
        {
            self.dc.set_low().map_err(|e| DisplayError::PinError(e))?;
            let cmd_id = command.command_id() as u8;
            let mut spi_cmd = [Operation::Write(&[cmd_id])];
            self.spi
                .transaction(&mut spi_cmd)
                .map_err(|e| DisplayError::SpiError(e))?;
        }

        // Send command data
        let data = command.command_data();
        if N > 0 {
            let mut spi_data = [Operation::Write(&data)];
            self.dc.set_high().map_err(|e| DisplayError::PinError(e))?;
            self.spi
                .transaction(&mut spi_data)
                .map_err(|e| DisplayError::SpiError(e))?;
        }

        // End transaction
        self.cs.set_high().map_err(|e| DisplayError::PinError(e))?;
        Ok(())
    }

    /// Disable the display peripheral and bring the Display Peripheral in Unitialized state
    #[inline(always)]
    pub fn disable(
        mut self,
    ) -> DisplayPeripheral<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR, Uninitialized> {
        // Shutdown procedure
        let _ = self.send_command(&DisplayOffCommand {}); // Shutdown display

        DisplayPeripheral {
            spi: self.spi,
            cs: self.cs,
            dc: self.dc,
            rst: self.rst,
            delay: self.delay,
            state: Uninitialized,
        }
    }

    /// Bring down the display peripheral into its components
    /// This will first disable the display before bringing down
    /// the peripheral components.
    #[inline(always)]
    pub fn bring_down(self) -> (SPI, CS, DC, RST, DELAYER) {
        self.disable().bring_down()
    }

    /// Set addressable window for subsequent RAM writes/reads
    #[inline(always)]
    pub(crate) fn set_address_window(
        &mut self,
        x0: u8,
        y0: u8,
        x1: u8,
        y1: u8,
    ) -> Result<(), DisplayError<PINERR, SPIERR>> {
        self.send_command(&SetColumnCommand::new(x0, x1).unwrap())?;
        self.send_command(&SetRowCommand::new(y0, y1).unwrap())?;
        Ok(())
    }

    /// Checkerboard display calibration pattern
    pub fn display_calibration_pattern(
        &mut self,
    ) -> Result<(), DisplayError<PINERR, SPIERR>> {
        self.set_address_window(0, 0, 127, 127)?;
        self.send_command(&WriteRamCommand {})?;
        self.dc.set_high().map_err(|e| DisplayError::PinError(e))?;
        self.cs.set_low().map_err(|e| DisplayError::PinError(e))?;

        let colors = [
            0b1111111111100000, // Yellow
            0b0000011111100000, // Green
            0b1111100000000000, // Red
            0b0000000000011111, // Blue
        ];

        // Display a "calibration" pattern (grid of 8x8 squares of 16x16 pixels)
        let mut color_offset = 0;
        for _ in 0..8 {
            // A row of 8 squares of 16x16 pixels
            for _ in 0..16 {
                // For each line of this row of 8 squares
                for k in 0..8 {
                    // For each square in this line
                    let color = colors[(color_offset + k) % colors.len()];
                    for _ in 0..16 {
                        // For each pixel in this square
                        let high_byte = (color >> 8) as u8;
                        let low_byte = (color & 0xFF) as u8;
                        self.spi
                            .write(&[high_byte, low_byte])
                            .map_err(|e| DisplayError::SpiError(e))?;
                    }
                }
            }
            color_offset += 1;
        }

        self.cs.set_high().map_err(|e| DisplayError::PinError(e))?;
        self.dc.set_low().map_err(|e| DisplayError::PinError(e))?;
        Ok(())
    }
}

#[cfg(feature = "graphics")]
mod graphics {
    use embedded_graphics_core::{pixelcolor::{Rgb565, raw::ToBytes}, prelude::{DrawTarget, OriginDimensions, Size}};
    use super::*;

    impl<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR>
        DisplayPeripheral<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR, Initialized>
    where
        SPI: SpiDevice<u8, Error = SPIERR>,
        CS: OutputPin<Error = PINERR>,
        DC: OutputPin<Error = PINERR>,
        RST: OutputPin<Error = PINERR>,
        DELAYER: crate::delay::DelayNs + Clone,
    {
        /// Draw an individual pixel at (x, y) with the given RGB565 color
        pub fn draw_pixel(
            &mut self,
            x: u8,
            y: u8,
            color: Rgb565,
        ) -> Result<(), DisplayError<PINERR, SPIERR>> {
            // Out of bounds -- ignore
            if x >= 128 || y >= 128 {
                return Ok(());
            }

            self.set_address_window(x, y, x, y)?;
            self.send_command(&WriteRamCommand {})?;
            self.dc.set_high().map_err(|e| DisplayError::PinError(e))?;
            self.cs.set_low().map_err(|e| DisplayError::PinError(e))?;

            let color: [u8; 2] = color.to_be_bytes();
            self.spi
                .write(&color)
                .map_err(|e| DisplayError::SpiError(e))?;

            self.cs.set_high().map_err(|e| DisplayError::PinError(e))?;
            Ok(())
        }

        /// Fill an entire rectangle defined by (x0, y0) to (x1, y1) with the given RGB565 color
        pub fn fill_rectangle(
            &mut self,
            x0: u8,
            y0: u8,
            x1: u8,
            y1: u8,
            color: Rgb565,
        ) -> Result<(), DisplayError<PINERR, SPIERR>> {
            // Clamp coordinates to display bounds
            // In debug mode, assert the coordinates are valid
            debug_assert!(x1 <= 127);
            debug_assert!(y1 <= 127);
            debug_assert!(x1 >= x0);
            debug_assert!(y1 >= y0);
            let x0 = x0.min(127);
            let y0 = y0.min(127);
            let x1 = x1.min(127);
            let y1 = y1.min(127);

            let color: [u8; 2] = color.to_be_bytes();
            let width = (x1 - x0 + 1) as u32;
            let height = (y1 - y0 + 1) as u32;
            let total_pixels = width * height;

            let mut buffer = [0u8; 128]; // Buffer for 64 pixels (128 bytes)
            // Fill buffer with the color
            for chunk in buffer.chunks_exact_mut(2) {
                chunk.copy_from_slice(&color);
            }

            self.set_address_window(x0, y0, x1, y1)?;
            self.send_command(&WriteRamCommand {})?;
            self.dc.set_high().map_err(|e| DisplayError::PinError(e))?;
            self.cs.set_low().map_err(|e| DisplayError::PinError(e))?;

            let mut pixels_written = 0;
            while pixels_written < total_pixels {
                let pixels_to_write = core::cmp::min(64, (total_pixels - pixels_written) as usize);
                let bytes_to_write = pixels_to_write * 2;
                self.spi
                    .write(&buffer[..bytes_to_write])
                    .map_err(|e| DisplayError::SpiError(e))?;
                pixels_written += pixels_to_write as u32;
            }

            self.cs.set_high().map_err(|e| DisplayError::PinError(e))?;
            Ok(())
        }

        /// Draw a continuous rectangle area defined by (x0, y0) to (x1, y1) with the given RGB565 pixels
        /// 
        /// The pixel data must be provided in row-major order (top-left to bottom-right).
        /// The length of the `pixels` iterator might be less than or equal to the rectangle area.
        /// If less, only that many pixels will be drawn.
        /// Note: If greater, they will not be consumed.
        pub fn draw_area<I>(
            &mut self,
            x0: u8,
            y0: u8,
            x1: u8,
            y1: u8,
            pixels: I,
        ) -> Result<(), DisplayError<PINERR, SPIERR>>
        where
            I: IntoIterator<Item = Rgb565>,
        {
            // Clamp coordinates to display bounds
            // In debug mode, assert the coordinates are valid
            debug_assert!(x1 <= 127);
            debug_assert!(y1 <= 127);
            debug_assert!(x1 >= x0);
            debug_assert!(y1 >= y0);
            let x0 = x0.min(127);
            let y0 = y0.min(127);
            let x1 = x1.min(127);
            let y1 = y1.min(127);

            let width = (x1 - x0 + 1) as u32;
            let height = (y1 - y0 + 1) as u32;
            let total_pixels = width * height;

            self.set_address_window(x0, y0, x1, y1)?;
            self.send_command(&WriteRamCommand {})?;
            self.dc.set_high().map_err(|e| DisplayError::PinError(e))?;
            self.cs.set_low().map_err(|e| DisplayError::PinError(e))?;

            // Transfer a pixel at a time
            let pixels: core::iter::Take<<I as IntoIterator>::IntoIter> = pixels.into_iter().take(total_pixels as usize);
            for pix in pixels {
                let color: [u8; 2] = pix.to_be_bytes();
                self.spi
                    .write(&color)
                    .map_err(|e| DisplayError::SpiError(e))?;
            }

            self.cs.set_high().map_err(|e| DisplayError::PinError(e))?;
            Ok(())
        }
    }

    impl<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR>
        DrawTarget 
        for DisplayPeripheral<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR, Initialized>
    where
        SPI: SpiDevice<u8, Error = SPIERR>,
        CS: OutputPin<Error = PINERR>,
        DC: OutputPin<Error = PINERR>,
        RST: OutputPin<Error = PINERR>,
        DELAYER: crate::delay::DelayNs + Clone,
    {
        type Color = Rgb565;
        type Error = DisplayError<PINERR, SPIERR>;
    
        // Draw pixels from an iterator
        // 
        // The iterator should yield pixels in any order, therefore we set each pixel individually.
        fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = embedded_graphics_core::Pixel<Self::Color>> {
            for pixel in pixels {
                let embedded_graphics_core::Pixel(coord, color) = pixel;
                let x = coord.x as u8;
                let y = coord.y as u8;
                self.draw_pixel(x, y, color)?;
            }
            Ok(())
        }

        // Fill a contiguous rectangle area with colors from an iterator
        // The colors must be provided in row-major order (top-left to bottom-right).
        // A lot more efficient than draw_iter for filled rectangle area.
        fn fill_contiguous<I>(&mut self, area: &embedded_graphics_core::primitives::Rectangle, colors: I) -> Result<(), Self::Error>
            where
                I: IntoIterator<Item = Self::Color>, {
            if let Some(br) = area.bottom_right() {
                // Non-zero area
                let x0 = area.top_left.x as u8;
                let y0 = area.top_left.y as u8;
                let x1 = br.x as u8;
                let y1 = br.y as u8;
                self.draw_area(x0, y0, x1, y1, colors)
            } else {
                // Zero area, nothing to fill
                return Ok(());
            }
        }

        // Fill a rectangle area with a single color
        fn fill_solid(&mut self, area: &embedded_graphics_core::primitives::Rectangle, color: Self::Color) -> Result<(), Self::Error> {
            // Check for rectangle size null
            if let Some(br) = area.bottom_right() {
                // Non-zero area
                let x0 = area.top_left.x as u8;
                let y0 = area.top_left.y as u8;
                let x1 = br.x as u8;
                let y1 = br.y as u8;
                self.fill_rectangle(x0, y0, x1, y1, color)
            } else {
                // Zero area, nothing to fill
                return Ok(());
            }
        }
    }

    impl<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR>
        OriginDimensions
        for DisplayPeripheral<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR, Initialized>
    where
        SPI: SpiDevice<u8, Error = SPIERR>,
        CS: OutputPin<Error = PINERR>,
        DC: OutputPin<Error = PINERR>,
        RST: OutputPin<Error = PINERR>,
        DELAYER: crate::delay::DelayNs + Clone,
    {
        fn size(&self) -> Size {
            Size::new(128, 128)
        }
    }
}