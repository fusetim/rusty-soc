#[doc = "Register `DATA_STEREO_SINGLE` writer"]
pub type W = crate::W<DataStereoSingleSpec>;
#[doc = "Field `SAMPLE_LEFT` writer - Left audio channel data."]
pub type SampleLeftW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAMPLE_RIGHT` writer - Right audio channel data."]
pub type SampleRightW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Left audio channel data."]
    #[inline(always)]
    pub fn sample_left(&mut self) -> SampleLeftW<'_, DataStereoSingleSpec> {
        SampleLeftW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Right audio channel data."]
    #[inline(always)]
    pub fn sample_right(&mut self) -> SampleRightW<'_, DataStereoSingleSpec> {
        SampleRightW::new(self, 8)
    }
}
#[doc = "Write a single stereo audio sample to the stream queue.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_stereo_single::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataStereoSingleSpec;
impl crate::RegisterSpec for DataStereoSingleSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`data_stereo_single::W`](W) writer structure"]
impl crate::Writable for DataStereoSingleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_STEREO_SINGLE to value 0"]
impl crate::Resettable for DataStereoSingleSpec {}
