use crate::app::AppState;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::{FONT_10X20};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle, StrokeAlignment};
use embedded_graphics::text::renderer::CharacterStyle;
use embedded_graphics::text::{Text, TextStyle};
use embedded_hal::digital::OutputPin;

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

        // Loading bar
        let outer_rect_origin = Point::new(34, 98);
        let outer_rect_size = Size::new(60, 12);
        let outer_rect = Rectangle::new(outer_rect_origin, outer_rect_size);
        let outer_rect_style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::WHITE)
            .stroke_width(1)
            .stroke_alignment(StrokeAlignment::Outside)
            .build();

        let inner_rect_origin = Point::new(36, 100);
        let inner_rect_max_size = Size::new(56, 8);
        let inner_rect_style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::CSS_PURPLE)
            .build();

        let mut character_style = MonoTextStyle::new(&FONT_10X20, Rgb565::CSS_PURPLE);
        character_style.set_background_color(Some(Rgb565::BLACK));

        // Debug info
        let mut sd_cap = None;


        // The current progress percentage
        let mut progress = 0;
        'running: loop {
            // Update LED status
            leds.set_all_states([
                progress >= 12,
                progress >= 25,
                progress >= 37,
                progress >= 50,
                progress >= 62,
                progress >= 75,
                progress >= 87,
                progress >= 100,
            ]);

            // Clear display and draw loading elements, when progress is 0
            if progress == 0 {
                display.clear(Rgb565::BLACK);

                // Draw the outer rectangle
                outer_rect.into_styled(outer_rect_style).draw(&mut display);
            }

            // Draw centered text.
            {
                let mut textbuf = [0u8; 4];
                let text = format_loading_percentage(progress, &mut textbuf);
                Text::with_text_style(
                    &text,
                    display.bounding_box().center(),
                    character_style,
                    TextStyle::default(),
                )
                .draw(&mut display);
            }

            // Debug info
            if let Some(cap) = sd_cap {
                // Display SDCard capacity
                let mut cap_buf = [0u8; 10]; // Enough for "XXX XXX MB"
                let cap_str = format_sd_capacity_bytes(cap, &mut cap_buf);
                Text::with_text_style(
                    &cap_str,
                    Point::new(20, 20),
                    character_style,
                    TextStyle::default(),
                ).draw(&mut display);
            }

            // Draw the loading bar
            {
                let filled_width = (inner_rect_max_size.width * progress as u32) / 100;
                let inner_rect = Rectangle::new(
                    inner_rect_origin,
                    Size::new(filled_width, inner_rect_max_size.height),
                );
                inner_rect.into_styled(inner_rect_style).draw(&mut display);
            }

            // Do progress
            if progress == 10 {
                // Initialize the SDCard by getting its size
                if let Ok(cap) = sdcard.num_bytes() {
                    sd_cap = Some(cap);
                } else {
                    // SDCard error
                    leds.set_all_low();
                    leds.led3.set_high(); // SDCard error
                    break 'running;
                }
            }
            
            progress += 1;
            if progress > 100 {
                break 'running;
            }
        }
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