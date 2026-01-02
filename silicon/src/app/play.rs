use crate::{
    VoidUnwrap,
    app::{AppState, SdDirState},
    delay_ms,
};
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_sdmmc::Mode;

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
        let display = playing_state.display;
        let mut leds = playing_state.leds;
        let mut btns = playing_state.btns;
        let mut sd_state = playing_state.sd_state;
        let mut audio_streamer = playing_state.audio_streamer;

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
