use core::convert::Infallible;

use crate::pac::{self, Spi0 as PacSpi0};
use crate::typesafe::Sealed;
use embedded_hal::spi::{ErrorType, SpiBus};

pub trait SpiPeripheral: Sealed + 'static {
    /// Get the register block for SPI
    fn get_perif() -> &'static pac::spi0::RegisterBlock;
}

#[derive(Debug)]
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
