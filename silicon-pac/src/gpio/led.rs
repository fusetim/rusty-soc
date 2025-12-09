#[doc = "Register `LED` reader"]
pub type R = crate::R<LedSpec>;
#[doc = "Register `LED` writer"]
pub type W = crate::W<LedSpec>;
#[doc = "Field `LED_OUTPUT(0-7)` reader - Set output of the %sth LED"]
pub type LedOutputR = crate::BitReader;
#[doc = "Field `LED_OUTPUT(0-7)` writer - Set output of the %sth LED"]
pub type LedOutputW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LED_MASK(0-7)` reader - Mask/unmask the output for %sth LED. It must be HIGH if you expect LED_OUTPUT%s to have an effect."]
pub type LedMaskR = crate::BitReader;
#[doc = "Field `LED_MASK(0-7)` writer - Mask/unmask the output for %sth LED. It must be HIGH if you expect LED_OUTPUT%s to have an effect."]
pub type LedMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Set output of the (0-7)th LED"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `LED_OUTPUT0` field.</div>"]
    #[inline(always)]
    pub fn led_output(&self, n: u8) -> LedOutputR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        LedOutputR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Set output of the (0-7)th LED"]
    #[inline(always)]
    pub fn led_output_iter(&self) -> impl Iterator<Item = LedOutputR> + '_ {
        (0..8).map(move |n| LedOutputR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Set output of the 0th LED"]
    #[inline(always)]
    pub fn led_output0(&self) -> LedOutputR {
        LedOutputR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set output of the 1th LED"]
    #[inline(always)]
    pub fn led_output1(&self) -> LedOutputR {
        LedOutputR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set output of the 2th LED"]
    #[inline(always)]
    pub fn led_output2(&self) -> LedOutputR {
        LedOutputR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set output of the 3th LED"]
    #[inline(always)]
    pub fn led_output3(&self) -> LedOutputR {
        LedOutputR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set output of the 4th LED"]
    #[inline(always)]
    pub fn led_output4(&self) -> LedOutputR {
        LedOutputR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set output of the 5th LED"]
    #[inline(always)]
    pub fn led_output5(&self) -> LedOutputR {
        LedOutputR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set output of the 6th LED"]
    #[inline(always)]
    pub fn led_output6(&self) -> LedOutputR {
        LedOutputR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set output of the 7th LED"]
    #[inline(always)]
    pub fn led_output7(&self) -> LedOutputR {
        LedOutputR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Mask/unmask the output for (0-7)th LED. It must be HIGH if you expect LED_OUTPUT(0-7) to have an effect."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `LED_MASK0` field.</div>"]
    #[inline(always)]
    pub fn led_mask(&self, n: u8) -> LedMaskR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        LedMaskR::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Mask/unmask the output for (0-7)th LED. It must be HIGH if you expect LED_OUTPUT(0-7) to have an effect."]
    #[inline(always)]
    pub fn led_mask_iter(&self) -> impl Iterator<Item = LedMaskR> + '_ {
        (0..8).map(move |n| LedMaskR::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    #[doc = "Bit 8 - Mask/unmask the output for 0th LED. It must be HIGH if you expect LED_OUTPUT0 to have an effect."]
    #[inline(always)]
    pub fn led_mask0(&self) -> LedMaskR {
        LedMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask/unmask the output for 1th LED. It must be HIGH if you expect LED_OUTPUT1 to have an effect."]
    #[inline(always)]
    pub fn led_mask1(&self) -> LedMaskR {
        LedMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask/unmask the output for 2th LED. It must be HIGH if you expect LED_OUTPUT2 to have an effect."]
    #[inline(always)]
    pub fn led_mask2(&self) -> LedMaskR {
        LedMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask/unmask the output for 3th LED. It must be HIGH if you expect LED_OUTPUT3 to have an effect."]
    #[inline(always)]
    pub fn led_mask3(&self) -> LedMaskR {
        LedMaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Mask/unmask the output for 4th LED. It must be HIGH if you expect LED_OUTPUT4 to have an effect."]
    #[inline(always)]
    pub fn led_mask4(&self) -> LedMaskR {
        LedMaskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mask/unmask the output for 5th LED. It must be HIGH if you expect LED_OUTPUT5 to have an effect."]
    #[inline(always)]
    pub fn led_mask5(&self) -> LedMaskR {
        LedMaskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Mask/unmask the output for 6th LED. It must be HIGH if you expect LED_OUTPUT6 to have an effect."]
    #[inline(always)]
    pub fn led_mask6(&self) -> LedMaskR {
        LedMaskR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Mask/unmask the output for 7th LED. It must be HIGH if you expect LED_OUTPUT7 to have an effect."]
    #[inline(always)]
    pub fn led_mask7(&self) -> LedMaskR {
        LedMaskR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Set output of the (0-7)th LED"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `LED_OUTPUT0` field.</div>"]
    #[inline(always)]
    pub fn led_output(&mut self, n: u8) -> LedOutputW<'_, LedSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        LedOutputW::new(self, n)
    }
    #[doc = "Bit 0 - Set output of the 0th LED"]
    #[inline(always)]
    pub fn led_output0(&mut self) -> LedOutputW<'_, LedSpec> {
        LedOutputW::new(self, 0)
    }
    #[doc = "Bit 1 - Set output of the 1th LED"]
    #[inline(always)]
    pub fn led_output1(&mut self) -> LedOutputW<'_, LedSpec> {
        LedOutputW::new(self, 1)
    }
    #[doc = "Bit 2 - Set output of the 2th LED"]
    #[inline(always)]
    pub fn led_output2(&mut self) -> LedOutputW<'_, LedSpec> {
        LedOutputW::new(self, 2)
    }
    #[doc = "Bit 3 - Set output of the 3th LED"]
    #[inline(always)]
    pub fn led_output3(&mut self) -> LedOutputW<'_, LedSpec> {
        LedOutputW::new(self, 3)
    }
    #[doc = "Bit 4 - Set output of the 4th LED"]
    #[inline(always)]
    pub fn led_output4(&mut self) -> LedOutputW<'_, LedSpec> {
        LedOutputW::new(self, 4)
    }
    #[doc = "Bit 5 - Set output of the 5th LED"]
    #[inline(always)]
    pub fn led_output5(&mut self) -> LedOutputW<'_, LedSpec> {
        LedOutputW::new(self, 5)
    }
    #[doc = "Bit 6 - Set output of the 6th LED"]
    #[inline(always)]
    pub fn led_output6(&mut self) -> LedOutputW<'_, LedSpec> {
        LedOutputW::new(self, 6)
    }
    #[doc = "Bit 7 - Set output of the 7th LED"]
    #[inline(always)]
    pub fn led_output7(&mut self) -> LedOutputW<'_, LedSpec> {
        LedOutputW::new(self, 7)
    }
    #[doc = "Mask/unmask the output for (0-7)th LED. It must be HIGH if you expect LED_OUTPUT(0-7) to have an effect."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `LED_MASK0` field.</div>"]
    #[inline(always)]
    pub fn led_mask(&mut self, n: u8) -> LedMaskW<'_, LedSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        LedMaskW::new(self, n + 8)
    }
    #[doc = "Bit 8 - Mask/unmask the output for 0th LED. It must be HIGH if you expect LED_OUTPUT0 to have an effect."]
    #[inline(always)]
    pub fn led_mask0(&mut self) -> LedMaskW<'_, LedSpec> {
        LedMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - Mask/unmask the output for 1th LED. It must be HIGH if you expect LED_OUTPUT1 to have an effect."]
    #[inline(always)]
    pub fn led_mask1(&mut self) -> LedMaskW<'_, LedSpec> {
        LedMaskW::new(self, 9)
    }
    #[doc = "Bit 10 - Mask/unmask the output for 2th LED. It must be HIGH if you expect LED_OUTPUT2 to have an effect."]
    #[inline(always)]
    pub fn led_mask2(&mut self) -> LedMaskW<'_, LedSpec> {
        LedMaskW::new(self, 10)
    }
    #[doc = "Bit 11 - Mask/unmask the output for 3th LED. It must be HIGH if you expect LED_OUTPUT3 to have an effect."]
    #[inline(always)]
    pub fn led_mask3(&mut self) -> LedMaskW<'_, LedSpec> {
        LedMaskW::new(self, 11)
    }
    #[doc = "Bit 12 - Mask/unmask the output for 4th LED. It must be HIGH if you expect LED_OUTPUT4 to have an effect."]
    #[inline(always)]
    pub fn led_mask4(&mut self) -> LedMaskW<'_, LedSpec> {
        LedMaskW::new(self, 12)
    }
    #[doc = "Bit 13 - Mask/unmask the output for 5th LED. It must be HIGH if you expect LED_OUTPUT5 to have an effect."]
    #[inline(always)]
    pub fn led_mask5(&mut self) -> LedMaskW<'_, LedSpec> {
        LedMaskW::new(self, 13)
    }
    #[doc = "Bit 14 - Mask/unmask the output for 6th LED. It must be HIGH if you expect LED_OUTPUT6 to have an effect."]
    #[inline(always)]
    pub fn led_mask6(&mut self) -> LedMaskW<'_, LedSpec> {
        LedMaskW::new(self, 14)
    }
    #[doc = "Bit 15 - Mask/unmask the output for 7th LED. It must be HIGH if you expect LED_OUTPUT7 to have an effect."]
    #[inline(always)]
    pub fn led_mask7(&mut self) -> LedMaskW<'_, LedSpec> {
        LedMaskW::new(self, 15)
    }
}
#[doc = "Control of the output of the on-board LEDs.\n\nYou can [`read`](crate::Reg::read) this register and get [`led::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`led::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LedSpec;
impl crate::RegisterSpec for LedSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`led::R`](R) reader structure"]
impl crate::Readable for LedSpec {}
#[doc = "`write(|w| ..)` method takes [`led::W`](W) writer structure"]
impl crate::Writable for LedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LED to value 0"]
impl crate::Resettable for LedSpec {}
