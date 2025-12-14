#![no_std]
#![no_main]
use embedded_hal::delay;
use embedded_hal::digital::{InputPin, OutputPin, PinState, StatefulOutputPin};
use embedded_hal::spi::{SpiBus, SpiDevice};
use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_sdmmc::SdCard;
use silicon_hal::gpio::led_bank::Led5;
use silicon_hal::spi::SpiSoft;
use silicon_hal::{
    delay::{DelayNs, INTR_DELAY},
    gpio::IntoPin as _,
    spi::Spi,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
fn main() -> ! {
    let mut peripherals = silicon_hal::init();

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

    let mut sdcard = SdCard::new(spi_sd, INTR_DELAY);

    loop {
        // Toggle LEDs to indicate activity
        led0.set_high();
        delay_ms(1000);
        led0.set_low();
        led1.set_low();
        led2.set_low();
        led3.set_low();
        led4.set_low();
        led5.set_low();
        led6.set_low();
        led7.set_low();
        delay_ms(1000);

        // Read the SD card capacity
        match sdcard.num_bytes() {
            Ok(cap) => {
                let cap_gb = cap / (1024 * 1024 * 1024);
                led1.set_state(to_pin_state(cap_gb & 0x01 != 0));
                led2.set_state(to_pin_state(cap_gb & 0x02 != 0));
                led3.set_state(to_pin_state(cap_gb & 0x04 != 0));
                led4.set_state(to_pin_state(cap_gb & 0x08 != 0));
                led5.set_state(to_pin_state(cap_gb & 0x10 != 0));
                led6.set_state(to_pin_state(cap_gb & 0x20 != 0));
                led7.set_low();
            }
            Err(err) => match err {
                embedded_sdmmc::SdCardError::RegisterReadError => {
                    led1.set_high();
                }
                embedded_sdmmc::SdCardError::CrcError(_, _) => {
                    led2.set_high();
                }
                embedded_sdmmc::SdCardError::ReadError => {
                    led3.set_high();
                }
                embedded_sdmmc::SdCardError::WriteError => {
                    led4.set_high();
                }
                embedded_sdmmc::SdCardError::BadState => {
                    led5.set_high();
                }
                embedded_sdmmc::SdCardError::CardNotFound => {
                    led6.set_high();
                }
                embedded_sdmmc::SdCardError::GpioError => {
                    led7.set_high();
                }
                _ => {}
            },
        }

        // Toggle LEDs to indicate activity
        led0.set_high();
        delay_ms(1000);
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
