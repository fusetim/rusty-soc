use crate::app::AppState;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::{FONT_10X20};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle, StrokeAlignment};
use embedded_graphics::text::renderer::CharacterStyle;
use embedded_graphics::text::{Text, TextStyle, TextStyleBuilder};
use embedded_hal::digital::OutputPin;
use embedded_sdmmc::{LfnBuffer, TimeSource, Timestamp, VolumeIdx, VolumeManager};
use silicon_hal::delay::INTR_DELAY;
use embedded_hal::delay::DelayNs;

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

/// Run the loading state logic.
///
/// This function initializes the necessary peripherals and transitions the application
/// from the Loading state to the Menu state.
///
//// # Arguments
///
/// * `state` - The current application state, expected to be in the Loading state.
///
/// # Returns
///
/// * `Option<AppState>` - The new application state after loading, or None if an error occurred.
pub fn run_loading(state: AppState) -> Option<AppState> {
    if let AppState::Loading(loading_state) = state {
        // TODO - Load the needed data from the SDCard, update display, etc.

        // For now, just display a loading text message on the OLED display
        let mut display = loading_state.display;
        let mut leds = loading_state.leds;
        let mut sdcard = loading_state.sdcard;

        if let Ok(cap) = sdcard.num_bytes() {
            let mut buf = [0u8; 10];
            let cap_str = format_sd_capacity_bytes(cap, &mut buf);

            let mut carstyle = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
            carstyle.background_color = Some(Rgb565::BLACK);

            display.clear(Rgb565::BLACK).ok()?;

            Text::new(cap_str, Point::new(0, 20), carstyle)
                .draw(&mut display);

            // Open first volume
            let time_source = ZeroTimeSource;
            let mut mgr = VolumeManager::new(sdcard, time_source);
            let indexZero= VolumeIdx(0);
            if let Ok(vol) =  mgr.open_volume(indexZero) {
                // Successfully opened volume
                leds.set_all_low();
                leds.led4.set_high(); // Indicate success
                INTR_DELAY.delay_ms(1000);

                // Open root directory, list files, etc.
                if let Ok(root_dir) = vol.open_root_dir() {
                    // Successfully opened root directory
                    leds.set_all_low();
                    leds.led5.set_high(); // Indicate success
                    INTR_DELAY.delay_ms(1000);

                    // List files, load data, etc.
                    let mut entries = [['.' as u8; 10]; 6];
                    let mut count = 0;
                    let mut buf = [0u8; 32];
                    let mut lfn_buffer = LfnBuffer::new(&mut buf);
                    root_dir.iterate_dir_lfn(&mut lfn_buffer, |entry, name| {
                        if count < entries.len() {
                            if let Some(name) = name {
                                entries[count].copy_from_slice(name.as_bytes());
                                count += 1;
                            }
                        }
                    });

                    // Successfully opened root directory
                    leds.set_all_low();
                    leds.led6.set_high(); // Indicate success
                    INTR_DELAY.delay_ms(1000);

                    if count > 0 {
                        for i in 0..count {
                            let name = core::str::from_utf8(&entries[i]).unwrap_or("???????");
                            let y = 40 + (i as i32) * 20;
                            if y > 120 {
                                break;
                            }
                            Text::new(name, Point::new(0, y), carstyle)
                                .draw(&mut display);
                        }
                    }

                    leds.led2.set_high(); // Indicate success
                    INTR_DELAY.delay_ms(1000);
                }
            }


            loop {}
        } else {
            leds.set_all_low();
            leds.led1.set_high(); // Indicate error
            INTR_DELAY.delay_ms(1000);
        };
    }
    None
}

pub fn format_loading_percentage<'a>(percentage: u8, buf: &'a mut [u8; 4]) -> &'a str {
    let s = buf;
    s[0] = ((percentage / 100) % 10 + b'0') as u8;
    s[1] = ((percentage / 10) % 10 + b'0') as u8;
    s[2] = (percentage % 10 + b'0') as u8;
    s[3] = b'%';

    // Safety: We only write ASCII digits and '%', so this is always valid UTF-8.
    if percentage >= 100 {
        unsafe { core::str::from_utf8_unchecked(&s[0..4]) }
    } else if percentage >= 10 {
        unsafe { core::str::from_utf8_unchecked(&s[1..4]) }
    } else {
        unsafe { core::str::from_utf8_unchecked(&s[2..4]) }
    }
}

pub fn format_sd_capacity_bytes<'a>(bytes: u64, buf: &'a mut [u8; 10]) -> &'a str {
    let mb = bytes / 1_000_000;
    let s = buf;
    let mut len = 0;

    if mb >= 100_000 {
        s[len] = (((mb / 100_000) % 10) as u8 + b'0') as u8;
        len += 1;
    }
    if mb >= 10_000 {
        s[len] = (((mb / 10_000) % 10) as u8 + b'0') as u8;
        len += 1;
    }
    if mb >= 1_000 {
        s[len] = (((mb / 1_000) % 10) as u8 + b'0') as u8;
        len += 1;

        s[len] = b' ';
        len += 1;
    }
    if mb >= 100 {
        s[len] = (((mb / 100) % 10) as u8 + b'0') as u8;
        len += 1;
    }
    if mb >= 10 {
        s[len] = (((mb / 10) % 10) as u8 + b'0') as u8;
        len += 1;
    }
    s[len] = ((mb % 10) as u8 + b'0') as u8;
    len += 1;

    // Add " MB" suffix
    s[len] = b' ';
    len += 1;
    s[len] = b'M';
    len += 1;
    s[len] = b'B';
    len += 1;

    // Safety: We only write ASCII digits and letters, so this is always valid UTF-8.
    unsafe { core::str::from_utf8_unchecked(&s[0..len]) }
}