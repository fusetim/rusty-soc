//! Display module 
//! 
//! This module provides abstractions and utilities for managing 
//! the onboard OLED SDD1351 display.

use embedded_hal::{delay, digital::PinState};

use crate::{gpio::{OledPins, SpiOledPins, never_bank::NeverPin, spi_oled_bank::{SpiOledClk, SpiOledCs, SpiOledDc, SpiOledMosi, SpiOledRes}}, spi::SpiSoft};

#[derive(Debug)]
pub struct DisplayPeripheral<DELAYER>
where DELAYER: crate::delay::DelayNs + Clone, {
    initialized: bool,
    spi: SpiSoft<SpiOledClk, SpiOledMosi, NeverPin, DELAYER>,
    dc: SpiOledDc,
    res: SpiOledRes,
    cs: SpiOledCs,
    delay: DELAYER,
}

impl<DELAYER> DisplayPeripheral<DELAYER>
where DELAYER: crate::delay::DelayNs + Clone, {
    pub fn new(oled_pins: OledPins, delay: DELAYER) -> Self {
        let (cs, mosi, clk, dc, res) = oled_pins;
        let spi = SpiSoft::new(
            SpiOledPins {
                mosi,
                clk,
                miso: NeverPin(PinState::Low),
            },
            delay.clone(),
        );
        Self {
            initialized: false,
            spi,
            dc,
            res,
            cs,
            delay,
        }
    }

    /// Forcefully retrieves the OLED pins, consuming the DisplayPeripheral.
    /// # Safety
    /// This function is unsafe because it allows bypassing any ownership
    /// checks that would normally prevent multiple mutable references to the
    /// same hardware resource.
    /// 
    /// In particular, the OLED display if the hardware framebuffer is used,
    /// may lead to undefined behavior if accessed concurrently.
    pub unsafe fn force_bring_down(self) -> OledPins {
        let (cs, mosi, clk, dc, res) = (
            self.cs,
            self.spi.mosi,
            self.spi.clk,
            self.dc,
            self.res,
        );
        (cs, mosi, clk, dc, res)
    }

    /// Safely retrieves the OLED pins, consuming the DisplayPeripheral.
    /// Returns an error if the pins are currently in use.
    pub fn bring_down(self) -> Result<OledPins, ()> {
        // TODO: Implement checks to ensure safe retrieval of OLED pins
        Ok(unsafe { self.force_bring_down() })
    }

    /// Check if the display has been initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Mark the display as initialized
    pub unsafe fn set_initialized(&mut self) {
        self.initialized = true;
    }

    /// Initialize the display hardware
    /// 
    /// This function sets up the OLED display for use.
    /// It will also reset the display if it has not been initialized yet.
    pub fn initialize(&mut self) {
        self.initialized = false;

        // Ensure CS is high before starting
        self.cs.set_high().unwrap();

        // Reset sequence
        {
            // RESET logic: active when low
            self.res.set_low().unwrap();
            self.delay.delay_ms(30); // wait 30ms
            self.res.set_high().unwrap();
            self.delay.delay_ms(30); // wait 30ms
            self.res.set_low().unwrap();
            self.delay.delay_ms(30); // wait 30ms
        }

        // Initialization commands
        {
            // Send CMD 0xAF (Display ON)
            // Wait 30ms
            // Send CMD 0xA0 (Set Remap & Color Depth)
            // Wait 1micros
            // Send DATA 0xA0 (RGB 666) or 0x20 (RGB565)
            // Wait 1micros
            // Send CMD 0xFD (Unlock Command)
            // Wait 1micros
            // Send DATA 0xB1
            // Wait 1micros
            // Send CMD 0xA2 (Set Display Start Line)
            // Wait 1micros
            // Send DATA 0x00
            // Wait 1micros
        }
    }
}
