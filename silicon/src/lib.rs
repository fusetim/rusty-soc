#![no_std]
#![no_main]
use embedded_hal::digital::OutputPin;
use silicon_hal::delay::{DelayNs, INTR_DELAY};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}


#[unsafe(no_mangle)]
fn main() -> ! {
    use silicon_hal::gpio::{Pin};

    let mut peripherals = silicon_hal::init();

    let mut led0 = Pin::new_output(peripherals.gpio.take_led0().unwrap());
    let mut led1 = Pin::new_output(peripherals.gpio.take_led1().unwrap());

    let mut count : u8 = 0;
    loop {
        if count % 2 == 0 {
            led0.set_high();
        } else {
            led0.set_low();
        }
        if count % 4 == 0 {
            led1.set_high();
        } else {
            //led1.set_low();
        }
        count = count.wrapping_add(1);
        delay_ms(1000);
    }
}

#[inline(always)]
pub fn delay_ms(ms: u32) {
    #[allow(const_item_mutation)]
    INTR_DELAY.delay_ms(ms);
}
