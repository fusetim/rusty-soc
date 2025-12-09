#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ENABLE` reader - Enable Timer"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable Timer"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` writer - Reset Timer to zero"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Timer"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timer"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset Timer to zero"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, CtrlSpec> {
        ResetW::new(self, 1)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
