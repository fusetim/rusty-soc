#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `READY` reader - Indicates if the SPI interface has data ready to be read."]
pub type ReadyR = crate::BitReader;
#[doc = "Field `BUSY` reader - Indicates if the SPI interface is currently busy with a transfer."]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates if the SPI interface has data ready to be read."]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates if the SPI interface is currently busy with a transfer."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Read the status of the SPI interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
