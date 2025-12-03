#![no_std]
#![no_main]
use silicon_hal::gpio::GpioPeripheral;
use silicon_hal::delay::{SocDelay, DelayNs};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const DELAY : SocDelay = SocDelay;

#[unsafe(no_mangle)]
fn main() -> ! {
    let peripherals = silicon_hal::init();
    let gpio = peripherals.gpio;

    let mut iter_count = 0; 
    loop {
        // Blinking pattern 3 times
        for _ in 0..3 {
            // Moving light to center
            for k in 0..4 {
                let pattern = (1 << k) | (1 << (7 - k));
                gpio.set_leds(pattern);
                delay_ms(100);
            }

            // Moving light back outwards
            for k in (0..4).rev() {
                let pattern = (1 << k) | (1 << (7 - k));
                gpio.set_leds(pattern);
                delay_ms(100);
            }
        }

        // Blink 3 times all LEDs
        for _ in 0..3 {
            gpio.set_leds(0b1111_1111); // All LEDs on
            delay_ms(200);
            gpio.set_leds(0b0000_0000); // All LEDs off
            delay_ms(200);
        }

        // Blink 2 times the iteration count in binary
        for _ in 0..2 {
            gpio.set_leds(iter_count & 0xFF); // Show iteration count on LEDs
            delay_ms(1000);
            gpio.set_leds(0b0000_0000); // All LEDs off
            delay_ms(500);
        }
        iter_count += 1;
    }
}

#[inline(always)]
pub fn delay_ms(ms: u32) {
    #[allow(const_item_mutation)]
    DELAY.delay_ms(ms);
}