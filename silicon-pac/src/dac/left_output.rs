#[doc = "Register `LEFT_OUTPUT` writer"]
pub type W = crate::W<LeftOutputSpec>;
#[doc = "Field `VALUE` writer - Set the output value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Set the output value"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, LeftOutputSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Control the left output of the DAC.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`left_output::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LeftOutputSpec;
impl crate::RegisterSpec for LeftOutputSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`left_output::W`](W) writer structure"]
impl crate::Writable for LeftOutputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEFT_OUTPUT to value 0"]
impl crate::Resettable for LeftOutputSpec {}
