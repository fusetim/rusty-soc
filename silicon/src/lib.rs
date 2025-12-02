#![no_std]
#![no_main]
use silicon_hal::led::SOC_LED;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
fn main() {
    /*
    SOC_LED.set(0b0000_0000); // Turn off all LEDs

    loop {
        for k in 0..8 {
            SOC_LED.set(1 << k); // Light up one LED at a time
            delay();
        }
    }
    */

    let peripherals = silicon_hal::init();

    let mut iter_count = 0; 
    loop {
        // Blinking pattern 3 times
        for _ in 0..3 {
            // Moving light to center
            for k in 0..4 {
                let pattern = (1 << k) | (1 << (7 - k));
                SOC_LED.set(pattern);
                delay_ms(100);
            }

            // Moving light back outwards
            for k in (0..4).rev() {
                let pattern = (1 << k) | (1 << (7 - k));
                SOC_LED.set(pattern);
                delay_ms(100);
            }
        }

        // Blink 3 times all LEDs
        for _ in 0..3 {
            SOC_LED.set(0b1111_1111); // All LEDs on
            delay_ms(200);
            SOC_LED.set(0b0000_0000); // All LEDs off
            delay_ms(200);
        }

        // Blink 2 times the iteration count in binary
        for _ in 0..2 {
            SOC_LED.set(iter_count & 0xFF); // Show iteration count on LEDs
            delay_ms(1000);
            SOC_LED.set(0b0000_0000); // All LEDs off
            delay_ms(500);
        }
        iter_count += 1;
    }
}

fn delay_ms(millis: u32) {
    for _ in 0..millis {
        delay_1ms();
    }
}

fn delay_1ms() {
    for _ in 0..100 {
        // Busy-wait loop for approximately 1ms delay
        unsafe { core::arch::asm!("nop") };
    }
}