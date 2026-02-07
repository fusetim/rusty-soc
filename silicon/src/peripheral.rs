pub use audio_streamer::*;
pub use btns::*;
pub use leds::*;
pub use oled::*;
pub use sdcard::*;

#[allow(dead_code)]
mod audio_streamer {
    //! Type aliases for Audio Streamer peripherals
    //!
    //! These types are used to simplify the interface for working with the Audio Streamer.

    use silicon_hal::delay::IntrDelay;

    /// Delayer used for the Audio Streamer.
    pub type AudioStreamerDelay = IntrDelay;

    /// Audio mode for the Audio Streamer.
    pub type AudioMode = silicon_hal::audio::Mono;

    /// Audio Streamer peripheral / type.
    pub type AudioStreamer<STATE> = silicon_hal::audio::AudioStreamer<AudioMode, STATE>;
}

#[allow(dead_code)]
mod oled {
    //! Type aliases for OLED display peripherals
    //!
    //! These types are used to simplify the interface for working with the OLED display.
    use core::convert::Infallible;

    use embedded_hal_bus::spi::ExclusiveDevice;
    use silicon_hal::{
        delay::IntrDelay,
        display::DisplayPeripheral,
        gpio::{
            Pin,
            never_bank::NeverPin,
            spi_oled_bank::{SpiOledCs, SpiOledDc, SpiOledRes},
        },
        spi::{Spi, Spi1},
    };

    /// Delayer used for the OLED display.
    pub type OledDelay = IntrDelay;
    /// SPI interface used for the OLED display.
    pub type OledSpi = Spi<Spi1, OledDelay>;
    /// Pin CS type for the OLED SPI display (never).
    pub type OledNeverCsPin = Pin<NeverPin>;
    /// Pin CS type for the OLED SPI display.
    pub type OledCsPin = Pin<SpiOledCs>;
    /// Pin DC type for the OLED SPI display.
    pub type OledDcPin = Pin<SpiOledDc>;
    /// Pin RESET type for the OLED SPI display.
    pub type OledResetPin = Pin<SpiOledRes>;
    /// SPI device type for the OLED display.
    pub type OledSpiDevice = ExclusiveDevice<OledSpi, OledNeverCsPin, OledDelay>;
    /// OLED display peripheral type.
    pub type OledDisplay<STATE> = DisplayPeripheral<
        OledSpiDevice,
        OledCsPin,
        OledDcPin,
        OledResetPin,
        OledDelay,
        Infallible,
        <OledSpiDevice as embedded_hal::spi::ErrorType>::Error,
        STATE,
    >;
}

#[allow(dead_code)]
mod sdcard {
    //! Type aliases for SDCard peripherals
    //!
    //! These types are used to simplify the interface for working with the SDCard.
    use embedded_hal_bus::spi::ExclusiveDevice;
    use silicon_hal::{
        delay::IntrDelay,
        gpio::{Pin, spi_sdcard_bank::SpiSdCs},
        spi::{Spi, Spi0},
    };

    /// Delayer used for the SDCard.
    pub type SdCardDelay = IntrDelay;
    /// SPI interface used for the SDCard.
    pub type SdCardSpi = Spi<Spi0, SdCardDelay>;
    /// Pin CS type for the SDCard SPI.
    pub type SdCardCsPin = Pin<SpiSdCs>;
    /// SPI device type for the SDCard.
    pub type SdCardSpiDeviceType = ExclusiveDevice<SdCardSpi, SdCardCsPin, SdCardDelay>;
    /// SDCard peripheral / Block Device type.
    pub type SdCard = embedded_sdmmc::SdCard<SdCardSpiDeviceType, SdCardDelay>;
}

#[allow(dead_code)]
mod leds {
    //! Type aliases for LED peripherals
    //!
    //! These types are used to simplify the interface for working with the LEDs.
    pub use silicon_hal::gpio::LedBank;
}

#[allow(dead_code)]
mod btns {
    //! Type aliases for Button peripherals
    //!
    //! These types are used to simplify the interface for working with the Buttons.
    pub use silicon_hal::gpio::BtnBank;
}
