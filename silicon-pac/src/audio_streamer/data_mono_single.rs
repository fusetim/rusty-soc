#[doc = "Register `DATA_MONO_SINGLE` writer"]
pub type W = crate::W<DataMonoSingleSpec>;
#[doc = "Field `SAMPLE0` writer - Mono channel audio sample."]
pub type Sample0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WID` writer - Write ID for the audio sample. Two consecutive writes with the same ID will be treated as a single sample write."]
pub type WidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Mono channel audio sample."]
    #[inline(always)]
    pub fn sample0(&mut self) -> Sample0W<'_, DataMonoSingleSpec> {
        Sample0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Write ID for the audio sample. Two consecutive writes with the same ID will be treated as a single sample write."]
    #[inline(always)]
    pub fn wid(&mut self) -> WidW<'_, DataMonoSingleSpec> {
        WidW::new(self, 8)
    }
}
#[doc = "Write a single mono audio sample to the stream queue.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_mono_single::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataMonoSingleSpec;
impl crate::RegisterSpec for DataMonoSingleSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`data_mono_single::W`](W) writer structure"]
impl crate::Writable for DataMonoSingleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_MONO_SINGLE to value 0"]
impl crate::Resettable for DataMonoSingleSpec {}
