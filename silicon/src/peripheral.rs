pub use oled::*;
pub use sdcard::*;
pub use leds::*;
pub use btns::*;
pub use audio_streamer::*;

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


mod sdcard {
    //! Type aliases for SDCard peripherals
    //! 
    //! These types are used to simplify the interface for working with the SDCard.
    use embedded_hal_bus::spi::ExclusiveDevice;
    use silicon_hal::{
        delay::IntrDelay, gpio::{
            Pin, spi_sdcard_bank::SpiSdCs,
        }, spi::{Spi, Spi0,}
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

mod leds {
    //! Type aliases for LED peripherals
    //! 
    //! These types are used to simplify the interface for working with the LEDs.

    use core::{convert::Infallible};

    use embedded_hal::digital::OutputPin;
    use silicon_hal::gpio::{Pin, led_bank::{Led0, Led1, Led2, Led3, Led4, Led5, Led6, Led7}};

    use crate::VoidUnwrap;
    
    pub struct LedBank {
        pub led0: Pin<Led0>,
        pub led1: Pin<Led1>,
        pub led2: Pin<Led2>,
        pub led3: Pin<Led3>,
        pub led4: Pin<Led4>,
        pub led5: Pin<Led5>,
        pub led6: Pin<Led6>,
        pub led7: Pin<Led7>,
    }

    impl LedBank {
        /// Create a new LED bank from individual LED pins.
        pub fn new(
            led0: Pin<Led0>,
            led1: Pin<Led1>,
            led2: Pin<Led2>,
            led3: Pin<Led3>,
            led4: Pin<Led4>,
            led5: Pin<Led5>,
            led6: Pin<Led6>,
            led7: Pin<Led7>,
        ) -> Self {
            Self {
                led0,
                led1,
                led2,
                led3,
                led4,
                led5,
                led6,
                led7,
            }
        }

        /// Create a new LED bank from GPIO raw pins
        pub fn from_gpio_pins(
            gpio_led0: Led0,
            gpio_led1: Led1,
            gpio_led2: Led2,
            gpio_led3: Led3,
            gpio_led4: Led4,
            gpio_led5: Led5,
            gpio_led6: Led6,
            gpio_led7: Led7,
        ) -> Self {
            Self::new(
                Pin::new_output(gpio_led0),
                Pin::new_output(gpio_led1),
                Pin::new_output(gpio_led2),
                Pin::new_output(gpio_led3),
                Pin::new_output(gpio_led4),
                Pin::new_output(gpio_led5),
                Pin::new_output(gpio_led6),
                Pin::new_output(gpio_led7),
            )
        }

        /// Get mutable references to all LED pins as an array.
        pub fn as_pins(&mut self) -> [&mut dyn OutputPin<Error = Infallible>; 8] {
            [
                &mut self.led0,
                &mut self.led1,
                &mut self.led2,
                &mut self.led3,
                &mut self.led4,
                &mut self.led5,
                &mut self.led6,
                &mut self.led7,
            ]
        }

        pub fn set_all_low(&mut self) {
            for led in self.as_pins().iter_mut() {
                led.set_low().void_unwrap();
            }
        }

        pub fn set_all_high(&mut self) {
            for led in self.as_pins().iter_mut() {
                led.set_high().void_unwrap();
            }
        }

        pub fn set_all_states(&mut self, states: [bool; 8]) {
            let pins = self.as_pins();
            for (pin, &state) in pins.into_iter().zip(states.iter()) {
                if state {
                    pin.set_high().void_unwrap();
                } else {
                    pin.set_low().void_unwrap();
                }
            }
        }
    }
}

mod btns {
    //! Type aliases for Button peripherals
    //! 
    //! These types are used to simplify the interface for working with the Buttons.

    use core::{convert::Infallible};

    use embedded_hal::digital::InputPin;
    use silicon_hal::gpio::{Pin, btn_bank::{Btn1, Btn2, Btn3, Btn4, Btn5, Btn6}};
    
    pub struct BtnBank {
        pub btn1: Pin<Btn1>,
        pub btn2: Pin<Btn2>,
        pub btn3: Pin<Btn3>,
        pub btn4: Pin<Btn4>,
        pub btn5: Pin<Btn5>,
        pub btn6: Pin<Btn6>,
    }

    impl BtnBank {
        /// Create a new Button bank from individual Button pins.
        pub fn new(
            btn1: Pin<Btn1>,
            btn2: Pin<Btn2>,
            btn3: Pin<Btn3>,
            btn4: Pin<Btn4>,
            btn5: Pin<Btn5>,
            btn6: Pin<Btn6>,
        ) -> Self {
            Self {
                btn1,
                btn2,
                btn3,
                btn4,
                btn5,
                btn6,
            }
        }

        /// Create a new Button bank from GPIO raw pins
        pub fn from_gpio_pins(
            gpio_btn1: Btn1,
            gpio_btn2: Btn2,
            gpio_btn3: Btn3,
            gpio_btn4: Btn4,
            gpio_btn5: Btn5,
            gpio_btn6: Btn6,
        ) -> Self {
            Self::new(
                Pin::new_input(gpio_btn1),
                Pin::new_input(gpio_btn2),
                Pin::new_input(gpio_btn3),
                Pin::new_input(gpio_btn4),
                Pin::new_input(gpio_btn5),
                Pin::new_input(gpio_btn6),
            )
        }


        /// Get references to all Button pins as an array.
        pub fn as_pins(&self) -> [&dyn InputPin<Error = Infallible>; 6] {
            [
                &self.btn1,
                &self.btn2,
                &self.btn3,
                &self.btn4,
                &self.btn5,
                &self.btn6,
            ]
        }
    }
}