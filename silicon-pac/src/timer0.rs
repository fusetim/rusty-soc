#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    _reserved1: [u8; 0x03],
    _reserved_1_hi_value: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - 32 highest bits of the Timer Value"]
    #[inline(always)]
    pub const fn hi_value(&self) -> &HiValue {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - 32 lowest bits of the Timer Value"]
    #[inline(always)]
    pub const fn lo_value(&self) -> &LoValue {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "LO_VALUE (r) register accessor: 32 lowest bits of the Timer Value\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lo_value`] module"]
#[doc(alias = "LO_VALUE")]
pub type LoValue = crate::Reg<lo_value::LoValueSpec>;
#[doc = "32 lowest bits of the Timer Value"]
pub mod lo_value;
#[doc = "HI_VALUE (r) register accessor: 32 highest bits of the Timer Value\n\nYou can [`read`](crate::Reg::read) this register and get [`hi_value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hi_value`] module"]
#[doc(alias = "HI_VALUE")]
pub type HiValue = crate::Reg<hi_value::HiValueSpec>;
#[doc = "32 highest bits of the Timer Value"]
pub mod hi_value;
