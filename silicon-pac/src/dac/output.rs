#[doc = "Register `OUTPUT` writer"]
pub type W = crate::W<OutputSpec>;
#[doc = "Field `LEFT_OUTPUT` writer - Set the left output value"]
pub type LeftOutputW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RIGHT_OUTPUT` writer - Set the right output value"]
pub type RightOutputW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Set the left output value"]
    #[inline(always)]
    pub fn left_output(&mut self) -> LeftOutputW<'_, OutputSpec> {
        LeftOutputW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Set the right output value"]
    #[inline(always)]
    pub fn right_output(&mut self) -> RightOutputW<'_, OutputSpec> {
        RightOutputW::new(self, 8)
    }
}
#[doc = "Control the output of the DAC (left and right).\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutputSpec;
impl crate::RegisterSpec for OutputSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`output::W`](W) writer structure"]
impl crate::Writable for OutputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTPUT to value 0"]
impl crate::Resettable for OutputSpec {}
