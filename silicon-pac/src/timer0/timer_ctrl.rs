#[doc = "Register `TimerCtrl` reader"]
pub type R = crate::R<TimerCtrlSpec>;
#[doc = "Register `TimerCtrl` writer"]
pub type W = crate::W<TimerCtrlSpec>;
#[doc = "Field `Enable` reader - Enable signal, if HIGH, the timer will start to count."]
pub type EnableR = crate::BitReader;
#[doc = "Field `Enable` writer - Enable signal, if HIGH, the timer will start to count."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reset` reader - Reset the timer to zero, if HIGH, the timer will be maintained at zero."]
pub type ResetR = crate::BitReader;
#[doc = "Field `Reset` writer - Reset the timer to zero, if HIGH, the timer will be maintained at zero."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable signal, if HIGH, the timer will start to count."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset the timer to zero, if HIGH, the timer will be maintained at zero."]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable signal, if HIGH, the timer will start to count."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, TimerCtrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset the timer to zero, if HIGH, the timer will be maintained at zero."]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, TimerCtrlSpec> {
        ResetW::new(self, 1)
    }
}
#[doc = "Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerCtrlSpec;
impl crate::RegisterSpec for TimerCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`timer_ctrl::R`](R) reader structure"]
impl crate::Readable for TimerCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`timer_ctrl::W`](W) writer structure"]
impl crate::Writable for TimerCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TimerCtrl to value 0"]
impl crate::Resettable for TimerCtrlSpec {}
