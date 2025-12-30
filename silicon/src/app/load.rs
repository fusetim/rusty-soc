use crate::{app::{AppState, SdDirState}, display::BinWrapDrawTarget, fs::VolumeManager, peripheral::{LedBank, OledDisplay}};
use embedded_graphics::{image::{Image, ImageDrawable, ImageDrawableExt as _, ImageRaw}, pixelcolor::{BinaryColor, Rgb565}, prelude::{Drawable, DrawTarget, Point, RgbColor, WebColors}};
use embedded_hal::digital::OutputPin;
use embedded_sdmmc::{Mode, VolumeIdx};
use embedded_sdmmc::VolumeManager as _;
use silicon_hal::{delay::{DelayNs, INTR_DELAY}, display};
use crate::fs::ZeroTimeSource;

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
        let mut display = loading_state.display;
        let mut leds = loading_state.leds;
        let sdcard = loading_state.sdcard;
        let audio_streamer = loading_state.audio_streamer;
        let mut delay = INTR_DELAY;

        leds.set_all_low();

        let mng = VolumeManager::new(sdcard, ZeroTimeSource);
        leds.led1.set_high();
        let volume = mng.open_raw_volume(VolumeIdx(0)).unwrap();
        leds.led2.set_high();
        let root = mng.open_root_dir(volume).unwrap();
        leds.led3.set_high();

        let mut sd_state = SdDirState {
            mng,
            volume,
            pwd: root,
        };

        delay.delay_ms(1000);

        // Display welcome animation
        let mut glyph_data= [0; 512];
        read_asset_data(&mut sd_state, "hi.raw", 0, &mut glyph_data, &mut leds).unwrap();
        leds.led4.set_high();

        welcome_animation(&mut display, &glyph_data, delay);
        leds.led5.set_high();

        delay.delay_ms(500);

        leds.set_all_low();
        return Some(AppState::AlbumMenu(crate::app::MenuState {
            leds,
            btns: loading_state.btns,
            display,
            audio_streamer,
            sd_state,
        }));
    }
    None
}

/// Display a welcome animation on the OLED display.
/// 
/// # Arguments
/// 
/// * `display` - The OLED display to draw the animation on.
/// * `glyph_data` - The glyph data to use for the animation. (64x64 pixels on/off - 512 bytes)
fn welcome_animation(display: &mut OledDisplay<display::Initialized>, glyph_data: &[u8; 512], mut delay: impl DelayNs) {
    let raw_image = ImageRaw::<BinaryColor>::new(glyph_data, 64);


    // Clear the display
    display.clear(Rgb565::BLACK);

    // Animation loop
    // Total: 2s / 64 "frames" = ~31.25ms per frame (~30 FPS)
    for k in 0..64 {
        // Pixel under the k-th column are drawn in WHITE
        {
            let white_pos = Point::new(32, 32);
            let white_area = embedded_graphics::primitives::Rectangle::new(
                embedded_graphics::prelude::Point::new(0, 0),
                embedded_graphics::prelude::Size::new(64, 64 - k),
            );
            let mut white_display = BinWrapDrawTarget::new(Rgb565::CSS_WHITE, Rgb565::BLACK, display);
            let sub_image = raw_image.sub_image(&white_area);
            let white_img = Image::new(&sub_image, white_pos);
            white_img.draw(&mut white_display);
        }

        // Pixel above and equal to the k-th column are drawn in PURPLE
        {
            let purple_pos = Point::new(32, 96 - (k as i32));
            let purple_area = embedded_graphics::primitives::Rectangle::new(
                embedded_graphics::prelude::Point::new(0, 64 - (k as i32)),
                embedded_graphics::prelude::Size::new(64, 64),
            );
            let mut purple_display = BinWrapDrawTarget::new(Rgb565::CSS_PURPLE, Rgb565::BLACK, display);
            let sub_image = raw_image.sub_image(&purple_area);
            let purple_image = Image::new(&sub_image, purple_pos);
            purple_image.draw(&mut purple_display);
        }
        delay.delay_ms(33);
    }

    // Clear the display
    display.clear(Rgb565::CSS_MAGENTA);
}

/// Read asset data from the SD card.
/// 
/// # Arguments
/// * `root` - The current SD card directory state.
/// * `path` - The path to the asset file.
/// * `offset` - The offset within the file to start reading from.
/// * `buf` - The buffer to read the data into.
/// # Returns
/// * `Result<usize, ()>` - The number of bytes read on success, or an error on failure.
pub fn read_asset_data(root: &mut SdDirState, path: &str, offset: u32, buf: &mut [u8], debug_leds: &mut LedBank) -> Result<usize, ()> {
    debug_leds.set_all_high();
    let mnr: &mut VolumeManager = &mut root.mng;
    if let Ok(sys_dir) = mnr.open_dir(root.pwd.clone(), "__SYS__") {
        debug_leds.led6.set_low();
        if let Ok(file) = mnr.open_file_in_dir(sys_dir, path, Mode::ReadOnly) {
            debug_leds.led5.set_low();
            if let Ok(_) = mnr.file_seek_from_start(file.clone(), offset) {
                debug_leds.led4.set_low();
                if let Ok(bytes_read) = mnr.read(file, buf) {
                    debug_leds.led3.set_low();
                    if let Ok(_) = mnr.close_file(file) {
                        debug_leds.led2.set_low();
                        if let Ok(_) = mnr.close_dir(sys_dir) {
                            // Successfully read data
                            debug_leds.set_all_low();
                            return Ok(bytes_read);
                        }
                    }
                }
            }
        }
    } 
    Err(())
}