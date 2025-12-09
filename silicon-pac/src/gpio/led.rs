#[doc = "Register `LED` reader"]
pub type R = crate::R<LedSpec>;
#[doc = "Register `LED` writer"]
pub type W = crate::W<LedSpec>;
#[doc = "Field `LED_Output(0-7)` reader - Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
pub type LedOutputR = crate::BitReader;
#[doc = "Field `LED_Output(0-7)` writer - Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
pub type LedOutputW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LED_WriteMask(0-7)` writer - Write mask for the control of LED.\nA bit must be set HIGH for the output specified in LED_Output to be applied to the specified pin."]
pub type LedWriteMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `LED_Output0` field.</div>"]
    #[inline(always)]
    pub fn led_output(&self, n: u8) -> LedOutputR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        LedOutputR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[inline(always)]
    pub fn led_output_iter(&self) -> impl Iterator<Item = LedOutputR> + '_ {
        (0..8).map(move |n| LedOutputR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[inline(always)]
    pub fn led_output0(&self) -> LedOutputR {
        LedOutputR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[inline(always)]
    pub fn led_output1(&self) -> LedOutputR {
        LedOutputR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[inline(always)]
    pub fn led_output2(&self) -> LedOutputR {
        LedOutputR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[inline(always)]
    pub fn led_output3(&self) -> LedOutputR {
        LedOutputR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[inline(always)]
    pub fn led_output4(&self) -> LedOutputR {
        LedOutputR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[inline(always)]
    pub fn led_output5(&self) -> LedOutputR {
        LedOutputR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[inline(always)]
    pub fn led_output6(&self) -> LedOutputR {
        LedOutputR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[inline(always)]
    pub fn led_output7(&self) -> LedOutputR {
        LedOutputR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `LED_Output0` field.</div>"]
    #[inline(always)]
    pub fn led_output(&mut self, n: u8) -> LedOutputW<'_, LedSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        LedOutputW::new(self, n)
    }
    #[doc = "Bit 0 - Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[inline(always)]
    pub fn led_output0(&mut self) -> LedOutputW<'_, LedSpec> {
        LedOutputW::new(self, 0)
    }
    #[doc = "Bit 1 - Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[inline(always)]
    pub fn led_output1(&mut self) -> LedOutputW<'_, LedSpec> {
        LedOutputW::new(self, 1)
    }
    #[doc = "Bit 2 - Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[inline(always)]
    pub fn led_output2(&mut self) -> LedOutputW<'_, LedSpec> {
        LedOutputW::new(self, 2)
    }
    #[doc = "Bit 3 - Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[inline(always)]
    pub fn led_output3(&mut self) -> LedOutputW<'_, LedSpec> {
        LedOutputW::new(self, 3)
    }
    #[doc = "Bit 4 - Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[inline(always)]
    pub fn led_output4(&mut self) -> LedOutputW<'_, LedSpec> {
        LedOutputW::new(self, 4)
    }
    #[doc = "Bit 5 - Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[inline(always)]
    pub fn led_output5(&mut self) -> LedOutputW<'_, LedSpec> {
        LedOutputW::new(self, 5)
    }
    #[doc = "Bit 6 - Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[inline(always)]
    pub fn led_output6(&mut self) -> LedOutputW<'_, LedSpec> {
        LedOutputW::new(self, 6)
    }
    #[doc = "Bit 7 - Controls the state of the 8 on-board LEDs. Each bit corresponds to an individual LED (bit 0 for LED0, bit 1 for LED1, ..., bit 7 for LED7)."]
    #[inline(always)]
    pub fn led_output7(&mut self) -> LedOutputW<'_, LedSpec> {
        LedOutputW::new(self, 7)
    }
    #[doc = "Write mask for the control of LED.\nA bit must be set HIGH for the output specified in LED_Output to be applied to the specified pin."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `LED_WriteMask0` field.</div>"]
    #[inline(always)]
    pub fn led_write_mask(&mut self, n: u8) -> LedWriteMaskW<'_, LedSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        LedWriteMaskW::new(self, n + 8)
    }
    #[doc = "Bit 8 - Write mask for the control of LED.\nA bit must be set HIGH for the output specified in LED_Output to be applied to the specified pin."]
    #[inline(always)]
    pub fn led_write_mask0(&mut self) -> LedWriteMaskW<'_, LedSpec> {
        LedWriteMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - Write mask for the control of LED.\nA bit must be set HIGH for the output specified in LED_Output to be applied to the specified pin."]
    #[inline(always)]
    pub fn led_write_mask1(&mut self) -> LedWriteMaskW<'_, LedSpec> {
        LedWriteMaskW::new(self, 9)
    }
    #[doc = "Bit 10 - Write mask for the control of LED.\nA bit must be set HIGH for the output specified in LED_Output to be applied to the specified pin."]
    #[inline(always)]
    pub fn led_write_mask2(&mut self) -> LedWriteMaskW<'_, LedSpec> {
        LedWriteMaskW::new(self, 10)
    }
    #[doc = "Bit 11 - Write mask for the control of LED.\nA bit must be set HIGH for the output specified in LED_Output to be applied to the specified pin."]
    #[inline(always)]
    pub fn led_write_mask3(&mut self) -> LedWriteMaskW<'_, LedSpec> {
        LedWriteMaskW::new(self, 11)
    }
    #[doc = "Bit 12 - Write mask for the control of LED.\nA bit must be set HIGH for the output specified in LED_Output to be applied to the specified pin."]
    #[inline(always)]
    pub fn led_write_mask4(&mut self) -> LedWriteMaskW<'_, LedSpec> {
        LedWriteMaskW::new(self, 12)
    }
    #[doc = "Bit 13 - Write mask for the control of LED.\nA bit must be set HIGH for the output specified in LED_Output to be applied to the specified pin."]
    #[inline(always)]
    pub fn led_write_mask5(&mut self) -> LedWriteMaskW<'_, LedSpec> {
        LedWriteMaskW::new(self, 13)
    }
    #[doc = "Bit 14 - Write mask for the control of LED.\nA bit must be set HIGH for the output specified in LED_Output to be applied to the specified pin."]
    #[inline(always)]
    pub fn led_write_mask6(&mut self) -> LedWriteMaskW<'_, LedSpec> {
        LedWriteMaskW::new(self, 14)
    }
    #[doc = "Bit 15 - Write mask for the control of LED.\nA bit must be set HIGH for the output specified in LED_Output to be applied to the specified pin."]
    #[inline(always)]
    pub fn led_write_mask7(&mut self) -> LedWriteMaskW<'_, LedSpec> {
        LedWriteMaskW::new(self, 15)
    }
}
#[doc = "Stateful output pins for handling the on-board LEDs\n\nYou can [`read`](crate::Reg::read) this register and get [`led::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`led::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
