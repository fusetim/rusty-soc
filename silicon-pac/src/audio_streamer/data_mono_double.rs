#[doc = "Register `DATA_MONO_DOUBLE` writer"]
pub type W = crate::W<DataMonoDoubleSpec>;
#[doc = "Field `SAMPLE0` writer - Mono channel audio sample."]
pub type Sample0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAMPLE1` writer - Mono channel audio sample."]
pub type Sample1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Mono channel audio sample."]
    #[inline(always)]
    pub fn sample0(&mut self) -> Sample0W<'_, DataMonoDoubleSpec> {
        Sample0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Mono channel audio sample."]
    #[inline(always)]
    pub fn sample1(&mut self) -> Sample1W<'_, DataMonoDoubleSpec> {
        Sample1W::new(self, 8)
    }
}
#[doc = "Write two mono audio samples to the stream queue.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_mono_double::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataMonoDoubleSpec;
impl crate::RegisterSpec for DataMonoDoubleSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`data_mono_double::W`](W) writer structure"]
impl crate::Writable for DataMonoDoubleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_MONO_DOUBLE to value 0"]
impl crate::Resettable for DataMonoDoubleSpec {}
