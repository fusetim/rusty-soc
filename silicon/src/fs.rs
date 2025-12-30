use embedded_sdmmc::{TimeSource, Timestamp};
use crate::peripheral::SdCard;

/// A TimeSource implementation that always returns a zero timestamp.
pub struct ZeroTimeSource;
impl TimeSource for ZeroTimeSource {
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 10,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

pub type VolumeManager = embedded_sdmmc::VolumeManager<SdCard, ZeroTimeSource>;
pub use embedded_sdmmc::{RawDirectory, RawFile, RawVolume};