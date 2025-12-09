#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    output: Output,
}
impl RegisterBlock {
    #[doc = "0x00 - Control the output of the DAC."]
    #[inline(always)]
    pub const fn output(&self) -> &Output {
        &self.output
    }
}
#[doc = "OUTPUT (w) register accessor: Control the output of the DAC.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@output`] module"]
#[doc(alias = "OUTPUT")]
pub type Output = crate::Reg<output::OutputSpec>;
#[doc = "Control the output of the DAC."]
pub mod output;
