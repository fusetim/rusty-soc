#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    led: Led,
    _reserved1: [u8; 0x02],
    btn: Btn,
    _reserved2: [u8; 0x03],
    spi_sdcard: SpiSdcard,
    _reserved3: [u8; 0x02],
    spi_oled: SpiOled,
}
impl RegisterBlock {
    #[doc = "0x00 - Control of the output of the on-board LEDs."]
    #[inline(always)]
    pub const fn led(&self) -> &Led {
        &self.led
    }
    #[doc = "0x04 - Get state of the on-board BTN inputs."]
    #[inline(always)]
    pub const fn btn(&self) -> &Btn {
        &self.btn
    }
    #[doc = "0x08 - Control of the input-output pins directly wired to the SD card SPI interface. Some pins are input-only or output-only."]
    #[inline(always)]
    pub const fn spi_sdcard(&self) -> &SpiSdcard {
        &self.spi_sdcard
    }
    #[doc = "0x0c - Control of the input-output pins directly wired to the OLED display SPI interface. Some pins are input-only or output-only."]
    #[inline(always)]
    pub const fn spi_oled(&self) -> &SpiOled {
        &self.spi_oled
    }
}
#[doc = "LED (rw) register accessor: Control of the output of the on-board LEDs.\n\nYou can [`read`](crate::Reg::read) this register and get [`led::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`led::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@led`] module"]
#[doc(alias = "LED")]
pub type Led = crate::Reg<led::LedSpec>;
#[doc = "Control of the output of the on-board LEDs."]
pub mod led;
#[doc = "BTN (r) register accessor: Get state of the on-board BTN inputs.\n\nYou can [`read`](crate::Reg::read) this register and get [`btn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btn`] module"]
#[doc(alias = "BTN")]
pub type Btn = crate::Reg<btn::BtnSpec>;
#[doc = "Get state of the on-board BTN inputs."]
pub mod btn;
#[doc = "SPI_SDCARD (rw) register accessor: Control of the input-output pins directly wired to the SD card SPI interface. Some pins are input-only or output-only.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_sdcard::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sdcard::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_sdcard`] module"]
#[doc(alias = "SPI_SDCARD")]
pub type SpiSdcard = crate::Reg<spi_sdcard::SpiSdcardSpec>;
#[doc = "Control of the input-output pins directly wired to the SD card SPI interface. Some pins are input-only or output-only."]
pub mod spi_sdcard;
#[doc = "SPI_OLED (rw) register accessor: Control of the input-output pins directly wired to the OLED display SPI interface. Some pins are input-only or output-only.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_oled::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_oled::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_oled`] module"]
#[doc(alias = "SPI_OLED")]
pub type SpiOled = crate::Reg<spi_oled::SpiOledSpec>;
#[doc = "Control of the input-output pins directly wired to the OLED display SPI interface. Some pins are input-only or output-only."]
pub mod spi_oled;
