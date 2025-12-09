#![no_std]
#![no_main]
use embedded_hal::digital::OutputPin;
use silicon_hal::delay::{DelayNs, SocDelay};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const DELAY: SocDelay = SocDelay;

#[unsafe(no_mangle)]
fn main() -> ! {
    let peripherals = silicon_hal::init();
    let gpio = peripherals.gpio;
    let mut led_second = gpio.led0;
    let mut led_minute = gpio.led1;
    let mut led_wait = gpio.led7;

    // Indicate startup by lighting up LED7 for a short time
    led_wait.set_high().unwrap();
    delay_ms(2000);
    led_wait.set_low().unwrap();

    let mut seconds = 0;
    loop {
        if seconds > 60 {
            led_minute.set_high().unwrap();
        } else {
            led_minute.set_low().unwrap();
        }
        if (seconds % 2) == 0 {
            led_second.set_high().unwrap();
        } else {
            led_second.set_low().unwrap();
        }
        delay_ms(1000);
        seconds += 1;
    }
}

#[inline(always)]
pub fn delay_ms(ms: u32) {
    #[allow(const_item_mutation)]
    DELAY.delay_ms(ms);
}
