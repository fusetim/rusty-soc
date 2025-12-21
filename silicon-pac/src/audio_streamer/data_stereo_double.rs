#[doc = "Register `DATA_STEREO_DOUBLE` writer"]
pub type W = crate::W<DataStereoDoubleSpec>;
#[doc = "Field `SAMPLE_LEFT0` writer - Left audio channel data."]
pub type SampleLeft0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAMPLE_RIGHT0` writer - Right audio channel data."]
pub type SampleRight0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAMPLE_LEFT1` writer - Left audio channel data."]
pub type SampleLeft1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAMPLE_RIGHT1` writer - Right audio channel data."]
pub type SampleRight1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Left audio channel data."]
    #[inline(always)]
    pub fn sample_left0(&mut self) -> SampleLeft0W<'_, DataStereoDoubleSpec> {
        SampleLeft0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Right audio channel data."]
    #[inline(always)]
    pub fn sample_right0(&mut self) -> SampleRight0W<'_, DataStereoDoubleSpec> {
        SampleRight0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Left audio channel data."]
    #[inline(always)]
    pub fn sample_left1(&mut self) -> SampleLeft1W<'_, DataStereoDoubleSpec> {
        SampleLeft1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Right audio channel data."]
    #[inline(always)]
    pub fn sample_right1(&mut self) -> SampleRight1W<'_, DataStereoDoubleSpec> {
        SampleRight1W::new(self, 24)
    }
}
#[doc = "Write two stereo audio sample to the stream queue.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_stereo_double::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataStereoDoubleSpec;
impl crate::RegisterSpec for DataStereoDoubleSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`data_stereo_double::W`](W) writer structure"]
impl crate::Writable for DataStereoDoubleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_STEREO_DOUBLE to value 0"]
impl crate::Resettable for DataStereoDoubleSpec {}
