use crate::{
    VoidUnwrap,
    app::{AppState, SdDirState},
    delay_ms,
    display::BinWrapDrawTarget,
    fs::VolumeManager,
    peripheral::OledDisplay,
};
use embedded_graphics::{
    image::Image,
    mono_font::{self, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::Rgb565,
    prelude::{Drawable, Point, RgbColor as _},
    text::Text,
};
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_sdmmc::{Mode, RawDirectory};
use heapless::format;
use silicon_hal::display;

const AUDIO_SAMPLE_RATE: usize = 48000; // 48kHz
const AUDIO_CHANNELS: usize = 1; // Mono
const AUDIO_BIT_DEPTH: usize = 8; // 8-bit
const AUDIO_DATA_RATE: usize = AUDIO_SAMPLE_RATE * AUDIO_CHANNELS * (AUDIO_BIT_DEPTH / 8); // Bytes per second

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
        display_cover_art(&mut display, &mut sd_state.mng, sd_state.pwd);

        // Open the music.raw file inside the current directory
        let mng = &mut sd_state.mng;
        let audio_file = match mng.open_file_in_dir(sd_state.pwd, "music.raw", Mode::ReadOnly) {
            Ok(f) => f,
            Err(_) => return None, // TODO: Handle error appropriately
        };
        // Get the size of the audio file
        let size = mng.file_length(audio_file).void_unwrap();
        let duration = size / (AUDIO_DATA_RATE as u32); // Duration in seconds
        leds.led1.set_high();

        // Start streaming audio
        let mut snd_vol = 8; // Volume level 1-8/8
        let mut led_vol_timeout = 0; // Timeout counter for volume LED indication
        let mut paused = false;
        let mut buffer = [0u8; 512];
        let mut cycle = 0; // Count loop iterations (it allows us to reevaluate the progress every ~500ms / 47 iters)
        loop {
            // Handle inputs
            {
                let pause_btn = btns.btn4.is_high().unwrap_or(false);
                let back_btn = btns.btn3.is_high().unwrap_or(false);
                let forward_btn = btns.btn6.is_high().unwrap_or(false);
                let backward_btn = btns.btn5.is_high().unwrap_or(false);
                let vol_up_btn = btns.btn2.is_high().unwrap_or(false);
                let vol_down_btn = btns.btn1.is_high().unwrap_or(false);
                // Check if the pause button is pressed
                if pause_btn {
                    paused = !paused;
                    // Simple debounce - wait 300ms
                    delay_ms(300);
                    cycle = 0; // Force immediate progress update after unpausing
                }
                // Check if the back button is pressed
                if back_btn {
                    // Stop playback and go back to title menu
                    break;
                }
                // Check if the forward button is pressed
                if forward_btn {
                    // TODO: Skip will fail if we are near EOF - handle that case (https://github.com/fusetim/rusty-soc/issues/1)
                    let _ = mng.file_seek_from_current(audio_file, 938 * 512); // Skip forward ~10s (assuming 48kHz mono 8-bit)
                    // Simple debounce - wait 300ms
                    delay_ms(300);
                    cycle = 0; // Force immediate progress update after unpausing
                } else if backward_btn {
                    // Check if the backward button is pressed
                    // TODO: Going backward will fail if we are near the start of the file - handle that case (https://github.com/fusetim/rusty-soc/issues/1)
                    let _ = mng.file_seek_from_current(audio_file, -(938 * 512)); // Skip backward ~10s
                    // Simple debounce - wait 300ms
                    delay_ms(300);
                    cycle = 0; // Force immediate progress update after unpausing
                }
                // Check volume up button
                if led_vol_timeout >= 24*7 {
                    // Debouncing for volume buttons (ignore if < 500ms since last volume change)
                } else if vol_up_btn {
                    if snd_vol < 8 {
                        snd_vol += 1;
                    }
                    led_vol_timeout = 48 * 4; // Show volume level for 4 seconds (assuming 48kHz sample rate and 512-byte reads)
                } else if vol_down_btn { // Check volume down button
                    if snd_vol > 1 {
                        snd_vol -= 1;
                    }
                    led_vol_timeout = 48 * 4; // Show volume level for 4 seconds (assuming 48kHz sample rate and 512-byte reads)
                }
            }

            // Display LED volume level indication (if needed)
            if led_vol_timeout > 0 {
                led_vol_timeout -= 1;
                if led_vol_timeout == 0 {
                    leds.set_all_low();
                } else {
                    leds.set_all_states([
                        snd_vol >= 1,
                        snd_vol >= 2,
                        snd_vol >= 3,
                        snd_vol >= 4,
                        snd_vol >= 5,
                        snd_vol >= 6,
                        snd_vol >= 7,
                        snd_vol >= 8,
                    ]);
                }
            }

            // Read audio data from the file
            if !paused {
                if led_vol_timeout == 0 {  leds.led2.set_low(); }
                if let Ok(bytes_read) = mng.read(audio_file, &mut buffer) {
                    if led_vol_timeout == 0 {  leds.led2.set_high(); }
                    if bytes_read == 0 {
                        break; // End of file
                    }
                    if led_vol_timeout == 0 {  leds.led3.set_high(); }
                    // Apply volume adjustment (simple scaling)
                    for sample in buffer[..bytes_read].iter_mut() {
                        let scaled = *sample >> (8 - snd_vol); // Scale down to 0-8/8 volume
                        *sample = scaled as u8;
                    }

                    let mut written = 0;
                    while written < bytes_read {
                        written += audio_streamer.write_samples(&buffer[written..bytes_read]);
                    }
                    if led_vol_timeout == 0 {  leds.led3.set_low(); }
                } else {
                    // Error reading file - stop playback
                    break;
                }
            }

            if cycle == 0 {
                // Update track progress display
                if let Ok(current_pos) = mng.file_offset(audio_file) {
                    let elapsed = current_pos / (AUDIO_DATA_RATE as u32); // Elapsed time in seconds
                    display_track_progress(&mut display, elapsed, duration, !paused);
                }
            }
            cycle += 1;
            if cycle >= 47 {
                // ~500ms at 48kHz mono 8-bit with 512-byte reads
                cycle = 0;
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
/// It should be encoded as a 128x128 RGB565 row-major raw image.
///
/// If the file is not found, ... TODO
pub fn display_cover_art(
    display: &mut OledDisplay<display::Initialized>,
    mng: &mut VolumeManager,
    title_dir: RawDirectory,
) {
    const X_START: u8 = 0;
    const Y_START: u8 = 0;

    // Try to open the art.raw file
    let art_file = match mng.open_file_in_dir(title_dir, "art.raw", Mode::ReadOnly) {
        Ok(f) => f,
        Err(_) => return, // TODO: Handle error appropriately
    };

    //debug_leds.led1.set_high();

    // Ensure the file is the expected size (128*128*2 = 32768 bytes)
    if let Ok(file_size) = mng.file_length(art_file) {
        //debug_leds.led2.set_high();
        if file_size < 32768 {
            let _ = mng.close_file(art_file);
            return; // Invalid file size
        }
    } else {
        let _ = mng.close_file(art_file);
        return; // Unable to get file size (probably an SD card error)
    }

    //debug_leds.led3.set_high();

    // Our stack is very small, so we have to read and draw the image in chunks
    // We do so by reading 2048 bytes at a time (128 pixels * 8 rows * 2 bytes per pixel)
    // Note: PCCM(128,512) = 2048 bytes
    let mut img_buffer = [0u8; 2048];
    for row in 0..16 {
        if let Ok(bytes_read) = mng.read(art_file, &mut img_buffer) {
            if bytes_read != 2048 {
                break; // Unexpected EOF
            }
            // Draw the 8 rows we just read
            let x0 = X_START;
            let y0 = Y_START + row * 8;
            let x1 = x0 + 128 - 1;
            let y1 = y0 + 8 - 1;
            display.draw_area_from_slice(x0, y0, x1, y1, &img_buffer);
        } else {
            break; // Read error
        }
    }

    //debug_leds.led4.set_high();

    // Close the art file
    mng.close_file(art_file).void_unwrap();
}

/// Display the track progress on the OLED display.
///
/// # Arguments
///
/// * `display` - The OLED display to draw on.
/// * `elapsed` - The elapsed time in seconds.
/// * `total` - The total duration of the track in seconds.
/// * `is_playing` - Whether the track is currently playing or paused.
pub fn display_track_progress(
    display: &mut OledDisplay<display::Initialized>,
    elapsed: u32,
    total: u32,
    is_playing: bool,
) {
    // TODO: Implement a progress bar on the OLED display
    // For now, we just print the elapsed time in seconds
    let progress = elapsed * 8 / total;
    let elapsed_str = format!(5; "{:02}:{:02}", elapsed / 60, elapsed % 60).void_unwrap();
    let total_str = format!(5; "{:02}:{:02}", total / 60, total % 60).void_unwrap();
    let bar_str = match progress {
        0 => "*-------",
        1 => "-*------",
        2 => "--*-----",
        3 => "---*----",
        4 => "----*---",
        5 => "-----*--",
        6 => "------*-",
        7 => "-------*",
        _ => "********",
    };
    let progress_str = format!(20; "{} {} {}", elapsed_str, bar_str, total_str).void_unwrap();

    const CHARACTER_STYLE: MonoTextStyle<Rgb565> = MonoTextStyleBuilder::new()
        .font(&mono_font::ascii::FONT_6X10)
        .text_color(Rgb565::WHITE)
        .background_color(Rgb565::BLACK)
        .build();
    let pos = Point::new(8, 128 - 2); // Bottom-left corner
    let text = Text::new(&progress_str, pos, CHARACTER_STYLE);
    let _ = text.draw(display);

    // Draw play/pause icon
    let icon_pos = Point::new(0, 128 - 8); // Bottom-left
    let icon = if is_playing {
        glyph::PLAY_GLYPH
    } else {
        glyph::PAUSE_GLYPH
    };
    let icon_image = Image::new(&icon, icon_pos);
    let mut white_display = BinWrapDrawTarget::new(Rgb565::WHITE, Rgb565::BLACK, display);
    let _ = icon_image.draw(&mut white_display);
}

mod glyph {
    //! Additional 8x8 glyphs for the Playing screen.
    use embedded_graphics::{image::ImageRaw, pixelcolor::BinaryColor};

    mod raw {
        pub const PLAY: [u8; 8] = [
            0b11000000, 0b11100000, 0b11110000, 0b11111000, 0b11110000, 0b11100000, 0b11000000,
            0b00000000,
        ];

        pub const PAUSE: [u8; 8] = [
            0b11011000, 0b11011000, 0b11011000, 0b11011000, 0b11011000, 0b11011000, 0b11011000,
            0b00000000,
        ];

        pub const CURSOR: [u8; 8] = [
            0b00011000, 0b00111100, 0b01111110, 0b11111111, 0b01111110, 0b00111100, 0b00011000,
            0b00000000,
        ];
    }

    pub const PLAY_GLYPH: ImageRaw<BinaryColor> = ImageRaw::new(&raw::PLAY, 8);
    pub const PAUSE_GLYPH: ImageRaw<BinaryColor> = ImageRaw::new(&raw::PAUSE, 8);
    pub const CURSOR_GLYPH: ImageRaw<BinaryColor> = ImageRaw::new(&raw::CURSOR, 8);
}
