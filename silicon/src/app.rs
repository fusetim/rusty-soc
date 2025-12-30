//! Music player application
//!
//! This module contains the main application logic for a simple music player, and its state management.

use silicon_hal::{Peripheral, audio, display};
use crate::{fs::{VolumeManager, RawVolume, RawDirectory}, peripheral::{AudioStreamer, BtnBank, LedBank, OledDisplay, SdCard}};

mod boot;
mod load;
mod menu;
mod play;

/// Represents the different states of the music player application.
pub enum AppState {
    Booting(BootingState),
    Loading(LoadingState),
    AlbumMenu(MenuState),
    TitleMenu(MenuState),
    Playing(PlayingState),
}

pub struct BootingState {
    peripherals: Peripheral,
}

pub struct LoadingState {
    leds: LedBank,
    btns: BtnBank,
    display: OledDisplay<display::Initialized>,
    sdcard: SdCard,
    audio_streamer: AudioStreamer<audio::Initialized>,
}

pub struct MenuState {
    leds: LedBank,
    btns: BtnBank,
    display: OledDisplay<display::Initialized>,
    audio_streamer: AudioStreamer<audio::Initialized>,
    sd_state: SdDirState,
}

pub struct PlayingState {
    leds: LedBank,
    btns: BtnBank,
    display: OledDisplay<display::Initialized>,
    audio_streamer: AudioStreamer<audio::Initialized>,
    sd_state: SdDirState,
}

/// Represents the state of the SD card directory.
pub struct SdDirState {
    pub mng: VolumeManager,
    pub volume: RawVolume,
    pub pwd: RawDirectory,
}

impl AppState {
    /// Run the application logic based on the current state.
    /// 
    /// Returns the new application state after running the logic.
    /// None if you want to stop the application.
    pub fn run(self) -> Option<Self> {
        match self {
            AppState::Booting(_) => {
                return boot::run_booting(self);
            }
            AppState::Loading(_) => {
                return load::run_loading(self);
            }
            AppState::AlbumMenu(_) => {
                // Menu state logic would go here
                return menu::run_menu(self);
            }
            AppState::TitleMenu(_) => {
                // Menu state logic would go here
                return menu::run_menu(self);
            }
            AppState::Playing(_) => {
                // Playing state logic would go here
                return play::run_playing(self);
            }
        }
        None
    }

    /// Create a new AppState in the Booting state.
    pub fn new(peripherals: Peripheral) -> Self {
        AppState::Booting(BootingState { peripherals })
    }
}
