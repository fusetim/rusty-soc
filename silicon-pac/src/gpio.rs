#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    led: Led,
    _reserved1: [u8; 0x02],
    btn: Btn,
}
impl RegisterBlock {
    #[doc = "0x00 - Stateful output pins for handling the on-board LEDs"]
    #[inline(always)]
    pub const fn led(&self) -> &Led {
        &self.led
    }
    #[doc = "0x04 - Input pins for handling the on-board LEDs"]
    #[inline(always)]
    pub const fn btn(&self) -> &Btn {
        &self.btn
    }
}
#[doc = "LED (rw) register accessor: Stateful output pins for handling the on-board LEDs\n\nYou can [`read`](crate::Reg::read) this register and get [`led::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`led::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@led`] module"]
#[doc(alias = "LED")]
pub type Led = crate::Reg<led::LedSpec>;
#[doc = "Stateful output pins for handling the on-board LEDs"]
pub mod led;
#[doc = "BTN (r) register accessor: Input pins for handling the on-board LEDs\n\nYou can [`read`](crate::Reg::read) this register and get [`btn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btn`] module"]
#[doc(alias = "BTN")]
pub type Btn = crate::Reg<btn::BtnSpec>;
#[doc = "Input pins for handling the on-board LEDs"]
pub mod btn;
