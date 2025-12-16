#![no_std]
#![no_main]
use core::convert::Infallible;

use embedded_hal::digital::{ErrorType, OutputPin, PinState};
use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_sdmmc::{Mode, SdCard, TimeSource, Timestamp, VolumeIdx, VolumeManager, sdcard};
use silicon_hal::dac;
use silicon_hal::delay::IntrDelay;
use silicon_hal::gpio::Pin;
use silicon_hal::gpio::spi_sdcard_bank::{SpiSdClk, SpiSdCs, SpiSdMiso, SpiSdMosi};
use silicon_hal::spi::SpiSoft;
use silicon_hal::{
    delay::{DelayNs, INTR_DELAY},
    gpio::IntoPin as _,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

type TheSdCard = SdCard<
    ExclusiveDevice<SpiSoft<SpiSdClk, SpiSdMosi, SpiSdMiso, IntrDelay>, Pin<SpiSdCs>, IntrDelay>,
    IntrDelay,
>;

#[silicon_hal::entry]
fn main() -> ! {
    let mut peripherals = silicon_hal::init();

    let mut dac = peripherals.dac;
    let (mut led0, mut led1, mut led2, mut led3, mut led4, mut led5, mut led6, mut led7) = {
        let (led0, led1, led2, led3, led4, led5, led6, led7) =
            peripherals.gpio.take_all_leds().unwrap();
        (
            led0.into_pin(),
            led1.into_pin(),
            led2.into_pin(),
            led3.into_pin(),
            led4.into_pin(),
            led5.into_pin(),
            led6.into_pin(),
            led7.into_pin(),
        )
    };
    let spi_sd = peripherals.gpio.take_spi_sd().unwrap();
    let mut sd_cs = peripherals.gpio.take_spi_sd_cs().unwrap().into_pin();
    let spi_soft = SpiSoft::new(spi_sd, INTR_DELAY);

    sd_cs.set_high();
    delay_ms(250);
    let spi_sd = ExclusiveDevice::new(spi_soft, sd_cs, INTR_DELAY).unwrap();

    let mut sdcard: TheSdCard = SdCard::new(spi_sd, INTR_DELAY);

    let mut leds: [&mut dyn OutputPin<Error = Infallible>; 8] = [
        &mut led0, &mut led1, &mut led2, &mut led3, &mut led4, &mut led5, &mut led6, &mut led7,
    ];

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
        test_tone(&mut dac, &mut leds);

        // Loading a music file from SD card and playing it via DAC
        sdcard = play_file(&mut dac, &mut leds, sdcard);
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

pub fn test_tone(dac: &mut dac::AudioDac, leds: &mut [&mut dyn OutputPin<Error = Infallible>]) {
    for k in 0..64 {
        reset_leds(leds);
        leds[k & 0x07].set_high();
        for i in 0..=8 {
            unsafe { core::arch::asm!("lb x0, 18(x0)") };
            dac.write_stereo_sample(8 * i as u8, 255 - 8 * i as u8);
            unsafe { core::arch::asm!("lb x0, 19(x0)") };
            INTR_DELAY.delay_ms(1);
            unsafe { core::arch::asm!("lb x0, 20(x0)") };
        }
    }
    dac.write_stereo_sample(0 as u8, 0 as u8);
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
    dac: &mut dac::AudioDac,
    leds: &mut [&mut dyn OutputPin<Error = Infallible>],
    sdcard: TheSdCard,
) -> TheSdCard {
    // Open the fat filesystem
    let volume_mgr = VolumeManager::new(sdcard, ZeroTimeSource);
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
        leds[7].set_high();
        delay_ms(500);
        leds[7].set_low();
        return volume_mgr.free().0;
    };

    // Root directory opened successfully
    leds[2].set_high();

    /*
    let Ok(mut my_file) = volume_mgr.open_file_in_dir(root_dir, "MY_FILE.TXT", Mode::ReadOnly) else {
        // Failed to open file, indicate error via LED7
        leds[7].set_high();
        delay_ms(500);
        leds[7].set_low();
        return volume_mgr.free().0;
    };

    leds[3].set_high();*/

    //while !my_file.is_eof() {
    //    let mut buffer = [0u8; 32];
    //    let Ok(num_read) = my_file.read(&mut buffer) else {
    //        // Read error, indicate via LED7
    //        leds[7].set_high();
    //        delay_ms(500);
    //        leds[7].set_low();
    //        break;
    //    };
    //    for b in &buffer[0..num_read] {
    //        reset_leds(leds);
    //        delay_ms(250);
    //        set_leds(leds, &[b & 0x01 != 0, b & 0x02 != 0, b & 0x04 != 0, b & 0x08 != 0, b & 0x10 != 0, b & 0x20 != 0, b & 0x40 != 0, b & 0x80 != 0]);
    //        delay_ms(250);
    //    }
    //    reset_leds(leds);
    //}
    leds[4].set_high();

    volume_mgr.free().0
}
