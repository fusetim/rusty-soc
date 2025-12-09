#[doc = "Register `LO_VALUE` reader"]
pub type R = crate::R<LoValueSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "32 lowest bits of the Timer Value\n\nYou can [`read`](crate::Reg::read) this register and get [`lo_value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoValueSpec;
impl crate::RegisterSpec for LoValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo_value::R`](R) reader structure"]
impl crate::Readable for LoValueSpec {}
#[doc = "`reset()` method sets LO_VALUE to value 0"]
impl crate::Resettable for LoValueSpec {}
