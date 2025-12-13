#[doc = "Register `SPI_CS` reader"]
pub type R = crate::R<SpiCsSpec>;
#[doc = "Register `SPI_CS` writer"]
pub type W = crate::W<SpiCsSpec>;
#[doc = "Field `CS_OUTPUT(0-7)` reader - Set output of the %sth SPI CS"]
pub type CsOutputR = crate::BitReader;
#[doc = "Field `CS_OUTPUT(0-7)` writer - Set output of the %sth SPI CS"]
pub type CsOutputW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_MASK(0-7)` reader - Mask/unmask the output for %sth SPI CS. It must be HIGH if you expect CS_OUTPUT%s to have an effect."]
pub type CsMaskR = crate::BitReader;
#[doc = "Field `CS_MASK(0-7)` writer - Mask/unmask the output for %sth SPI CS. It must be HIGH if you expect CS_OUTPUT%s to have an effect."]
pub type CsMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Set output of the (0-7)th SPI CS"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CS_OUTPUT0` field.</div>"]
    #[inline(always)]
    pub fn cs_output(&self, n: u8) -> CsOutputR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CsOutputR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Set output of the (0-7)th SPI CS"]
    #[inline(always)]
    pub fn cs_output_iter(&self) -> impl Iterator<Item = CsOutputR> + '_ {
        (0..8).map(move |n| CsOutputR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Set output of the 0th SPI CS"]
    #[inline(always)]
    pub fn cs_output0(&self) -> CsOutputR {
        CsOutputR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set output of the 1th SPI CS"]
    #[inline(always)]
    pub fn cs_output1(&self) -> CsOutputR {
        CsOutputR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set output of the 2th SPI CS"]
    #[inline(always)]
    pub fn cs_output2(&self) -> CsOutputR {
        CsOutputR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set output of the 3th SPI CS"]
    #[inline(always)]
    pub fn cs_output3(&self) -> CsOutputR {
        CsOutputR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set output of the 4th SPI CS"]
    #[inline(always)]
    pub fn cs_output4(&self) -> CsOutputR {
        CsOutputR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set output of the 5th SPI CS"]
    #[inline(always)]
    pub fn cs_output5(&self) -> CsOutputR {
        CsOutputR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set output of the 6th SPI CS"]
    #[inline(always)]
    pub fn cs_output6(&self) -> CsOutputR {
        CsOutputR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set output of the 7th SPI CS"]
    #[inline(always)]
    pub fn cs_output7(&self) -> CsOutputR {
        CsOutputR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Mask/unmask the output for (0-7)th SPI CS. It must be HIGH if you expect CS_OUTPUT(0-7) to have an effect."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CS_MASK0` field.</div>"]
    #[inline(always)]
    pub fn cs_mask(&self, n: u8) -> CsMaskR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CsMaskR::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Mask/unmask the output for (0-7)th SPI CS. It must be HIGH if you expect CS_OUTPUT(0-7) to have an effect."]
    #[inline(always)]
    pub fn cs_mask_iter(&self) -> impl Iterator<Item = CsMaskR> + '_ {
        (0..8).map(move |n| CsMaskR::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    #[doc = "Bit 8 - Mask/unmask the output for 0th SPI CS. It must be HIGH if you expect CS_OUTPUT0 to have an effect."]
    #[inline(always)]
    pub fn cs_mask0(&self) -> CsMaskR {
        CsMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask/unmask the output for 1th SPI CS. It must be HIGH if you expect CS_OUTPUT1 to have an effect."]
    #[inline(always)]
    pub fn cs_mask1(&self) -> CsMaskR {
        CsMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask/unmask the output for 2th SPI CS. It must be HIGH if you expect CS_OUTPUT2 to have an effect."]
    #[inline(always)]
    pub fn cs_mask2(&self) -> CsMaskR {
        CsMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask/unmask the output for 3th SPI CS. It must be HIGH if you expect CS_OUTPUT3 to have an effect."]
    #[inline(always)]
    pub fn cs_mask3(&self) -> CsMaskR {
        CsMaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Mask/unmask the output for 4th SPI CS. It must be HIGH if you expect CS_OUTPUT4 to have an effect."]
    #[inline(always)]
    pub fn cs_mask4(&self) -> CsMaskR {
        CsMaskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mask/unmask the output for 5th SPI CS. It must be HIGH if you expect CS_OUTPUT5 to have an effect."]
    #[inline(always)]
    pub fn cs_mask5(&self) -> CsMaskR {
        CsMaskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Mask/unmask the output for 6th SPI CS. It must be HIGH if you expect CS_OUTPUT6 to have an effect."]
    #[inline(always)]
    pub fn cs_mask6(&self) -> CsMaskR {
        CsMaskR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Mask/unmask the output for 7th SPI CS. It must be HIGH if you expect CS_OUTPUT7 to have an effect."]
    #[inline(always)]
    pub fn cs_mask7(&self) -> CsMaskR {
        CsMaskR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Set output of the (0-7)th SPI CS"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CS_OUTPUT0` field.</div>"]
    #[inline(always)]
    pub fn cs_output(&mut self, n: u8) -> CsOutputW<'_, SpiCsSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CsOutputW::new(self, n)
    }
    #[doc = "Bit 0 - Set output of the 0th SPI CS"]
    #[inline(always)]
    pub fn cs_output0(&mut self) -> CsOutputW<'_, SpiCsSpec> {
        CsOutputW::new(self, 0)
    }
    #[doc = "Bit 1 - Set output of the 1th SPI CS"]
    #[inline(always)]
    pub fn cs_output1(&mut self) -> CsOutputW<'_, SpiCsSpec> {
        CsOutputW::new(self, 1)
    }
    #[doc = "Bit 2 - Set output of the 2th SPI CS"]
    #[inline(always)]
    pub fn cs_output2(&mut self) -> CsOutputW<'_, SpiCsSpec> {
        CsOutputW::new(self, 2)
    }
    #[doc = "Bit 3 - Set output of the 3th SPI CS"]
    #[inline(always)]
    pub fn cs_output3(&mut self) -> CsOutputW<'_, SpiCsSpec> {
        CsOutputW::new(self, 3)
    }
    #[doc = "Bit 4 - Set output of the 4th SPI CS"]
    #[inline(always)]
    pub fn cs_output4(&mut self) -> CsOutputW<'_, SpiCsSpec> {
        CsOutputW::new(self, 4)
    }
    #[doc = "Bit 5 - Set output of the 5th SPI CS"]
    #[inline(always)]
    pub fn cs_output5(&mut self) -> CsOutputW<'_, SpiCsSpec> {
        CsOutputW::new(self, 5)
    }
    #[doc = "Bit 6 - Set output of the 6th SPI CS"]
    #[inline(always)]
    pub fn cs_output6(&mut self) -> CsOutputW<'_, SpiCsSpec> {
        CsOutputW::new(self, 6)
    }
    #[doc = "Bit 7 - Set output of the 7th SPI CS"]
    #[inline(always)]
    pub fn cs_output7(&mut self) -> CsOutputW<'_, SpiCsSpec> {
        CsOutputW::new(self, 7)
    }
    #[doc = "Mask/unmask the output for (0-7)th SPI CS. It must be HIGH if you expect CS_OUTPUT(0-7) to have an effect."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CS_MASK0` field.</div>"]
    #[inline(always)]
    pub fn cs_mask(&mut self, n: u8) -> CsMaskW<'_, SpiCsSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CsMaskW::new(self, n + 8)
    }
    #[doc = "Bit 8 - Mask/unmask the output for 0th SPI CS. It must be HIGH if you expect CS_OUTPUT0 to have an effect."]
    #[inline(always)]
    pub fn cs_mask0(&mut self) -> CsMaskW<'_, SpiCsSpec> {
        CsMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - Mask/unmask the output for 1th SPI CS. It must be HIGH if you expect CS_OUTPUT1 to have an effect."]
    #[inline(always)]
    pub fn cs_mask1(&mut self) -> CsMaskW<'_, SpiCsSpec> {
        CsMaskW::new(self, 9)
    }
    #[doc = "Bit 10 - Mask/unmask the output for 2th SPI CS. It must be HIGH if you expect CS_OUTPUT2 to have an effect."]
    #[inline(always)]
    pub fn cs_mask2(&mut self) -> CsMaskW<'_, SpiCsSpec> {
        CsMaskW::new(self, 10)
    }
    #[doc = "Bit 11 - Mask/unmask the output for 3th SPI CS. It must be HIGH if you expect CS_OUTPUT3 to have an effect."]
    #[inline(always)]
    pub fn cs_mask3(&mut self) -> CsMaskW<'_, SpiCsSpec> {
        CsMaskW::new(self, 11)
    }
    #[doc = "Bit 12 - Mask/unmask the output for 4th SPI CS. It must be HIGH if you expect CS_OUTPUT4 to have an effect."]
    #[inline(always)]
    pub fn cs_mask4(&mut self) -> CsMaskW<'_, SpiCsSpec> {
        CsMaskW::new(self, 12)
    }
    #[doc = "Bit 13 - Mask/unmask the output for 5th SPI CS. It must be HIGH if you expect CS_OUTPUT5 to have an effect."]
    #[inline(always)]
    pub fn cs_mask5(&mut self) -> CsMaskW<'_, SpiCsSpec> {
        CsMaskW::new(self, 13)
    }
    #[doc = "Bit 14 - Mask/unmask the output for 6th SPI CS. It must be HIGH if you expect CS_OUTPUT6 to have an effect."]
    #[inline(always)]
    pub fn cs_mask6(&mut self) -> CsMaskW<'_, SpiCsSpec> {
        CsMaskW::new(self, 14)
    }
    #[doc = "Bit 15 - Mask/unmask the output for 7th SPI CS. It must be HIGH if you expect CS_OUTPUT7 to have an effect."]
    #[inline(always)]
    pub fn cs_mask7(&mut self) -> CsMaskW<'_, SpiCsSpec> {
        CsMaskW::new(self, 15)
    }
}
#[doc = "Control of the output of the on-board SPI chip selects.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiCsSpec;
impl crate::RegisterSpec for SpiCsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spi_cs::R`](R) reader structure"]
impl crate::Readable for SpiCsSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_cs::W`](W) writer structure"]
impl crate::Writable for SpiCsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_CS to value 0"]
impl crate::Resettable for SpiCsSpec {}
