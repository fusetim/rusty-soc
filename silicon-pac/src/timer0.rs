#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timer_lo_value: TimerLoValue,
    timer_hi_value: TimerHiValue,
    timer_ctrl: TimerCtrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer Lower Value Register"]
    #[inline(always)]
    pub const fn timer_lo_value(&self) -> &TimerLoValue {
        &self.timer_lo_value
    }
    #[doc = "0x04 - Timer Higher Value Register"]
    #[inline(always)]
    pub const fn timer_hi_value(&self) -> &TimerHiValue {
        &self.timer_hi_value
    }
    #[doc = "0x08 - Timer Control Register"]
    #[inline(always)]
    pub const fn timer_ctrl(&self) -> &TimerCtrl {
        &self.timer_ctrl
    }
}
#[doc = "TimerLoValue (r) register accessor: Timer Lower Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_lo_value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_lo_value`] module"]
pub type TimerLoValue = crate::Reg<timer_lo_value::TimerLoValueSpec>;
#[doc = "Timer Lower Value Register"]
pub mod timer_lo_value;
#[doc = "TimerHiValue (r) register accessor: Timer Higher Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_hi_value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_hi_value`] module"]
pub type TimerHiValue = crate::Reg<timer_hi_value::TimerHiValueSpec>;
#[doc = "Timer Higher Value Register"]
pub mod timer_hi_value;
#[doc = "TimerCtrl (rw) register accessor: Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_ctrl`] module"]
pub type TimerCtrl = crate::Reg<timer_ctrl::TimerCtrlSpec>;
#[doc = "Timer Control Register"]
pub mod timer_ctrl;
