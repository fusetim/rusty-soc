#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_output: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x00 - Control the output of the DAC (left and right)."]
    #[inline(always)]
    pub const fn output(&self) -> &Output {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Control the left output of the DAC."]
    #[inline(always)]
    pub const fn left_output(&self) -> &LeftOutput {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x01 - Control the right output of the DAC."]
    #[inline(always)]
    pub const fn right_output(&self) -> &RightOutput {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1).cast() }
    }
}
#[doc = "LEFT_OUTPUT (w) register accessor: Control the left output of the DAC.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`left_output::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@left_output`] module"]
#[doc(alias = "LEFT_OUTPUT")]
pub type LeftOutput = crate::Reg<left_output::LeftOutputSpec>;
#[doc = "Control the left output of the DAC."]
pub mod left_output;
pub use LeftOutput as RightOutput;
pub use left_output as right_output;
#[doc = "OUTPUT (w) register accessor: Control the output of the DAC (left and right).\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@output`] module"]
#[doc(alias = "OUTPUT")]
pub type Output = crate::Reg<output::OutputSpec>;
#[doc = "Control the output of the DAC (left and right)."]
pub mod output;
