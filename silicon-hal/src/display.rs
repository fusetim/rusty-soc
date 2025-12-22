//! Display module
//!
//! This module provides abstractions and utilities for managing
//! the onboard OLED SDD1351 display.

use core::fmt::Debug;

use embedded_hal::{digital::OutputPin, spi::{Operation, SpiDevice}};

pub mod spidisplay;

use crate::{display::spidisplay::{Ssd1351Command, cmd::{ClockDivCommand, CommandLockCommand, ContrastAbcCommand, ContrastMasterCommand, DisplayGpioConfig, DisplayOffCommand, DisplayOffsetCommand, DisplayOnCommand, FunctionSelectCommand, FunctionSelectParallelOption, InvertDisplayCommand, MuxRatioCommand, NormalDisplayCommand, Precharge2Command, PrechargeCommand, SetColumnCommand, SetGpioCommand, SetRemapCommand, SetRowCommand, SetVslCommand, VcomhCommand, WriteRamCommand}}, typesafe::Sealed};

/// Marker trait for display states
pub trait DisplayState : Sealed {}
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

impl<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR> DisplayPeripheral<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR, Uninitialized>
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

    #[inline(always)]
    fn send_command<const N: usize>(&mut self, command: &dyn Ssd1351Command<N>) -> Result<(), DisplayError<PINERR, SPIERR>> {
        // Begin transation
        self.cs.set_low().map_err(|e| DisplayError::PinError(e))?;
        
        // Send command ID
        self.dc.set_low().map_err(|e| DisplayError::PinError(e))?;
        let cmd_id = command.command_id() as u8;
        self.spi.write(&[cmd_id]).map_err(|e| DisplayError::SpiError(e))?;

        // Send command data
        let data = command.command_data();
        if N > 0 {
            self.dc.set_high().map_err(|e| DisplayError::PinError(e))?;
            self.spi.write(&data).map_err(|e| DisplayError::SpiError(e))?;
        }

        // End transaction
        self.cs.set_high().map_err(|e| DisplayError::PinError(e))?;
        Ok(())
    }

    /// Initialize the display peripheral and bring it to Initialized state
    pub fn initialize(mut self) -> Result<DisplayPeripheral<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR, Initialized>, DisplayError<PINERR, SPIERR>> {
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
            self.send_command(&SetGpioCommand::new(DisplayGpioConfig::InputDisabled, DisplayGpioConfig::InputDisabled))?;
            self.send_command(&FunctionSelectCommand::new(FunctionSelectParallelOption::Parallel8Bit, false))?;
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
            remap.set_com_split_odd_even(true);
            self.send_command(&remap)?;
        }

        self.delay.delay_ms(1);

        // Set the display white
        {
            self.send_command(&SetColumnCommand::new(0,127).unwrap())?;
            self.send_command(&SetRowCommand::new(0,127).unwrap())?;
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
                            self.spi.write(&[high_byte, low_byte]).map_err(|e| DisplayError::SpiError(e))?;
                        }
                    }
                }
                color_offset += 1;
            }

            self.cs.set_high().map_err(|e| DisplayError::PinError(e))?;  
            self.dc.set_low().map_err(|e| DisplayError::PinError(e))?;   
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

impl<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR> DisplayPeripheral<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR, Initialized>
where
    SPI: SpiDevice<u8, Error = SPIERR>,
    CS: OutputPin<Error = PINERR>,
    DC: OutputPin<Error = PINERR>,
    RST: OutputPin<Error = PINERR>,
    DELAYER: crate::delay::DelayNs + Clone,
{
    #[inline(always)]
    pub fn send_command<const N: usize>(&mut self, command: &dyn Ssd1351Command<N>) -> Result<(), DisplayError<PINERR, SPIERR>> {
        // Begin transation
        self.cs.set_low().map_err(|e| DisplayError::PinError(e))?;
        
        // Send command ID
        {
            self.dc.set_low().map_err(|e| DisplayError::PinError(e))?;
            let cmd_id = command.command_id() as u8;
            let mut spi_cmd = [Operation::Write(&[cmd_id])];
            self.spi.transaction(&mut spi_cmd).map_err(|e| DisplayError::SpiError(e))?;
        }

        // Send command data
        let data = command.command_data();
        if N > 0 {
            let mut spi_data = [Operation::Write(&data)];
            self.dc.set_high().map_err(|e| DisplayError::PinError(e))?;
            self.spi.transaction(&mut spi_data).map_err(|e| DisplayError::SpiError(e))?;
        }

        // End transaction
        self.cs.set_high().map_err(|e| DisplayError::PinError(e))?;
        Ok(())
    }

    /// Disable the display peripheral and bring the Display Peripheral in Unitialized state
    #[inline(always)]
    pub fn disable(mut self) -> DisplayPeripheral<SPI, CS, DC, RST, DELAYER, PINERR, SPIERR, Uninitialized> {
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
}