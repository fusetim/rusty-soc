#[doc = "Register `TimerLoValue` reader"]
pub type R = crate::R<TimerLoValueSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Timer Lower Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_lo_value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerLoValueSpec;
impl crate::RegisterSpec for TimerLoValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_lo_value::R`](R) reader structure"]
impl crate::Readable for TimerLoValueSpec {}
#[doc = "`reset()` method sets TimerLoValue to value 0"]
impl crate::Resettable for TimerLoValueSpec {}
