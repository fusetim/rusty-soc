use crate::app::AppState;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::{FONT_10X20};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle, StrokeAlignment};
use embedded_graphics::text::renderer::CharacterStyle;
use embedded_graphics::text::{Text, TextStyle, TextStyleBuilder};
use embedded_hal::digital::OutputPin;
use embedded_sdmmc::{Mode, TimeSource, Timestamp, VolumeIdx, VolumeManager};
use silicon_hal::delay::INTR_DELAY;
use embedded_hal::delay::DelayNs;
use silicon_hal::pac::audio_streamer;

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
        let mut audio_streamer = loading_state.audio_streamer;

        leds.set_all_low();

        let mut mng = VolumeManager::new(sdcard, ZeroTimeSource);
        leds.led1.set_high();
        let volume = mng.open_raw_volume(VolumeIdx(0)).unwrap();
        leds.led2.set_high();
        let root = mng.open_root_dir(volume).unwrap();
        leds.led3.set_high();

        let mut names = [0u8; 12*8];
        let mut name_index = [0usize; 8];
        let mut count = 0;
        mng.iterate_dir(root, |entry| {
            // Just iterate through the directory entries
            if count % 2 == 0 {
                leds.led6.set_low();
            } else {
                leds.led6.set_high();
            }

            if count < 7 {
                // Store the name of the first 8 entries
                let base = entry.name.base_name();
                let ext = entry.name.extension();
                let mut name = [0u8; 32];
                let mut len = base.len();
                name[..len].copy_from_slice(base);
                if !ext.is_empty() {
                    name[len] = b'.';
                    len += 1;
                    name[len..len + ext.len()].copy_from_slice(ext);
                    len += ext.len();
                }
                let start = name_index[count];
                names[start..start + len].copy_from_slice(&name[0..len]);
                name_index[count + 1] = start + len;
            }
            count += 1;
        }).unwrap();
        leds.led4.set_high();

        // Print the names to the display
        let character_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);

        // Play the music file one after another
        let mut playing = 0;
        let mut audio_samples = [0u8; 512];
        while playing < count {
            leds.set_all_low();
            // Display the list of songs and the one currently playing
            display.clear(Rgb565::BLACK);
        
            // Currently playing song get a black purple rectangle behind it
            let rect = Rectangle::new(
                Point::new(0, (playing as i32) * 20),
                Size::new(128, 20),
            );
            let style = PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::CSS_DARK_MAGENTA)
                .build();
            rect.into_styled(style).draw(&mut display);

            // Draw all the names
            for i in 0..count {
                let start = name_index[i];
                let end = name_index[i + 1];
                let name_str = core::str::from_utf8(&names[start..end]).unwrap_or("?!");
                Text::new(name_str, Point::new(0, 16+(i as i32) * 20), character_style)
                    .draw(&mut display)
                    .unwrap();
            }

            // Play the song using the Audio Streamer
            let start = name_index[playing];
            let end = name_index[playing + 1];
            if let Ok(name) = core::str::from_utf8(&names[start..end]) {
                leds.led4.set_high();
                if let Ok(file) = mng.open_file_in_dir(root, name, Mode::ReadOnly) {
                    leds.led5.set_high();
                    loop {
                        let read = mng.read(file, &mut audio_samples).unwrap();
                        if read == 0 {
                            break;
                        }
                        leds.led6.set_high();
                        // Write samples to audio streamer
                        let mut written = 0;
                        while written < read {
                            written += audio_streamer.write_samples(&audio_samples[written..read]);
                        }
                        leds.led6.set_low();
                    }
                }
            }
            playing += 1;
        }

        loop {}
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