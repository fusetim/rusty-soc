#![no_std]
#![no_main]
use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};
use silicon_hal::{
    delay::{DelayNs, INTR_DELAY},
    gpio::IntoPin as _,
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
    let (mut btn1, mut btn2, mut btn3, mut btn4, mut btn5, mut btn6) = {
        let (btn1, btn2, btn3, btn4, btn5, btn6) = peripherals.gpio.take_all_btns().unwrap();
        (
            btn1.into_pin(),
            btn2.into_pin(),
            btn3.into_pin(),
            btn4.into_pin(),
            btn5.into_pin(),
            btn6.into_pin(),
        )
    };

    let mut count: u8 = 0;
    loop {
        // Read buttons and set corresponding LEDs
        if btn1.is_high().unwrap() {
            led1.set_high().unwrap();
        } else {
            led1.set_low().unwrap();
        }
        if btn2.is_high().unwrap() {
            led2.set_high().unwrap();
        } else {
            led2.set_low().unwrap();
        }
        if btn3.is_high().unwrap() {
            led3.set_high().unwrap();
        } else {
            led3.set_low().unwrap();
        }
        if btn4.is_high().unwrap() {
            led4.set_high().unwrap();
        } else {
            led4.set_low().unwrap();
        }
        if btn5.is_high().unwrap() {
            led5.set_high().unwrap();
        } else {
            led5.set_low().unwrap();
        }
        if btn6.is_high().unwrap() {
            led6.set_high().unwrap();
        } else {
            led6.set_low().unwrap();
        }

        // Toggle led0/led7 every 8 cycles
        if count % 8 == 0 {
            led0.toggle().unwrap();
        } else if count % 8 == 4 {
            led7.toggle().unwrap();
        }

        // Simple delay
        delay_ms(125);

        // Increment count (not used for now)
        count = count.wrapping_add(1);
    }
}

#[inline(always)]
pub fn delay_ms(ms: u32) {
    #[allow(const_item_mutation)]
    INTR_DELAY.delay_ms(ms);
}
