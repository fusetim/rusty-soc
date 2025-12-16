#[doc = "Register `TRANSFER` writer"]
pub type W = crate::W<TransferSpec>;
#[doc = "Field `BYTE` writer - Set the byte of data or command to send to the device."]
pub type ByteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TYPE` writer - Set the type of transfer (CMD or DATA). CMD = 0, DATA = 1"]
pub type TypeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:7 - Set the byte of data or command to send to the device."]
    #[inline(always)]
    pub fn byte(&mut self) -> ByteW<'_, TransferSpec> {
        ByteW::new(self, 0)
    }
    #[doc = "Bit 8 - Set the type of transfer (CMD or DATA). CMD = 0, DATA = 1"]
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW<'_, TransferSpec> {
        TypeW::new(self, 8)
    }
}
#[doc = "Send a CMD or DATA transfer to the Display.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`transfer::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TransferSpec;
impl crate::RegisterSpec for TransferSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`transfer::W`](W) writer structure"]
impl crate::Writable for TransferSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRANSFER to value 0"]
impl crate::Resettable for TransferSpec {}
