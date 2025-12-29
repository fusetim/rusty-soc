#![no_std]

pub mod mbr;

pub mod block { 
    pub use embedded_sdmmc::{BlockDevice, Block, BlockIdx, BlockCount};
}

pub mod sd {
    pub use embedded_sdmmc::SdCard;
    pub use embedded_sdmmc::sdcard::*;
}