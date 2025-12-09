#[doc = "Register `BTN` reader"]
pub type R = crate::R<BtnSpec>;
#[doc = "Field `BTN_INPUT(0-7)` reader - Get the input from the %sth BTN."]
pub type BtnInputR = crate::BitReader;
impl R {
    #[doc = "Get the input from the (0-7)th BTN."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `BTN_INPUT0` field.</div>"]
    #[inline(always)]
    pub fn btn_input(&self, n: u8) -> BtnInputR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        BtnInputR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Get the input from the (0-7)th BTN."]
    #[inline(always)]
    pub fn btn_input_iter(&self) -> impl Iterator<Item = BtnInputR> + '_ {
        (0..8).map(move |n| BtnInputR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Get the input from the 0th BTN."]
    #[inline(always)]
    pub fn btn_input0(&self) -> BtnInputR {
        BtnInputR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Get the input from the 1th BTN."]
    #[inline(always)]
    pub fn btn_input1(&self) -> BtnInputR {
        BtnInputR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Get the input from the 2th BTN."]
    #[inline(always)]
    pub fn btn_input2(&self) -> BtnInputR {
        BtnInputR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Get the input from the 3th BTN."]
    #[inline(always)]
    pub fn btn_input3(&self) -> BtnInputR {
        BtnInputR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Get the input from the 4th BTN."]
    #[inline(always)]
    pub fn btn_input4(&self) -> BtnInputR {
        BtnInputR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Get the input from the 5th BTN."]
    #[inline(always)]
    pub fn btn_input5(&self) -> BtnInputR {
        BtnInputR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Get the input from the 6th BTN."]
    #[inline(always)]
    pub fn btn_input6(&self) -> BtnInputR {
        BtnInputR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Get the input from the 7th BTN."]
    #[inline(always)]
    pub fn btn_input7(&self) -> BtnInputR {
        BtnInputR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Get state of the on-board BTN inputs.\n\nYou can [`read`](crate::Reg::read) this register and get [`btn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtnSpec;
impl crate::RegisterSpec for BtnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`btn::R`](R) reader structure"]
impl crate::Readable for BtnSpec {}
#[doc = "`reset()` method sets BTN to value 0"]
impl crate::Resettable for BtnSpec {}
