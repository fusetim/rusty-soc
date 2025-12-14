#[doc = "Register `SPI_SDCARD` reader"]
pub type R = crate::R<SpiSdcardSpec>;
#[doc = "Register `SPI_SDCARD` writer"]
pub type W = crate::W<SpiSdcardSpec>;
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
#[doc = "Field `MISO_INPUT` reader - Read the input of the SD card SPI MISO"]
pub type MisoInputR = crate::BitReader;
#[doc = "Field `CS_MASK` writer - Mask/unmask the output for SPI CS. It must be HIGH if you expect CS_OUTPUT to have an effect."]
pub type CsMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSI_MASK` writer - Mask/unmask the output for SPI MOSI. It must be HIGH if you expect MOSI_OUTPUT to have an effect."]
pub type MosiMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MASK` writer - Mask/unmask the output for SPI CLK. It must be HIGH if you expect CLK_OUTPUT to have an effect."]
pub type ClkMaskW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - Read the input of the SD card SPI MISO"]
    #[inline(always)]
    pub fn miso_input(&self) -> MisoInputR {
        MisoInputR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read/Set output of the SD card SPI CS"]
    #[inline(always)]
    pub fn cs_output(&mut self) -> CsOutputW<'_, SpiSdcardSpec> {
        CsOutputW::new(self, 0)
    }
    #[doc = "Bit 1 - Read/Set output of the SD card SPI MOSI"]
    #[inline(always)]
    pub fn mosi_output(&mut self) -> MosiOutputW<'_, SpiSdcardSpec> {
        MosiOutputW::new(self, 1)
    }
    #[doc = "Bit 2 - Read/Set output of the SD card SPI CLK"]
    #[inline(always)]
    pub fn clk_output(&mut self) -> ClkOutputW<'_, SpiSdcardSpec> {
        ClkOutputW::new(self, 2)
    }
    #[doc = "Bit 8 - Mask/unmask the output for SPI CS. It must be HIGH if you expect CS_OUTPUT to have an effect."]
    #[inline(always)]
    pub fn cs_mask(&mut self) -> CsMaskW<'_, SpiSdcardSpec> {
        CsMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - Mask/unmask the output for SPI MOSI. It must be HIGH if you expect MOSI_OUTPUT to have an effect."]
    #[inline(always)]
    pub fn mosi_mask(&mut self) -> MosiMaskW<'_, SpiSdcardSpec> {
        MosiMaskW::new(self, 9)
    }
    #[doc = "Bit 10 - Mask/unmask the output for SPI CLK. It must be HIGH if you expect CLK_OUTPUT to have an effect."]
    #[inline(always)]
    pub fn clk_mask(&mut self) -> ClkMaskW<'_, SpiSdcardSpec> {
        ClkMaskW::new(self, 10)
    }
}
#[doc = "Control of the input-output pins directly wired to the SD card SPI interface. Some pins are input-only or output-only.\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_sdcard::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sdcard::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiSdcardSpec;
impl crate::RegisterSpec for SpiSdcardSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spi_sdcard::R`](R) reader structure"]
impl crate::Readable for SpiSdcardSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_sdcard::W`](W) writer structure"]
impl crate::Writable for SpiSdcardSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SDCARD to value 0"]
impl crate::Resettable for SpiSdcardSpec {}
