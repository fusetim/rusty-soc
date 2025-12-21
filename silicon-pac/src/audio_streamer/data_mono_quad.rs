#[doc = "Register `DATA_MONO_QUAD` writer"]
pub type W = crate::W<DataMonoQuadSpec>;
#[doc = "Field `SAMPLE0` writer - Mono channel audio sample."]
pub type Sample0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAMPLE1` writer - Mono channel audio sample."]
pub type Sample1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAMPLE2` writer - Mono channel audio sample."]
pub type Sample2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAMPLE3` writer - Mono channel audio sample."]
pub type Sample3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Mono channel audio sample."]
    #[inline(always)]
    pub fn sample0(&mut self) -> Sample0W<'_, DataMonoQuadSpec> {
        Sample0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Mono channel audio sample."]
    #[inline(always)]
    pub fn sample1(&mut self) -> Sample1W<'_, DataMonoQuadSpec> {
        Sample1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Mono channel audio sample."]
    #[inline(always)]
    pub fn sample2(&mut self) -> Sample2W<'_, DataMonoQuadSpec> {
        Sample2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Mono channel audio sample."]
    #[inline(always)]
    pub fn sample3(&mut self) -> Sample3W<'_, DataMonoQuadSpec> {
        Sample3W::new(self, 24)
    }
}
#[doc = "Write four mono audio samples to the stream queue.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_mono_quad::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataMonoQuadSpec;
impl crate::RegisterSpec for DataMonoQuadSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`data_mono_quad::W`](W) writer structure"]
impl crate::Writable for DataMonoQuadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_MONO_QUAD to value 0"]
impl crate::Resettable for DataMonoQuadSpec {}
