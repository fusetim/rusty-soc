#[doc = "Register `SPI_OLED` reader"]
pub type R = crate::R<SpiOledSpec>;
#[doc = "Register `SPI_OLED` writer"]
pub type W = crate::W<SpiOledSpec>;
#[doc = "Field `CS_OUTPUT` reader - Read/Set output of the SD card SPI CS"]
pub type CsOutputR = crate::BitReader;
#[doc = "Field `CS_OUTPUT` writer - Read/Set output of the SD card SPI CS"]
pub type CsOutputW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSI_OUTPUT` reader - Read/Set output of the SD card SPI MOSI"]
pub type MosiOutputR = crate::BitReader;
#[doc = "Field `MOSI_OUTPUT` writer - Read/Set output of the SD card SPI MOSI"]
pub type MosiOutputW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_OUTPUT` reader - Read/Set output of the SD card SPI CLK"]
pub type ClkOutputR = crate::BitReader;
#[doc = "Field `CLK_OUTPUT` writer - Read/Set output of the SD card SPI CLK"]
pub type ClkOutputW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC_OUTPUT` reader - Read/Set output of the OLED display Data/Command pin"]
pub type DcOutputR = crate::BitReader;
#[doc = "Field `DC_OUTPUT` writer - Read/Set output of the OLED display Data/Command pin"]
pub type DcOutputW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_OUTPUT` reader - Read/Set output of the OLED display Reset pin"]
pub type ResetOutputR = crate::BitReader;
#[doc = "Field `RESET_OUTPUT` writer - Read/Set output of the OLED display Reset pin"]
pub type ResetOutputW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_MASK` writer - Mask/unmask the output for SPI CS. It must be HIGH if you expect CS_OUTPUT to have an effect."]
pub type CsMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSI_MASK` writer - Mask/unmask the output for SPI MOSI. It must be HIGH if you expect MOSI_OUTPUT to have an effect."]
pub type MosiMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MASK` writer - Mask/unmask the output for SPI CLK. It must be HIGH if you expect CLK_OUTPUT to have an effect."]
pub type ClkMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC_MASK` writer - Mask/unmask the output for SPI DC. It must be HIGH if you expect DC_OUTPUT to have an effect."]
pub type DcMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_MASK` writer - Mask/unmask the output for SPI RESET. It must be HIGH if you expect RESET_OUTPUT to have an effect."]
pub type ResetMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Read/Set output of the SD card SPI CS"]
    #[inline(always)]
    pub fn cs_output(&self) -> CsOutputR {
        CsOutputR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read/Set output of the SD card SPI MOSI"]
    #[inline(always)]
    pub fn mosi_output(&self) -> MosiOutputR {
        MosiOutputR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read/Set output of the SD card SPI CLK"]
    #[inline(always)]
    pub fn clk_output(&self) -> ClkOutputR {
        ClkOutputR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read/Set output of the OLED display Data/Command pin"]
    #[inline(always)]
    pub fn dc_output(&self) -> DcOutputR {
        DcOutputR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read/Set output of the OLED display Reset pin"]
    #[inline(always)]
    pub fn reset_output(&self) -> ResetOutputR {
        ResetOutputR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read/Set output of the SD card SPI CS"]
    #[inline(always)]
    pub fn cs_output(&mut self) -> CsOutputW<'_, SpiOledSpec> {
        CsOutputW::new(self, 0)
    }
    #[doc = "Bit 1 - Read/Set output of the SD card SPI MOSI"]
    #[inline(always)]
    pub fn mosi_output(&mut self) -> MosiOutputW<'_, SpiOledSpec> {
        MosiOutputW::new(self, 1)
    }
    #[doc = "Bit 2 - Read/Set output of the SD card SPI CLK"]
    #[inline(always)]
    pub fn clk_output(&mut self) -> ClkOutputW<'_, SpiOledSpec> {
        ClkOutputW::new(self, 2)
    }
    #[doc = "Bit 3 - Read/Set output of the OLED display Data/Command pin"]
    #[inline(always)]
    pub fn dc_output(&mut self) -> DcOutputW<'_, SpiOledSpec> {
        DcOutputW::new(self, 3)
    }
    #[doc = "Bit 4 - Read/Set output of the OLED display Reset pin"]
    #[inline(always)]
    pub fn reset_output(&mut self) -> ResetOutputW<'_, SpiOledSpec> {
        ResetOutputW::new(self, 4)
    }
    #[doc = "Bit 8 - Mask/unmask the output for SPI CS. It must be HIGH if you expect CS_OUTPUT to have an effect."]
    #[inline(always)]
    pub fn cs_mask(&mut self) -> CsMaskW<'_, SpiOledSpec> {
        CsMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - Mask/unmask the output for SPI MOSI. It must be HIGH if you expect MOSI_OUTPUT to have an effect."]
    #[inline(always)]
    pub fn mosi_mask(&mut self) -> MosiMaskW<'_, SpiOledSpec> {
        MosiMaskW::new(self, 9)
    }
    #[doc = "Bit 10 - Mask/unmask the output for SPI CLK. It must be HIGH if you expect CLK_OUTPUT to have an effect."]
    #[inline(always)]
    pub fn clk_mask(&mut self) -> ClkMaskW<'_, SpiOledSpec> {
        ClkMaskW::new(self, 10)
    }
    #[doc = "Bit 11 - Mask/unmask the output for SPI DC. It must be HIGH if you expect DC_OUTPUT to have an effect."]
    #[inline(always)]
    pub fn dc_mask(&mut self) -> DcMaskW<'_, SpiOledSpec> {
        DcMaskW::new(self, 11)
    }
    #[doc = "Bit 12 - Mask/unmask the output for SPI RESET. It must be HIGH if you expect RESET_OUTPUT to have an effect."]
    #[inline(always)]
    pub fn reset_mask(&mut self) -> ResetMaskW<'_, SpiOledSpec> {
        ResetMaskW::new(self, 12)
    }
}
#[doc = "Control of the input-output pins directly wired to the OLED display SPI interface. Some pins are input-only or output-only.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_oled::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_oled::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiOledSpec;
impl crate::RegisterSpec for SpiOledSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spi_oled::R`](R) reader structure"]
impl crate::Readable for SpiOledSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_oled::W`](W) writer structure"]
impl crate::Writable for SpiOledSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_OLED to value 0"]
impl crate::Resettable for SpiOledSpec {}
