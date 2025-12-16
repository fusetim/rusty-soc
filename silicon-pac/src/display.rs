#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    _reserved1: [u8; 0x03],
    transfer: Transfer,
}
impl RegisterBlock {
    #[doc = "0x00 - Control and Status of the Display Driver."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Send a CMD or DATA transfer to the Display."]
    #[inline(always)]
    pub const fn transfer(&self) -> &Transfer {
        &self.transfer
    }
}
#[doc = "CTRL (rw) register accessor: Control and Status of the Display Driver.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control and Status of the Display Driver."]
pub mod ctrl;
#[doc = "TRANSFER (w) register accessor: Send a CMD or DATA transfer to the Display.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`transfer::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transfer`] module"]
#[doc(alias = "TRANSFER")]
pub type Transfer = crate::Reg<transfer::TransferSpec>;
#[doc = "Send a CMD or DATA transfer to the Display."]
pub mod transfer;
