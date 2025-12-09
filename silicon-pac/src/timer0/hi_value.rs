#[doc = "Register `HI_VALUE` reader"]
pub type R = crate::R<HiValueSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "32 highest bits of the Timer Value\n\nYou can [`read`](crate::Reg::read) this register and get [`hi_value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HiValueSpec;
impl crate::RegisterSpec for HiValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hi_value::R`](R) reader structure"]
impl crate::Readable for HiValueSpec {}
#[doc = "`reset()` method sets HI_VALUE to value 0"]
impl crate::Resettable for HiValueSpec {}
