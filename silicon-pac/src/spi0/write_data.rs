#[doc = "Register `WRITE_DATA` writer"]
pub type W = crate::W<WriteDataSpec>;
#[doc = "Field `DATA` writer - Data to be transmitted on the SPI interface."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Data to be transmitted on the SPI interface."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, WriteDataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Write a byte of data to transmit on the SPI interface. Important: This will trigger the SPI transfer.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`write_data::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WriteDataSpec;
impl crate::RegisterSpec for WriteDataSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`write_data::W`](W) writer structure"]
impl crate::Writable for WriteDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRITE_DATA to value 0"]
impl crate::Resettable for WriteDataSpec {}
