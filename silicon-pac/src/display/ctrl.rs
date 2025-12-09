#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ENABLE_HW_FB` reader - Enable the hardware framebuffer (will refresh at 60Hz)."]
pub type EnableHwFbR = crate::BitReader;
#[doc = "Field `ENABLE_HW_FB` writer - Enable the hardware framebuffer (will refresh at 60Hz)."]
pub type EnableHwFbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_HW_FB` writer - Clear the Hardware Framebuffer (every cell will be null)"]
pub type ClearHwFbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the hardware framebuffer (will refresh at 60Hz)."]
    #[inline(always)]
    pub fn enable_hw_fb(&self) -> EnableHwFbR {
        EnableHwFbR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the hardware framebuffer (will refresh at 60Hz)."]
    #[inline(always)]
    pub fn enable_hw_fb(&mut self) -> EnableHwFbW<'_, CtrlSpec> {
        EnableHwFbW::new(self, 0)
    }
    #[doc = "Bit 7 - Clear the Hardware Framebuffer (every cell will be null)"]
    #[inline(always)]
    pub fn clear_hw_fb(&mut self) -> ClearHwFbW<'_, CtrlSpec> {
        ClearHwFbW::new(self, 7)
    }
}
#[doc = "Control and Status of the Display Driver.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
