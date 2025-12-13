#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    _reserved1: [u8; 0x03],
    write_data: WriteData,
    _reserved2: [u8; 0x03],
    read_data: ReadData,
    _reserved3: [u8; 0x03],
    status: Status,
    _reserved4: [u8; 0x03],
    read_and_status: ReadAndStatus,
}
impl RegisterBlock {
    #[doc = "0x00 - Control of the SPI0 interface."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Write a byte of data to transmit on the SPI interface. Important: This will trigger the SPI transfer."]
    #[inline(always)]
    pub const fn write_data(&self) -> &WriteData {
        &self.write_data
    }
    #[doc = "0x08 - Read a byte of data from the SPI interface."]
    #[inline(always)]
    pub const fn read_data(&self) -> &ReadData {
        &self.read_data
    }
    #[doc = "0x0c - Read the status of the SPI interface."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10 - Read the data received and the status of the SPI interface."]
    #[inline(always)]
    pub const fn read_and_status(&self) -> &ReadAndStatus {
        &self.read_and_status
    }
}
#[doc = "CTRL (w) register accessor: Control of the SPI0 interface.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control of the SPI0 interface."]
pub mod ctrl;
#[doc = "WRITE_DATA (w) register accessor: Write a byte of data to transmit on the SPI interface. Important: This will trigger the SPI transfer.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`write_data::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@write_data`] module"]
#[doc(alias = "WRITE_DATA")]
pub type WriteData = crate::Reg<write_data::WriteDataSpec>;
#[doc = "Write a byte of data to transmit on the SPI interface. Important: This will trigger the SPI transfer."]
pub mod write_data;
#[doc = "READ_DATA (r) register accessor: Read a byte of data from the SPI interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`read_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@read_data`] module"]
#[doc(alias = "READ_DATA")]
pub type ReadData = crate::Reg<read_data::ReadDataSpec>;
#[doc = "Read a byte of data from the SPI interface."]
pub mod read_data;
#[doc = "STATUS (r) register accessor: Read the status of the SPI interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Read the status of the SPI interface."]
pub mod status;
#[doc = "READ_AND_STATUS (r) register accessor: Read the data received and the status of the SPI interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`read_and_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@read_and_status`] module"]
#[doc(alias = "READ_AND_STATUS")]
pub type ReadAndStatus = crate::Reg<read_and_status::ReadAndStatusSpec>;
#[doc = "Read the data received and the status of the SPI interface."]
pub mod read_and_status;
