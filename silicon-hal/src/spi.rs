use core::convert::Infallible;

use crate::gpio::{InputCapablePin, OutputCapablePin, SpiPins};
use crate::pac::{self, Spi0 as PacSpi0};
use crate::typesafe::Sealed;
use embedded_hal::spi::{ErrorType, SpiBus, SpiDevice};

pub trait SpiPeripheral: Sealed + 'static {
    /// Get the register block for SPI
    fn get_perif() -> &'static pac::spi0::RegisterBlock;
}

pub struct Spi<P: SpiPeripheral> {
    peripheral: P,
}

pub struct Spi0 {
    _inner: (),
}

impl Spi0 {
    pub(crate) fn new() -> Self {
        Self { _inner: () }
    }
}

impl Sealed for Spi0 {}
impl SpiPeripheral for Spi0 {
    fn get_perif() -> &'static pac::spi0::RegisterBlock {
        // Safety: Only used for reading/writing SPI0 registers
        // Spi0 is a singleton peripheral
        unsafe { &*PacSpi0::ptr() }
    }
}

impl<P> Spi<P>
where
    P: SpiPeripheral,
{
    pub fn new(peripheral: P) -> Self {
        Self { peripheral }
    }

    pub fn bring_down(self) -> P {
        self.peripheral
    }
}

impl<P> ErrorType for Spi<P>
where
    P: SpiPeripheral,
{
    type Error = Infallible;
}

impl<P> SpiBus<u8> for Spi<P>
where
    P: SpiPeripheral,
{
    #[inline(always)]
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        // Unfortunately, SPI peripheral requires writing to initiate reads
        for word in words.iter_mut() {
            *word = 0;
        }
        // Initiate the read by transferring the dummy data
        return self.transfer_in_place(words);
    }

    #[inline(always)]
    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        let spi = Spi0::get_perif();
        for &word in words {
            self.flush()?; // Ensure no other transfers are ongoing
            // Write the Tx byte to send, this will also start the transfer
            spi.write_data()
                .write(|w: &mut pac::spi0::write_data::W| unsafe { w.bits(word) });
        }
        Ok(())
    }

    #[inline(always)]
    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        let spi = Spi0::get_perif();
        let len = read.len().min(write.len());
        self.flush()?; // Ensure no other transfers are ongoing
        for i in 0..len {
            // Write the Tx byte to send, this will also start the transfer
            spi.write_data()
                .write(|w: &mut pac::spi0::write_data::W| unsafe { w.bits(write[i]) });
            // Wait until data is ready
            loop {
                let read_and_status = spi.read_and_status().read();
                if read_and_status.busy().bit_is_clear() {
                    // Read the received byte once ready
                    read[i] = read_and_status.data().bits();
                    break;
                }
            }
        }
        // Handle remaining bytes if read and write lengths differ
        if read.len() > len {
            self.read(&mut read[len..])?;
        } else {
            self.write(&write[len..])?;
        }
        Ok(())
    }

    #[inline(always)]
    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let spi = Spi0::get_perif();
        self.flush()?; // Ensure no other transfers are ongoing
        for word in words.iter_mut() {
            // Write the Tx byte to send, this will also start the transfer
            spi.write_data()
                .write(|w: &mut pac::spi0::write_data::W| unsafe { w.bits(*word) });
            // Wait until data is ready
            loop {
                let read_and_status = spi.read_and_status().read();
                if read_and_status.busy().bit_is_clear() {
                    // Read the received byte once ready
                    *word = read_and_status.data().bits();
                    break;
                }
            }
        }
        Ok(())
    }

    #[inline(always)]
    fn flush(&mut self) -> Result<(), Self::Error> {
        // Flush -- ensure the SPI peripheral status is not busy
        let spi = Spi0::get_perif();
        while spi.status().read().busy().bit_is_set() {}
        Ok(())
    }
}

/// Software (bit-banged) SPI implementation
/// Uses embedded-hal traits for GPIO pins and delay
///
/// Assumes Mode 0 (CPOL=0, CPHA=0) operation
/// SPI frequency assumes to be 400kHz - one symbol is 2.5us
pub struct SpiSoft<CLK, MOSI, MISO, DELAYER>
where
    CLK: OutputCapablePin,
    MOSI: OutputCapablePin,
    MISO: InputCapablePin,
    DELAYER: embedded_hal::delay::DelayNs,
{
    spi: SpiPins<MOSI, CLK, MISO>,
    delay: DELAYER,
}

impl<CLK, MOSI, MISO, DELAYER> SpiSoft<CLK, MOSI, MISO, DELAYER>
where
    CLK: OutputCapablePin,
    MOSI: OutputCapablePin,
    MISO: InputCapablePin,
    DELAYER: embedded_hal::delay::DelayNs,
{
    pub fn new(spi: SpiPins<MOSI, CLK, MISO>, delay: DELAYER) -> Self {
        Self { spi, delay }
    }

    /// Consume the software SPI and return the underlying pins
    pub fn bring_down(self) -> SpiPins<MOSI, CLK, MISO> {
        self.spi
    }

    /// Transfer a single byte over SPI and return the received byte
    #[inline(always)]
    fn transfer_byte(&mut self, byte: u8) -> u8 {
        // 400kHz SPI: 2.5us per bit = 1.25us high, 1.25us low
        let mut received: u8 = 0;
        for i in (0..8).rev() {
            // Set CLK low and
            self.spi.clk.set_low().unwrap();
            // Set MOSI based on the current bit to send
            if (byte & (1 << i)) != 0 {
                self.spi.mosi.set_high().unwrap();
            } else {
                self.spi.mosi.set_low().unwrap();
            }
            self.delay.delay_ns(1250);
            // Set CLK high to sample
            self.spi.clk.set_high().unwrap();
            // Read MISO
            let miso_high = self.spi.miso.is_high().unwrap();
            if miso_high {
                received |= 1 << i;
            }
            self.delay.delay_ns(1250);
        }
        received
    }
}

impl<CLK, MOSI, MISO, DELAYER> ErrorType for SpiSoft<CLK, MOSI, MISO, DELAYER>
where
    CLK: OutputCapablePin,
    MOSI: OutputCapablePin,
    MISO: InputCapablePin,
    DELAYER: embedded_hal::delay::DelayNs,
{
    type Error = Infallible;
}

impl<CLK, MOSI, MISO, DELAYER> SpiBus<u8> for SpiSoft<CLK, MOSI, MISO, DELAYER>
where
    CLK: OutputCapablePin,
    MOSI: OutputCapablePin,
    MISO: InputCapablePin,
    DELAYER: embedded_hal::delay::DelayNs,
{
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        for word in words.iter_mut() {
            *word = self.transfer_byte(0x00);
        }
        Ok(())
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        for &word in words {
            self.transfer_byte(word);
        }
        Ok(())
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        let len = read.len().min(write.len());
        for i in 0..len {
            read[i] = self.transfer_byte(write[i]);
        }
        // Handle remaining bytes if read and write lengths differ
        if read.len() > len {
            self.read(&mut read[len..])?;
        } else {
            self.write(&write[len..])?;
        }
        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        for word in words.iter_mut() {
            *word = self.transfer_byte(*word);
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        // Soft SPI is always ready
        Ok(())
    }
}
