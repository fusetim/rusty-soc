use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::Text,
};
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_sdmmc::ShortFileName;
use heapless::{String, Vec};
use silicon_hal::{delay::INTR_DELAY, display};

use crate::{
    app::{AppState, MenuState, PlayingState, SdDirState},
    peripheral::OledDisplay,
};

/// Run the Album Menu logic.
///
/// This function initializes the necessary peripherals and transitions the application
/// from the Loading state to the Album Menu state.
///
//// # Arguments
///
/// * `state` - The current application state, expected to be in the Loading state.
///
/// # Returns
///
/// * `Option<AppState>` - The new application state after loading, or None if an error occurred.
pub fn run_menu(state: AppState) -> Option<AppState> {
    let (mut display, mut leds, mut btns, mut sd_state, mut audio_streamer, title_select) =
        match state {
            AppState::AlbumMenu(menu_state) => (
                menu_state.display,
                menu_state.leds,
                menu_state.btns,
                menu_state.sd_state,
                menu_state.audio_streamer,
                false,
            ),
            AppState::TitleMenu(menu_state) => (
                menu_state.display,
                menu_state.leds,
                menu_state.btns,
                menu_state.sd_state,
                menu_state.audio_streamer,
                true,
            ),
            _ => return None, // Invalid state transition
        };

    // Reset LEDs & Display
    leds.set_all_low();
    display.clear(Rgb565::BLACK);

    let files: Vec<ShortFileName, 8>;
    // Load the 8 first file names from the SDCard
    files = match get_file_names::<8>(&mut sd_state, 0) {
        Ok(names) => names,
        Err(_) => return None, // TODO: Handle error appropriately
    };
    leds.led1.set_high(); // Indicate loading complete

    let mut cursor = 0;

    'select: loop {
        leds.led2.set_low();
        leds.led3.set_low();

        // Display the file menu
        let mut file_names = Vec::<String<12>, 8>::new();
        for file in files.iter() {
            let name_str = sfn_to_str(file);
            let _ = file_names.push(name_str); // Ignore push errors for simplicity
        }
        render_menu(&mut display, &file_names, cursor);
        leds.led2.set_high(); // Indicate display complete

        // Handle button inputs
        'inputs: loop {
            // BTN3 = Up / Prev
            if btns.btn3.is_high().unwrap() {
                cursor = cursor.saturating_sub(1);
                break 'inputs;
            }
            // BTN4 = Down / Next
            if btns.btn4.is_high().unwrap() {
                cursor = (cursor + 1).min(files.len() - 1);
                break 'inputs;
            }
            // BTN6 = Select / OK
            if btns.btn6.is_high().unwrap() {
                // Select album
                break 'select;
            }
        }
        leds.led3.set_high(); // Indicate button processed
    }

    leds.led4.set_high(); // Indicate album selected

    // Transition to the next state

    // Open the selected album directory
    let selected_file = files.get(cursor).unwrap();
    let new_dir = match sd_state.mng.open_dir(sd_state.pwd, selected_file) {
        Ok(dir) => dir,
        Err(_) => return None, // TODO: Handle error appropriately
    };
    leds.led5.set_high(); // Indicate directory opened

    // Close the current directory
    sd_state.mng.close_dir(sd_state.pwd).unwrap(); // This is important to avoid running out of dir handles
    leds.led6.set_high(); // Indicate directory closed

    if !title_select {
        Some(AppState::TitleMenu(MenuState {
            leds,
            btns,
            display,
            audio_streamer,
            sd_state: SdDirState {
                mng: sd_state.mng,
                volume: sd_state.volume,
                pwd: new_dir,
            },
        }))
    } else {
        // Selected a title
        // or have we? (check if the selected entry is not .. (the parent))
        const PARENT_DIR: [u8; 2] = [b'.', b'.'];
        if files.get(cursor).unwrap().base_name() == PARENT_DIR {
            // Go back to album menu
            return Some(AppState::AlbumMenu(MenuState {
                leds,
                btns,
                display,
                audio_streamer,
                sd_state: SdDirState {
                    mng: sd_state.mng,
                    volume: sd_state.volume,
                    pwd: new_dir,
                },
            }));
        }

        display.clear(Rgb565::CSS_PURPLE);
        Some(AppState::Playing(PlayingState {
            leds,
            btns,
            display,
            audio_streamer,
            sd_state: SdDirState {
                mng: sd_state.mng,
                volume: sd_state.volume,
                pwd: new_dir,
            },
        }))
    }
}

/// Retrieve file names from the SD card directory, starting from a specified offset.
///
/// # Arguments
/// * `sd_state` - The current state of the SD card directory.
/// * `offset` - The number of entries to skip before collecting album names.
/// # Returns
/// * `Result<Vec<ShortFileName, N>, ()>` - A vector of file names or an error.
fn get_file_names<const N: usize>(
    sd_state: &mut SdDirState,
    offset: usize,
) -> Result<Vec<ShortFileName, N>, ()> {
    let mut file_names = Vec::new();

    let mng = &mut sd_state.mng;
    let root = sd_state.pwd;

    let mut count = 0;
    match mng.iterate_dir(root, |entry| {
        if count < offset {
            count += 1;
            return; // Skip entries until we reach the offset
        }
        if file_names.len() >= N {
            return; // Stop iteration when we have enough file names
        }
        if entry.attributes.is_directory() {
            const SYS_DIR: [u8; 7] = [b'_', b'_', b'S', b'Y', b'S', b'_', b'_'];
            const CURRENT_DIR: [u8; 1] = [b'.'];
            // Ensure it is not the __SYS__ directory, nor .
            if entry.name.base_name() == SYS_DIR || entry.name.base_name() == CURRENT_DIR {
                return;
            }
            let _ = file_names.push(entry.name); // Ignore push errors for simplicity
        }
        count += 1;
    }) {
        Ok(_) => Ok(file_names),
        // TODO: Handle errors appropriately
        Err(_) => return Err(()),
    }
}

/// Convert a ShortFileName to a string representation.
///
/// Safety: This function assumes that the ShortFileName contains valid ASCII characters, which is generally true for FAT file systems.
fn sfn_to_str(sfn: &ShortFileName) -> String<12> {
    let mut rst = String::new();
    let base = unsafe { core::str::from_utf8_unchecked(sfn.base_name()) };
    rst.push_str(base).unwrap();
    if !sfn.extension().is_empty() {
        let ext = unsafe { core::str::from_utf8_unchecked(sfn.extension()) };
        rst.push('.').unwrap();
        rst.push_str(ext).unwrap();
    }
    rst
}

fn render_menu<const N: usize>(
    display: &mut OledDisplay<display::Initialized>,
    album_names: &Vec<String<12>, N>,
    selected_index: usize,
) {
    // Clear the display
    display.clear(Rgb565::BLACK);

    // Define text style
    let text_style = MonoTextStyle::new(
        &embedded_graphics::mono_font::ascii::FONT_10X20,
        Rgb565::WHITE,
    );

    // Renter the selection cursor
    {
        let cursor_position = Point::new(0, (selected_index as i32) * 20);
        let cursor_rect = Rectangle::new(cursor_position, Size::new(128, 20));
        cursor_rect
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .fill_color(Rgb565::CSS_PURPLE)
                    .build(),
            )
            .draw(display);
    }

    // Render each album name
    for (i, name) in album_names.iter().enumerate() {
        let position = Point::new(0, 16 + (i as i32) * 20); // 20 pixels per line - 16 for the baseline
        let text = Text::new(name, position, text_style);
        text.draw(display).unwrap();
    }
}
