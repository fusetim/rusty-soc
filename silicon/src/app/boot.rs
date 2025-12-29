use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::{DrawTarget, RgbColor};
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{OutputPin, PinState};
use silicon_hal::{
    audio,
    dac::AudioDac,
    delay::INTR_DELAY,
    display::Initialized,
    gpio::{Gpio, IntoPin as _, Pin, never_bank::NeverPin},
    spi::{Spi0, Spi1},
};

use crate::app::LoadingState;
use crate::peripheral::{
    AudioStreamer, BtnBank, LedBank, OledDisplay, OledSpi, OledSpiDevice, SdCard, SdCardSpi,
    SdCardSpiDeviceType,
};

use super::AppState;

/// Run the booting state logic.
///
/// This function initializes the necessary peripherals and transitions the application
/// from the Booting state to the Menu state.
///
//// # Arguments
///
/// * `state` - The current application state, expected to be in the Booting state.
///
/// # Returns
///
/// * `Option<AppState>` - The new application state after booting, or None if an error occurred.
pub fn run_booting(state: AppState) -> Option<AppState> {
    if let AppState::Booting(booting_state) = state {
        let mut peripherals = booting_state.peripherals;

        // Setup the LED bank
        let mut led_bank = setup_leds(&mut peripherals.gpio);

        // Reset the LED status
        led_bank.set_all_low();
        led_bank.led0.set_high(); // LED ok

        // Setup the button bank
        let btn_bank = setup_btns(&mut peripherals.gpio);
        led_bank.led1.set_high(); // Buttons ok

        // Setup OLED display
        let oled_display: OledDisplay<Initialized> =
            setup_display(peripherals.spi1, &mut peripherals.gpio);
        led_bank.led2.set_high(); // Display ok

        // Setup the SDCard
        let sdcard = setup_sdcard(peripherals.spi0, &mut peripherals.gpio);
        led_bank.led3.set_high(); // SDCard ok

        // Setup the audio streamer
        let audio_streamer = setup_audio_streamer(peripherals.dac);
        led_bank.led4.set_high(); // Audio ok

        // Wait 1s before transitioning
        INTR_DELAY.delay_ms(1000);
        led_bank.set_all_high();
        INTR_DELAY.delay_ms(500);
        led_bank.set_all_low();

        // Transition to Loading state
        return Some(AppState::Loading(LoadingState {
            leds: led_bank,
            btns: btn_bank,
            display: oled_display,
            sdcard,
            audio_streamer,
        }));
    }
    None
}

fn setup_display(spi: Spi1, gpio: &mut Gpio) -> OledDisplay<Initialized> {
    // Initialize the OLED display here using the provided SPI and GPIO peripherals.

    // Get the needed GPIO pins for the OLED display
    let (mut oled_cs, oled_dc, oled_rst) = {
        let (oled_cs, _, _, oled_dc, oled_rst) = gpio.take_oled().unwrap();
        (oled_cs.into_pin(), oled_dc.into_pin(), oled_rst.into_pin())
    };
    oled_cs.set_high();

    // Create the SPI interface for the OLED display
    let oled_spi_cs = Pin::new_output(NeverPin(PinState::Low));
    let oled_spi = OledSpi::new(spi, INTR_DELAY);
    let oled_spi_device = OledSpiDevice::new(oled_spi, oled_spi_cs, INTR_DELAY).unwrap();

    // Create and initialize the OLED display peripheral
    let mut oled_display: OledDisplay<_> =
        OledDisplay::new(oled_spi_device, oled_cs, oled_dc, oled_rst, INTR_DELAY);

    let mut display = oled_display.initialize().unwrap();

    // Clear the display
    display.clear(Rgb565::BLACK);

    display
}

fn setup_sdcard(spi: Spi0, gpio: &mut Gpio) -> SdCard {
    // Initialize the SDCard here using the provided SPI and GPIO peripherals.
    let mut sd_cs = gpio.take_spi_sd_cs().unwrap().into_pin();
    sd_cs.set_high();

    // Setup the SPI interface for the SDCard
    let sd_spi = SdCardSpi::new(spi, INTR_DELAY);
    let sd_spi_device = SdCardSpiDeviceType::new(sd_spi, sd_cs, INTR_DELAY).unwrap();

    // Create and return the SDCard peripheral
    SdCard::new(sd_spi_device, INTR_DELAY)
}

fn setup_leds(gpio: &mut Gpio) -> LedBank {
    let leds = gpio.take_all_leds().unwrap();
    LedBank::new(
        leds.0.into_pin(),
        leds.1.into_pin(),
        leds.2.into_pin(),
        leds.3.into_pin(),
        leds.4.into_pin(),
        leds.5.into_pin(),
        leds.6.into_pin(),
        leds.7.into_pin(),
    )
}

fn setup_btns(gpio: &mut Gpio) -> BtnBank {
    let btns = gpio.take_all_btns().unwrap();
    BtnBank::new(
        btns.0.into_pin(),
        btns.1.into_pin(),
        btns.2.into_pin(),
        btns.3.into_pin(),
        btns.4.into_pin(),
        btns.5.into_pin(),
    )
}

fn setup_audio_streamer(dac: AudioDac) -> AudioStreamer<audio::Initialized> {
    let streamer = AudioStreamer::new_mono(dac);
    streamer.initialize()
}
