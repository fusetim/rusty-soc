#[doc = "Register `Output` writer"]
pub type W = crate::W<OutputSpec>;
#[doc = "Field `Left` writer - Set the left audio output value"]
pub type LeftW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Right` writer - Set the right audio output value"]
pub type RightW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Set the left audio output value"]
    #[inline(always)]
    pub fn left(&mut self) -> LeftW<'_, OutputSpec> {
        LeftW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Set the right audio output value"]
    #[inline(always)]
    pub fn right(&mut self) -> RightW<'_, OutputSpec> {
        RightW::new(self, 8)
    }
}
#[doc = "Output signal (in big-endian)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutputSpec;
impl crate::RegisterSpec for OutputSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`output::W`](W) writer structure"]
impl crate::Writable for OutputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Output to value 0"]
impl crate::Resettable for OutputSpec {}
