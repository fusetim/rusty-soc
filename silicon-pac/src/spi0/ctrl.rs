#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `START` writer - Start the SPI transfer."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` writer - Reset the SPI0 interface."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Start the SPI transfer."]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, CtrlSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset the SPI0 interface."]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, CtrlSpec> {
        ResetW::new(self, 1)
    }
}
#[doc = "Control of the SPI0 interface.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
