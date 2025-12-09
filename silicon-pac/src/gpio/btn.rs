#[doc = "Register `BTN` reader"]
pub type R = crate::R<BtnSpec>;
#[doc = "Field `BTN_Input(0-7)` reader - Get the individual input value for each LED (bit 0 for BTN0, bit 1 for BTN1, ..., bit 7 for BTN7)."]
pub type BtnInputR = crate::BitReader;
impl R {
    #[doc = "Get the individual input value for each LED (bit 0 for BTN0, bit 1 for BTN1, ..., bit 7 for BTN7)."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `BTN_Input0` field.</div>"]
    #[inline(always)]
    pub fn btn_input(&self, n: u8) -> BtnInputR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        BtnInputR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Get the individual input value for each LED (bit 0 for BTN0, bit 1 for BTN1, ..., bit 7 for BTN7)."]
    #[inline(always)]
    pub fn btn_input_iter(&self) -> impl Iterator<Item = BtnInputR> + '_ {
        (0..8).map(move |n| BtnInputR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Get the individual input value for each LED (bit 0 for BTN0, bit 1 for BTN1, ..., bit 7 for BTN7)."]
    #[inline(always)]
    pub fn btn_input0(&self) -> BtnInputR {
        BtnInputR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Get the individual input value for each LED (bit 0 for BTN0, bit 1 for BTN1, ..., bit 7 for BTN7)."]
    #[inline(always)]
    pub fn btn_input1(&self) -> BtnInputR {
        BtnInputR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Get the individual input value for each LED (bit 0 for BTN0, bit 1 for BTN1, ..., bit 7 for BTN7)."]
    #[inline(always)]
    pub fn btn_input2(&self) -> BtnInputR {
        BtnInputR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Get the individual input value for each LED (bit 0 for BTN0, bit 1 for BTN1, ..., bit 7 for BTN7)."]
    #[inline(always)]
    pub fn btn_input3(&self) -> BtnInputR {
        BtnInputR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Get the individual input value for each LED (bit 0 for BTN0, bit 1 for BTN1, ..., bit 7 for BTN7)."]
    #[inline(always)]
    pub fn btn_input4(&self) -> BtnInputR {
        BtnInputR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Get the individual input value for each LED (bit 0 for BTN0, bit 1 for BTN1, ..., bit 7 for BTN7)."]
    #[inline(always)]
    pub fn btn_input5(&self) -> BtnInputR {
        BtnInputR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Get the individual input value for each LED (bit 0 for BTN0, bit 1 for BTN1, ..., bit 7 for BTN7)."]
    #[inline(always)]
    pub fn btn_input6(&self) -> BtnInputR {
        BtnInputR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Get the individual input value for each LED (bit 0 for BTN0, bit 1 for BTN1, ..., bit 7 for BTN7)."]
    #[inline(always)]
    pub fn btn_input7(&self) -> BtnInputR {
        BtnInputR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Input pins for handling the on-board LEDs\n\nYou can [`read`](crate::Reg::read) this register and get [`btn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtnSpec;
impl crate::RegisterSpec for BtnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`btn::R`](R) reader structure"]
impl crate::Readable for BtnSpec {}
#[doc = "`reset()` method sets BTN to value 0"]
impl crate::Resettable for BtnSpec {}
