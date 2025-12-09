#[doc = "Register `Control` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `Control` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `EnableHWFB` writer - Enable Hardware Framebuffer."]
pub type EnableHwfbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ClearHW` writer - Clear the hardware framebuffer."]
pub type ClearHwW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Enable Hardware Framebuffer."]
    #[inline(always)]
    pub fn enable_hwfb(&mut self) -> EnableHwfbW<'_, ControlSpec> {
        EnableHwfbW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear the hardware framebuffer."]
    #[inline(always)]
    pub fn clear_hw(&mut self) -> ClearHwW<'_, ControlSpec> {
        ClearHwW::new(self, 1)
    }
}
#[doc = "Control Register (Enable, ClearAll)\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Control to value 0"]
impl crate::Resettable for ControlSpec {}
