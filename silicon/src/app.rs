//! Music player application
//!
//! This module contains the main application logic for a simple music player, and its state management.

use silicon_hal::{Peripheral, audio, display};
use crate::peripheral::{LedBank, BtnBank, OledDisplay, SdCard, AudioStreamer};

mod boot;
mod load;

/// Represents the different states of the music player application.
pub enum AppState {
    Booting(BootingState),
    Loading(LoadingState),
    Menu,
    Playing,
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
            AppState::Menu => {
                // Menu state logic would go here
            }
            AppState::Playing => {
                // Playing state logic would go here
            }
        }
        None
    }

    /// Create a new AppState in the Booting state.
    pub fn new(peripherals: Peripheral) -> Self {
        AppState::Booting(BootingState { peripherals })
    }
}
