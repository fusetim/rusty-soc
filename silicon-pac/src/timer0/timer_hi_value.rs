#[doc = "Register `TimerHiValue` reader"]
pub type R = crate::R<TimerHiValueSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Timer Higher Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_hi_value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerHiValueSpec;
impl crate::RegisterSpec for TimerHiValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_hi_value::R`](R) reader structure"]
impl crate::Readable for TimerHiValueSpec {}
#[doc = "`reset()` method sets TimerHiValue to value 0"]
impl crate::Resettable for TimerHiValueSpec {}
