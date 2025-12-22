//! Audio Streamer module
//! This module provides an interface for streaming audio data to the DAC peripheral.
//! It supports stereo and mono audio modes.
//!
//! The AudioStreamer struct manages a ring buffer for audio samples and provides methods to write samples to the DAC.

use silicon_pac::audio_streamer::control::W;

use crate::pac;
use crate::{dac::AudioDac, typesafe::Sealed};
/// Marker trait for AudioStreamer states (Uninitialized and Initialized).
pub trait AudioStreamerState: Sealed {}
pub struct Uninitialized;
pub struct Initialized {
    // Write ID for synchronization
    wid: u8,
}
impl Sealed for Uninitialized {}
impl Sealed for Initialized {}
impl AudioStreamerState for Uninitialized {}
impl AudioStreamerState for Initialized {}

/// Marker trait for audio modes (Stereo and Mono).
pub trait AudioMode: Sealed {}
pub struct Stereo;
pub struct Mono;
impl Sealed for Stereo {}
impl Sealed for Mono {}
impl AudioMode for Stereo {}
impl AudioMode for Mono {}

/// Enum representing the status of the audio buffer.
/// It indicates whether the buffer is empty, has available samples, or is full.
///
/// - `Empty`: The buffer has no samples available for playback.
/// - `Available(usize)`: The buffer has still a certain number of samples available for playback, but
///  is not full. The usize value indicates the number of empty samples.
/// - `Full`: The buffer is full and cannot accept more samples until some are consumed.
pub enum AudioBufferStatus {
    /// The buffer has no samples available for playback.
    Empty,
    /// The buffer has still a certain number of samples available for playback, but is not full.
    /// The usize value indicates the number of empty samples.
    ///
    /// Important: This value represents the number of assured empty slots in the buffer, not the number of available samples.
    /// Moreover this number is an approximation view of the buffer and may not reflect the exact current state of the buffer.
    Available(usize),
    /// The buffer is full and cannot accept more samples until some are consumed.
    Full,
}

/// AudioStreamer struct for managing audio streaming to the DAC.
///
/// It needs to capture the DAC peripheral to ensure exclusive access to
/// the audio output functionality.
pub struct AudioStreamer<Mode: AudioMode, State: AudioStreamerState> {
    dac: AudioDac,
    mode: Mode,
    state: State,
}

impl<Mode: AudioMode> AudioStreamer<Mode, Uninitialized> {
    /// Brings down the AudioStreamer, releasing the DAC peripheral.
    #[inline(always)]
    pub fn release(self) -> AudioDac {
        self.dac
    }
}

impl<Mode: AudioMode> AudioStreamer<Mode, Initialized> {
    /// Disable the AudioStreamer (the DAC and audio peripheral will no longer be used by the streamer).
    #[inline(always)]
    pub fn disable(self) -> AudioStreamer<Mode, Uninitialized> {
        // Safety:
        // - Only one AudioStreamer can be initialized at a time.
        // - We have exclusive access to the DAC peripheral here.
        unsafe {
            let peri = pac::AudioStreamer::steal();
            peri.control().write_with_zero(|w| w.enable().clear_bit());
        }
        AudioStreamer {
            dac: self.dac,
            mode: self.mode,
            state: Uninitialized,
        }
    }

    /// Brings down the AudioStreamer, releasing the DAC peripheral.
    #[inline(always)]
    pub fn release(self) -> AudioDac {
        self.disable().release()
    }
}

impl<Mode: AudioMode> AudioStreamer<Mode, Uninitialized> {
    /// Convert the current AudioStreamer into Stereo mode.
    #[inline(always)]
    pub fn into_stereo(self) -> AudioStreamer<Stereo, Uninitialized> {
        // There is nothing to do while uninitialized
        AudioStreamer {
            dac: self.dac,
            mode: Stereo,
            state: self.state,
        }
    }

    /// Convert the current AudioStreamer into Mono mode.
    #[inline(always)]
    pub fn into_mono(self) -> AudioStreamer<Mono, Uninitialized> {
        // There is nothing to do while uninitialized
        AudioStreamer {
            dac: self.dac,
            mode: Mono,
            state: self.state,
        }
    }
}

impl<Mode: AudioMode> AudioStreamer<Mode, Initialized> {
    /// Convert the current AudioStreamer into Stereo mode.
    #[inline(always)]
    pub fn into_stereo(self) -> AudioStreamer<Stereo, Initialized> {
        // Safety:
        // - Only one AudioStreamer can be initialized at a time.
        // - We have exclusive access to the DAC peripheral here.
        unsafe {
            let peri = pac::AudioStreamer::steal();
            peri.control()
                .write_with_zero(|w| w.enable().set_bit().mode().set_bit());
        }
        AudioStreamer {
            dac: self.dac,
            mode: Stereo,
            state: self.state,
        }
    }

    /// Convert the current AudioStreamer into Mono mode.
    #[inline(always)]
    pub fn into_mono(self) -> AudioStreamer<Mono, Initialized> {
        // Safety:
        // - Only one AudioStreamer can be initialized at a time.
        // - We have exclusive access to the DAC peripheral here.
        unsafe {
            let peri = pac::AudioStreamer::steal();
            peri.control()
                .write_with_zero(|w| w.enable().set_bit().mode().clear_bit());
        }
        AudioStreamer {
            dac: self.dac,
            mode: Mono,
            state: self.state,
        }
    }
}

impl AudioStreamer<Stereo, Uninitialized> {
    /// Create an AudioStreamer in Stereo mode.
    #[inline(always)]
    pub fn new_stereo(dac: AudioDac) -> Self {
        AudioStreamer {
            dac,
            mode: Stereo,
            state: Uninitialized,
        }
    }

    /// Initialize the AudioStreamer in Stereo mode.
    #[inline(always)]
    pub fn initialize(self) -> AudioStreamer<Stereo, Initialized> {
        // Safety:
        // - Only one AudioStreamer can be initialized at a time.
        // - We have exclusive access to the DAC peripheral here.
        unsafe {
            let peri = pac::AudioStreamer::steal();
            peri.control()
                .write_with_zero(|w| w.enable().set_bit().mode().set_bit());
        }
        AudioStreamer {
            dac: self.dac,
            mode: Stereo,
            state: Initialized { wid: 0 },
        }
    }
}

impl AudioStreamer<Mono, Uninitialized> {
    /// Create an AudioStreamer in Mono mode.
    #[inline(always)]
    pub fn new_mono(dac: AudioDac) -> Self {
        AudioStreamer {
            dac,
            mode: Mono,
            state: Uninitialized,
        }
    }

    /// Initialize the AudioStreamer in Mono mode.
    #[inline(always)]
    pub fn initialize(self) -> AudioStreamer<Mono, Initialized> {
        // Safety:
        // - Only one AudioStreamer can be initialized at a time.
        // - We have exclusive access to the DAC peripheral here.
        unsafe {
            let peri = pac::AudioStreamer::steal();
            peri.control()
                .write_with_zero(|w| w.enable().set_bit().mode().clear_bit());
        }
        AudioStreamer {
            dac: self.dac,
            mode: self.mode,
            state: Initialized { wid: 0 },
        }
    }
}

impl AudioStreamer<Stereo, Initialized> {
    /// Write a pair of left and right audio samples to the streamer.
    ///
    /// It does not check for buffer overflows; the caller must ensure that there is space in the buffer.
    #[inline(always)]
    pub unsafe fn write_sample_unchecked(&mut self, left: u8, right: u8) {
        // TODO: Write ID
        // Safety:
        // - Only one AudioStreamer can be initialized at a time.
        // - We have exclusive access to the DAC peripheral here.
        unsafe {
            let peri = pac::AudioStreamer::steal();
            peri.data_stereo_single()
                .write(|w| w.sample_left().bits(left).sample_right().bits(right));
        }
    }

    /// Check if the current state of the audio buffer
    pub fn buffer_status(&self) -> AudioBufferStatus {
        // Safety:
        // - Only one AudioStreamer can be initialized at a time.
        // - We have exclusive access to the DAC peripheral here.
        unsafe {
            let peri = pac::AudioStreamer::steal();
            let status = peri.control().read();
            if status.queue_empty().bit() {
                AudioBufferStatus::Empty
            } else if status.queue_full().bit() {
                AudioBufferStatus::Full
            } else if status.queue_almost_empty().bit() {
                AudioBufferStatus::Available(480)
            } else {
                // In any other case, at least 32 samples can be safely written
                AudioBufferStatus::Available(32)
            }
        }
    }

    /// Write a pair of left and right audio samples to the streamer.
    ///
    /// Returns Ok(()) if the samples were written, or Err(()) if the buffer is full.
    #[inline(always)]
    pub fn write_sample(&mut self, left: u8, right: u8) -> Result<(), ()> {
        match self.buffer_status() {
            AudioBufferStatus::Full => Err(()),
            _ => {
                // Safety:
                // - AudioBuffer status checked, there is space in the buffer.
                unsafe {
                    self.write_sample_unchecked(left, right);
                }
                Ok(())
            }
        }
    }

    /// Write multiple pairs of left and right audio samples to the streamer.
    ///
    /// Returns the number of samples successfully written.
    pub fn write_samples(&mut self, samples: &[(u8, u8)]) -> usize {
        let mut written = 0;
        loop {
            // If all samples have been written, break the loop
            if samples.len() == written {
                break;
            }

            // Otherwise, check the buffer status and write samples accordingly
            match self.buffer_status() {
                AudioBufferStatus::Full => {
                    // Buffer is full, cannot write more samples
                    break;
                }
                AudioBufferStatus::Available(available) => {
                    // Buffer has some available space, write as many samples as possible
                    let to_write = core::cmp::min(available, samples.len() - written);
                    for i in 0..to_write {
                        // Safety:
                        // There is space in the buffer, we can write samples without checking
                        unsafe {
                            self.write_sample_unchecked(
                                samples[written + i].0,
                                samples[written + i].1,
                            );
                        }
                    }
                    written += to_write;
                }
                AudioBufferStatus::Empty => {
                    // Buffer is empty, we can write at 32 samples
                    // Safety:
                    // - Only one AudioStreamer can be initialized at a time.
                    // - We have exclusive access to the DAC peripheral here.
                    let to_write = core::cmp::min(32, samples.len() - written);
                    for i in 0..to_write {
                        // TODO: Optimize by writing in bursts if possible
                        // Safety:
                        // Buffer is empty, we can write samples without checking
                        unsafe {
                            self.write_sample_unchecked(
                                samples[written + i].0,
                                samples[written + i].1,
                            );
                        }
                    }
                    written += to_write;
                }
            }
        }
        written
    }
}

impl AudioStreamer<Mono, Initialized> {
    /// Write a single audio sample.
    ///
    /// It does not check for buffer overflows; the caller must ensure that there is space in the buffer.
    #[inline(always)]
    pub unsafe fn write_sample_unchecked(&mut self, sample: u8) {
        // Safety:
        // - Only one AudioStreamer can be initialized at a time.
        // - We have exclusive access to the DAC peripheral here.
        unsafe {
            let peri = pac::AudioStreamer::steal();
            peri.data_mono_single()
                .write(|w| w.sample0().bits(sample).wid().bits(self.state.wid));
        }
        self.state.wid = self.state.wid.wrapping_add(1);
    }

    /// Check if the current state of the audio buffer
    pub fn buffer_status(&self) -> AudioBufferStatus {
        // Safety:
        // - Only one AudioStreamer can be initialized at a time.
        // - We have exclusive access to the DAC peripheral here.
        unsafe {
            let peri = pac::AudioStreamer::steal();
            let status = peri.control().read();
            if status.queue_empty().bit() {
                AudioBufferStatus::Empty
            } else if status.queue_full().bit() {
                AudioBufferStatus::Full
            } else if status.queue_almost_empty().bit() {
                AudioBufferStatus::Available(512)
            } else if status.queue_almost_full().bit() {
                AudioBufferStatus::Available(1)
            } else {
                // In any other case, at least 32 samples can be safely written
                AudioBufferStatus::Available(32)
            }
        }
    }

    /// Write a single audio sample to the streamer.
    ///
    /// Returns Ok(()) if the sample was written, or Err(()) if the buffer is full.
    #[inline(always)]
    pub fn write_sample(&mut self, sample: u8) -> Result<(), ()> {
        match self.buffer_status() {
            AudioBufferStatus::Full => Err(()),
            _ => {
                // Safety:
                // AudioBuffer status checked, there is space in the buffer.
                unsafe {
                    self.write_sample_unchecked(sample);
                }
                Ok(())
            }
        }
    }

    /// Write multiple audio samples to the streamer.
    ///
    /// Returns the number of samples successfully written.
    pub fn write_samples(&mut self, samples: &[u8]) -> usize {
        let mut written = 0;
        loop {
            // If all samples have been written, break the loop
            if samples.len() == written {
                break;
            }

            // Otherwise, check the buffer status and write samples accordingly
            match self.buffer_status() {
                AudioBufferStatus::Full => {
                    // Buffer is full, cannot write more samples
                    break;
                }
                AudioBufferStatus::Available(available) => {
                    // Buffer has some available space, write as many samples as possible
                    let to_write = core::cmp::min(available, samples.len() - written);
                    for i in 0..to_write {
                        // Safety:
                        // There is space in the buffer, we can write samples without checking
                        unsafe {
                            self.write_sample_unchecked(samples[written + i]);
                        }
                    }
                    written += to_write;
                }
                AudioBufferStatus::Empty => {
                    // Buffer is empty, we can write at 32 samples
                    // Safety:
                    // - Only one AudioStreamer can be initialized at a time.
                    // - We have exclusive access to the DAC peripheral here.
                    let to_write = core::cmp::min(32, samples.len() - written);
                    for i in 0..to_write {
                        // TODO: Optimize by writing in bursts if possible
                        // Safety:
                        // Buffer is empty, we can write samples without checking
                        unsafe {
                            self.write_sample_unchecked(samples[written + i]);
                        }
                    }
                    written += to_write;
                }
            }
        }
        written
    }
}
