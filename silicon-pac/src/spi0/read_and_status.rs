#[doc = "Register `READ_AND_STATUS` reader"]
pub type R = crate::R<ReadAndStatusSpec>;
#[doc = "Field `DATA` reader - Data to be transmitted on the SPI interface."]
pub type DataR = crate::FieldReader;
#[doc = "Field `READY` reader - Indicates if the SPI interface has data ready to be read."]
pub type ReadyR = crate::BitReader;
#[doc = "Field `BUSY` reader - Indicates if the SPI interface is currently busy with a transfer."]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Data to be transmitted on the SPI interface."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Indicates if the SPI interface has data ready to be read."]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates if the SPI interface is currently busy with a transfer."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Read the data received and the status of the SPI interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`read_and_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadAndStatusSpec;
impl crate::RegisterSpec for ReadAndStatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`read_and_status::R`](R) reader structure"]
impl crate::Readable for ReadAndStatusSpec {}
#[doc = "`reset()` method sets READ_AND_STATUS to value 0"]
impl crate::Resettable for ReadAndStatusSpec {}
