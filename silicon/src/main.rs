#![no_std]
#![no_main]

use embedded_hal::digital::OutputPin;
use silicon_hal::gpio::IntoPin as _;

mod app;
mod display;
mod fs;
mod peripheral;
mod utils;

pub use utils::{VoidUnwrap, delay_ms};

#[panic_handler]
fn __panic(_info: &core::panic::PanicInfo) -> ! {
    // In case of panic, just loop indefinitely
    // Also try to get a led to light up or something -- unsafe but useful for debugging
    let mut gpio = unsafe { silicon_hal::gpio::Gpio::steal() };
    let mut led0 = gpio.take_led0().unwrap().into_pin();
    let mut led7 = gpio.take_led7().unwrap().into_pin();
    loop {
        led0.set_low();
        led7.set_high();
        delay_ms(1000);
        led7.set_low();
        led0.set_high();
        delay_ms(1000);
    }
}

/// The main entry point of the program.
#[silicon_hal::entry]
fn main() -> ! {
    let peripheral = silicon_hal::init();
    let mut app_state = app::AppState::new(peripheral);

    loop {
        if let Some(new_state) = app_state.run() {
            app_state = new_state;
        } else {
            break;
        }
    }

    loop {}
}
