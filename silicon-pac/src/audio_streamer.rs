#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    control: Control,
    _reserved1: [u8; 0x03],
    _reserved_1_data: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x00 - Control and configure the peripheral."]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x04 - Write a single mono audio sample to the stream queue."]
    #[inline(always)]
    pub const fn data_mono_single(&self) -> &DataMonoSingle {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Write a single stereo audio sample to the stream queue."]
    #[inline(always)]
    pub const fn data_stereo_single(&self) -> &DataStereoSingle {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
}
#[doc = "CONTROL (rw) register accessor: Control and configure the peripheral.\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`] module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Control and configure the peripheral."]
pub mod control;
#[doc = "DATA_STEREO_SINGLE (w) register accessor: Write a single stereo audio sample to the stream queue.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_stereo_single::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_stereo_single`] module"]
#[doc(alias = "DATA_STEREO_SINGLE")]
pub type DataStereoSingle = crate::Reg<data_stereo_single::DataStereoSingleSpec>;
#[doc = "Write a single stereo audio sample to the stream queue."]
pub mod data_stereo_single;
#[doc = "DATA_MONO_SINGLE (w) register accessor: Write a single mono audio sample to the stream queue.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_mono_single::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_mono_single`] module"]
#[doc(alias = "DATA_MONO_SINGLE")]
pub type DataMonoSingle = crate::Reg<data_mono_single::DataMonoSingleSpec>;
#[doc = "Write a single mono audio sample to the stream queue."]
pub mod data_mono_single;
