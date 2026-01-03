use crate::{
    VoidUnwrap, app::{AppState, SdDirState}, delay_ms, fs::VolumeManager, peripheral::{LedBank, OledDisplay}
};
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_sdmmc::{Mode, RawDirectory};
use silicon_hal::display;

/// Run the Playing logic.
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
pub fn run_playing(state: AppState) -> Option<AppState> {
    if let AppState::Playing(playing_state) = state {
        let mut display = playing_state.display;
        let mut leds = playing_state.leds;
        let mut btns = playing_state.btns;
        let mut sd_state = playing_state.sd_state;
        let mut audio_streamer = playing_state.audio_streamer;

        leds.set_all_low();

        // Draw the cover art for the current title
        display_cover_art(&mut display, &mut sd_state.mng, sd_state.pwd, &mut leds);

        delay_ms(1000);
        leds.set_all_low();

        // Open the music.raw file inside the current directory
        let mng = &mut sd_state.mng;
        let audio_file = match mng.open_file_in_dir(sd_state.pwd, "music.raw", Mode::ReadOnly) {
            Ok(f) => f,
            Err(_) => return None, // TODO: Handle error appropriately
        };
        leds.led1.set_high();

        // Start streaming audio
        let mut paused = false;
        let mut buffer = [0u8; 512];
        loop {
            // Handle inputs
            {
                let pause_btn = btns.btn4.is_high().unwrap_or(false);
                let back_btn = btns.btn3.is_high().unwrap_or(false);
                let forward_btn = btns.btn6.is_high().unwrap_or(false);
                let backward_btn = btns.btn5.is_high().unwrap_or(false);
                // Check if the pause button is pressed
                if pause_btn {
                    paused = !paused;
                    // Simple debounce - wait 300ms
                    delay_ms(300);
                }
                // Check if the back button is pressed
                if back_btn {
                    // Stop playback and go back to title menu
                    break;
                }
                // Check if the forward button is pressed
                if forward_btn {
                    // TODO: Skip will fail if we are near EOF - handle that case (https://github.com/fusetim/rusty-soc/issues/1)
                    let _ = mng.file_seek_from_current(audio_file, 938*512); // Skip forward ~10s (assuming 48kHz mono 8-bit)
                    // Simple debounce - wait 300ms
                    delay_ms(300);
                } else if backward_btn { // Check if the backward button is pressed
                    // TODO: Going backward will fail if we are near the start of the file - handle that case (https://github.com/fusetim/rusty-soc/issues/1)
                    let _ = mng.file_seek_from_current(audio_file, -(938*512)); // Skip backward ~10s
                    // Simple debounce - wait 300ms
                    delay_ms(300);
                }
            }

            if paused {
                continue; // Skip reading and writing audio while paused
            }

            // Read audio data from the file
            {
                leds.led2.set_low();
                if let Ok(bytes_read) = mng.read(audio_file, &mut buffer) {
                    leds.led2.set_high();
                    if bytes_read == 0 {
                        break; // End of file
                    }
                    leds.led3.set_high();
                    let mut written = 0;
                    while written < bytes_read {
                        written += audio_streamer.write_samples(&buffer[written..bytes_read]);
                    }
                    leds.led3.set_low();
                } else {
                    // Error reading file - stop playback
                    break;
                }
            }
        }

        // EOF reached, stop audio streamer
        mng.close_file(audio_file).void_unwrap();
        leds.led4.set_high();

        // Go back to the title menu
        // - Open the parent directory (album dir)
        let parent_dir = mng.open_dir(sd_state.pwd, "..").void_unwrap();
        leds.led5.set_high();
        // - Close the current directory
        mng.close_dir(sd_state.pwd).void_unwrap();
        leds.led6.set_high();
        // - Update SdDirState to point to the parent directory
        let sd_state = SdDirState {
            mng: sd_state.mng,
            volume: sd_state.volume,
            pwd: parent_dir,
        };

        // - Byebye
        return Some(AppState::TitleMenu(crate::app::MenuState {
            leds,
            btns,
            display,
            audio_streamer,
            sd_state,
        }));
    }
    None
}

/// Display the cover art for the current title.
/// 
/// This function attempts to open and display a "art.raw" file from the given title directory.
/// It should be encoded as a 80x80 RGB565 row-major raw image.
/// 
/// If the file is not found, ... TODO
pub fn display_cover_art(display: &mut OledDisplay<display::Initialized>, mng: &mut VolumeManager, title_dir: RawDirectory, debug_leds: &mut LedBank) {
    const X_START: u8 = 24;
    const Y_START: u8 = 0;

    // Try to open the art.raw file
    let art_file = match mng.open_file_in_dir(title_dir, "art.raw", Mode::ReadOnly) {
        Ok(f) => f,
        Err(_) => return, // TODO: Handle error appropriately
    };

    debug_leds.led1.set_high();

    // Ensure the file is the expected size (80*80*2 = 12800 bytes)
    if let Ok(file_size) = mng.file_length(art_file) {
        debug_leds.led2.set_high();
        if file_size != 12800 {
            let _ = mng.close_file(art_file);
            return; // Invalid file size
        }
    } else {
        let _ = mng.close_file(art_file);
        return; // Unable to get file size (probably an SD card error)
    }

    debug_leds.led3.set_high();

    // Our stack is very small, so we have to read and draw the image in chunks
    // We do so by reading 2560 bytes at a time (80 pixels * 16 rows * 2 bytes per pixel)
    // Note: PCCM(80,512) = 2560 bytes
    let mut img_buffer = [0u8; 2560];
    for row in 0..16 {
        if let Ok(bytes_read) = mng.read(art_file, &mut img_buffer) {
            if bytes_read != 2560 {
                break; // Unexpected EOF
            }
            // Draw the 16 rows we just read
            let x0 = X_START;
            let y0 = Y_START + row * 16;
            let x1 = x0 + 80 - 1;
            let y1 = y0 + 16 - 1;
            display.draw_area_from_slice(x0, y0, x1, y1, &img_buffer);
        } else {
            break; // Read error
        }
    }

    debug_leds.led4.set_high();

    // Close the art file
    mng.close_file(art_file).void_unwrap();
}