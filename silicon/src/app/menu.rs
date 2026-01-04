use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::Text,
};
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_sdmmc::{LfnBuffer, ShortFileName};
use heapless::{String, Vec};
use silicon_hal::display;

use crate::{
    VoidUnwrap,
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
    let (mut display, mut leds, mut btns, mut sd_state, audio_streamer, title_select) = match state
    {
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

    const FILES_PER_PAGE: usize = 6;
    const MAX_NAME_LENGTH: usize = 20;
    let mut files: PaginatedEntries<FILES_PER_PAGE, MAX_NAME_LENGTH>;
    // Load the 6 first file names from the SDCard
    files = match get_file_names(&mut sd_state, 0) {
        Ok(names) => names,
        Err(_) => return None, // TODO: Handle error appropriately
    };
    leds.led1.set_high(); // Indicate loading complete

    let mut cursor = 0;

    'select: loop {
        leds.led2.set_low();
        leds.led3.set_low();

        // Refresh file list if needed
        if cursor >= files.offset + files.len() || cursor < files.offset {
            files = match get_file_names(&mut sd_state, (cursor / FILES_PER_PAGE) * FILES_PER_PAGE)
            {
                Ok(names) => names,
                Err(_) => return None, // TODO: Handle error appropriately
            };
        }

        // Display the file menu
        render_menu(&mut display, &files.entries, cursor - files.offset);
        leds.led2.set_high(); // Indicate display complete

        // Handle button inputs
        'inputs: loop {
            // BTN3 = Up / Prev
            if btns.btn3.is_high().void_unwrap() {
                cursor = cursor.saturating_sub(1);
                break 'inputs;
            }
            // BTN4 = Down / Next
            if btns.btn4.is_high().void_unwrap() {
                cursor = (cursor + 1).min(files.total - 1);
                break 'inputs;
            }
            // BTN5 = Back
            if btns.btn5.is_high().void_unwrap() {
                // Normally, go back = select the parent directory
                // It only works if we are in title select mode, and the parent is then the first entry
                if title_select {
                    cursor = 0;
                    break 'select;
                }
            }
            // BTN6 = Select / OK
            if btns.btn6.is_high().void_unwrap() {
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
    let new_dir = match sd_state
        .mng
        .open_dir(sd_state.pwd, selected_file.short_name)
    {
        Ok(dir) => dir,
        Err(_) => return None, // TODO: Handle error appropriately
    };
    leds.led5.set_high(); // Indicate directory opened

    // Close the current directory
    sd_state.mng.close_dir(sd_state.pwd).void_unwrap(); // This is important to avoid running out of dir handles
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
        if files.get(cursor).unwrap().short_name.base_name() == PARENT_DIR {
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

        display.clear(Rgb565::BLACK);
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

/// Represents a paginated list of file entries.
///
/// This structure holds a vector of entries, along with pagination metadata.
///
/// const N: usize - Maximum number of entries per page.
/// const M: usize - Maximum length of each entry's display name.
///
/// Fields:
/// * `entries` - A vector of file entries.
/// * `offset` - The current start offset in the overall list of entries.
/// * `total` - The total number of entries available.
struct PaginatedEntries<const N: usize, const M: usize> {
    entries: Vec<Entry<M>, N>,
    offset: usize,
    total: usize,
}

impl<const N: usize, const M: usize> PaginatedEntries<N, M> {
    /// Get the number of entries currently stored.
    fn len(&self) -> usize {
        self.entries.len()
    }

    /// Retrieve an entry by its paginated index.
    ///
    /// # Arguments
    /// * `index` - The index of the entry to retrieve.
    ///
    /// # Returns
    /// * `Option<&Entry<M>>` - A reference to the entry if it exists, or None.
    fn get(&self, index: usize) -> Option<&Entry<M>> {
        self.entries.get(index - self.offset)
    }
}

/// Represents a file entry with both short and long names.
///
/// If a long name is not available, the display name can be derived from the short name.
struct Entry<const M: usize> {
    pub short_name: ShortFileName,
    pub display_name: String<M>,
}

/// Retrieve file names from the SD card directory, starting from a specified offset.
///
/// # Arguments
/// * `sd_state` - The current state of the SD card directory.
/// * `offset` - The number of entries to skip before collecting album names.
/// # Returns
/// * `Result<Vec<(ShortFileName, String<M>), N>, ()>` - A vector of file names (SFN, LFN) or an error.
fn get_file_names<const N: usize, const M: usize>(
    sd_state: &mut SdDirState,
    offset: usize,
) -> Result<PaginatedEntries<N, M>, ()> {
    let mut file_names = Vec::new();

    let mng = &mut sd_state.mng;
    let root = sd_state.pwd;

    let mut count = 0;
    let mut lfn_storage = [0u8; M];
    let mut lfn_buf = LfnBuffer::new(&mut lfn_storage);
    match mng.iterate_dir_lfn(root, &mut lfn_buf, |entry, lfn| {
        if entry.attributes.is_directory() {
            const SYS_DIR: [u8; 7] = [b'_', b'_', b'S', b'Y', b'S', b'_', b'_'];
            const CURRENT_DIR: [u8; 1] = [b'.'];

            // Ensure it is not the __SYS__ directory, nor .
            if entry.name.base_name() == SYS_DIR || entry.name.base_name() == CURRENT_DIR {
                return; // Skip hidden directories
            }

            if count >= offset && file_names.len() < N {
                // If a long file name exists, use it; otherwise, convert the short file name
                let mut lfn_name = String::<M>::new();
                if let Some(lfn_str) = lfn {
                    let _ = lfn_name.push_str(lfn_str); // Ignore push errors for simplicity
                } else {
                    let sfn_str = sfn_to_str(&entry.name);
                    let _ = lfn_name.push_str(&sfn_str); // Ignore push errors for simplicity
                }
                let _ = file_names.push(Entry {
                    short_name: entry.name,
                    display_name: lfn_name,
                }); // Ignore push errors for simplicity    
            }
            count += 1;
        }
    }) {
        Ok(_) => Ok(PaginatedEntries {
            entries: file_names,
            offset,
            total: count,
        }),
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
    rst.push_str(base).void_unwrap();
    if !sfn.extension().is_empty() {
        let ext = unsafe { core::str::from_utf8_unchecked(sfn.extension()) };
        rst.push('.').void_unwrap();
        rst.push_str(ext).void_unwrap();
    }
    rst
}

fn render_menu<const N: usize, const M: usize>(
    display: &mut OledDisplay<display::Initialized>,
    entries: &Vec<Entry<M>, N>,
    selected_index: usize,
) {
    // Clear the display
    display.clear(Rgb565::BLACK);

    // Define text style
    const TEXT_STYLE: MonoTextStyle<Rgb565> = MonoTextStyle::new(
        &embedded_graphics::mono_font::ascii::FONT_10X20,
        Rgb565::WHITE,
    );

    // Render the selection cursor
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
    for (i, entry) in entries.iter().enumerate() {
        let position = Point::new(0, 16 + (i as i32) * 20); // 20 pixels per line - 16 for the baseline
        let text = Text::new(&entry.display_name, position, TEXT_STYLE);
        text.draw(display).void_unwrap();
    }
}
