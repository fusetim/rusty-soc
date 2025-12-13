#[doc = "Register `READ_DATA` reader"]
pub type R = crate::R<ReadDataSpec>;
#[doc = "Field `DATA` reader - Data to be transmitted on the SPI interface."]
pub type DataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Data to be transmitted on the SPI interface."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Read a byte of data from the SPI interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`read_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadDataSpec;
impl crate::RegisterSpec for ReadDataSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`read_data::R`](R) reader structure"]
impl crate::Readable for ReadDataSpec {}
#[doc = "`reset()` method sets READ_DATA to value 0"]
impl crate::Resettable for ReadDataSpec {}
