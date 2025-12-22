#![no_std]
#![no_main]
use core::convert::Infallible;

use embedded_hal::digital::{InputPin, OutputPin, PinState};
use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_sdmmc::{Mode, SdCard, TimeSource, Timestamp, VolumeIdx, VolumeManager};
use silicon_hal::audio::AudioStreamer;
use silicon_hal::delay::IntrDelay;
use silicon_hal::display::DisplayPeripheral;
use silicon_hal::gpio::{Pin, self as _};
use silicon_hal::gpio::never_bank::NeverPin;
use silicon_hal::gpio::spi_sdcard_bank::SpiSdCs;
use silicon_hal::spi::{Spi, Spi0};
use silicon_hal::{audio, dac};
use silicon_hal::{
    delay::{DelayNs, INTR_DELAY},
    gpio::IntoPin as _,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

/*  SOFT SPI
type TheSdCard = SdCard<
    ExclusiveDevice<SpiSoft<SpiSdClk, SpiSdMosi, SpiSdMiso, IntrDelay>, Pin<SpiSdCs>, IntrDelay>,
    IntrDelay,
>;
*/
// HARD SPI
type TheSdCard = SdCard<ExclusiveDevice<Spi<Spi0, IntrDelay>, Pin<SpiSdCs>, IntrDelay>, IntrDelay>;

/*
#[silicon_hal::entry]
fn main() -> ! {
    let mut peripherals = silicon_hal::init();

    let mut leds: [&mut dyn OutputPin<Error = Infallible>; 8] = {
        let (led0, led1, led2, led3, led4, led5, led6, led7) =
            peripherals.gpio.take_all_leds().unwrap();
        [
            &mut led0.into_pin(),
            &mut led1.into_pin(),
            &mut led2.into_pin(),
            &mut led3.into_pin(),
            &mut led4.into_pin(),
            &mut led5.into_pin(),
            &mut led6.into_pin(),
            &mut led7.into_pin(),
        ]
    };

    // SPI
    let mut sd_cs = peripherals.gpio.take_spi_sd_cs().unwrap().into_pin();
    /* SOFT SPI
    let spi_sd: silicon_hal::gpio::SpiPins<SpiSdMosi, SpiSdClk, SpiSdMiso> = peripherals.gpio.take_spi_sd().unwrap();
    let spi_soft = SpiSoft::new(spi_sd, INTR_DELAY);
    sd_cs.set_high();
    delay_ms(250);
    let spi_sd = ExclusiveDevice::new(spi_soft, sd_cs, INTR_DELAY).unwrap();
    */
    // HARD SPI
    let mut spi_hard = Spi::new(peripherals.spi0, INTR_DELAY);
    sd_cs.set_high();
    spi_hard.initialize();
    delay_ms(250);
    let spi_sd = ExclusiveDevice::new(spi_hard, sd_cs, INTR_DELAY).unwrap();
    let mut sdcard: TheSdCard = SdCard::new(spi_sd, INTR_DELAY);

    // Audio
    let dac = peripherals.dac;
    let mut audio_streamer = AudioStreamer::new_mono(dac).initialize();

    loop {
        // Init the LEDs
        reset_leds(&mut leds);

        leds[0].set_high();
        delay_ms(1000);
        leds[0].set_low();

        // Init the SD card
        let capacity = {
            if let Ok(capacity) = sdcard.num_bytes() {
                capacity
            } else {
                // SD card init failed, skip the rest of the loop
                // Toggle LED7 to indicate error
                leds[7].set_high();
                delay_ms(500);
                leds[7].set_low();
                continue;
            }
        };

        // Blink LEDs based on capacity (in GB)
        {
            let gb = capacity / (1024 * 1024 * 1024);
            set_leds(
                &mut leds,
                &[
                    gb & 0x01 != 0,
                    gb & 0x02 != 0,
                    gb & 0x04 != 0,
                    gb & 0x08 != 0,
                    gb & 0x10 != 0,
                    gb & 0x20 != 0,
                    gb & 0x40 != 0,
                    gb & 0x80 != 0,
                ],
            );
            delay_ms(1000);
            reset_leds(&mut leds);
        }

        delay_ms(500);

        // Output a test tone via DAC
        //test_tone(&mut dac, &mut leds);

        // Loading a music file from SD card and playing it via DAC
        sdcard = play_file(&mut audio_streamer, &mut leds, sdcard);
    }
}
*/

// TESTING - DISPLAY ONLY
#[silicon_hal::entry]
fn main() -> ! {
    let mut peripherals = silicon_hal::init();
    let mut leds: [&mut dyn OutputPin<Error = Infallible>; 8] = {
        let (led0, led1, led2, led3, led4, led5, led6, led7) =
            peripherals.gpio.take_all_leds().unwrap();
        [
            &mut led0.into_pin(),
            &mut led1.into_pin(),
            &mut led2.into_pin(),
            &mut led3.into_pin(),
            &mut led4.into_pin(),
            &mut led5.into_pin(),
            &mut led6.into_pin(),
            &mut led7.into_pin(),
        ]
    };

    // Get the button pins
    let mut btn1 = {
        let btn1 = peripherals.gpio.take_btn1().unwrap();
        btn1.into_pin()
    };

    // Get the OLED pins
    let (oled_cs, oled_dc, oled_rst) = {
        let (oled_cs, _, _, oled_dc, oled_rst) = peripherals.gpio.take_oled().unwrap();
        (oled_cs.into_pin(), oled_dc.into_pin(), oled_rst.into_pin())
    };
    // Initialize the SPI
    let mut oled_spi = Spi::new(peripherals.spi1, INTR_DELAY);
    let oled_spi_cs = Pin::new_output(NeverPin(PinState::Low));
    oled_spi.initialize();
    delay_ms(250);
    let oled_spi_device = ExclusiveDevice::new(
        oled_spi,
        oled_spi_cs,
        INTR_DELAY,
    ).unwrap();
    // Create the display peripheral
    let mut display = DisplayPeripheral::new(
        oled_spi_device,
        oled_cs,
        oled_dc,
        oled_rst,
        INTR_DELAY.clone(),
    );

    loop {
        for i in 0..8 {
            reset_leds(&mut leds);
            leds[i].set_high();
            delay_ms(250);
        }

        reset_leds(&mut leds);

        // Initialize the display
        let _display = display.initialize().unwrap();
        leds[0].set_high();
        delay_ms(1000);

        // Wait for BTN1 
        while btn1.is_low().unwrap() {}

        leds[0].set_low();
        display = _display.disable();

    }
}

pub fn to_pin_state(state: bool) -> PinState {
    if state { PinState::High } else { PinState::Low }
}

#[inline(always)]
pub fn delay_ms(ms: u32) {
    #[allow(const_item_mutation)]
    INTR_DELAY.delay_ms(ms);
}

pub fn reset_leds(led_pins: &mut [&mut dyn OutputPin<Error = Infallible>]) {
    set_leds(&mut led_pins[..], &[false; 8]);
}

pub fn set_leds(led_pins: &mut [&mut dyn OutputPin<Error = Infallible>], states: &[bool]) {
    for (pin, &state) in led_pins.iter_mut().zip(states.iter()) {
        if state {
            pin.set_high();
        } else {
            pin.set_low();
        }
    }
}

struct ZeroTimeSource;
impl TimeSource for ZeroTimeSource {
    #[inline(always)]
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 10,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

#[inline(never)]
pub fn play_file(
    streamer: &mut AudioStreamer<audio::Mono, audio::Initialized>,
    leds: &mut [&mut dyn OutputPin<Error = Infallible>],
    sdcard: TheSdCard,
) -> TheSdCard {
    // Open the fat filesystem
    let volume_mgr: VolumeManager<_, _, 1, 1, 1> =
        VolumeManager::new_with_limits(sdcard, ZeroTimeSource, 0);
    let Ok(volume0) = volume_mgr.open_raw_volume(VolumeIdx(0)) else {
        // Failed to open volume, indicate error via LED7
        leds[7].set_high();
        delay_ms(500);
        leds[7].set_low();
        return volume_mgr.free().0;
    };
    // Volume opened successfully
    leds[1].set_high();

    let Ok(root_dir) = volume_mgr.open_root_dir(volume0) else {
        // Failed to open root directory, indicate error via LED7
        leds[6].set_high();
        delay_ms(500);
        leds[6].set_low();
        return volume_mgr.free().0;
    };

    // Root directory opened successfully
    leds[2].set_high();

    let Ok(my_file) = volume_mgr.open_file_in_dir(root_dir, "music.raw", Mode::ReadOnly) else {
        // Failed to open file, indicate error via LED7
        leds[5].set_high();
        delay_ms(500);
        leds[5].set_low();
        volume_mgr.close_dir(root_dir);
        return volume_mgr.free().0;
    };

    leds[3].set_high();

    // Read to the end of the file
    let mut buffer = [0u8; 512];
    loop {
        let Ok(bytes_read) = volume_mgr.read(my_file.clone(), &mut buffer) else {
            // Read error, indicate via LED7
            leds[7].set_high();
            leds[6].set_high();
            delay_ms(500);
            leds[7].set_low();
            leds[6].set_low();
            break;
        };
        if bytes_read == 0 {
            // End of file reached
            break;
        }

        let mut toggle_led5 = false;
        let mut tx = 0;
        // Process the read data (e.g., send to DAC)
        while (tx < bytes_read) {
            tx += streamer.write_samples(&buffer[tx..bytes_read]);
            toggle_led5 = !toggle_led5;
            leds[5].set_state(to_pin_state(toggle_led5));
        }
    }

    let _ = streamer.write_sample(0);
    reset_leds(leds);
    leds[4].set_high();

    delay_ms(1000);
    volume_mgr.close_dir(root_dir);
    volume_mgr.free().0
}
