#[doc = "Register `AUDIO_VIZ` reader"]
pub type R = crate::R<AudioVizSpec>;
#[doc = "Register `AUDIO_VIZ` writer"]
pub type W = crate::W<AudioVizSpec>;
#[doc = "Field `ENABLE` reader - Read/Set the enable state of the audio visualization. If enabled, the audio visualization is active, and on-board LEDs reflect the audio signal and not the GPIO state."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Read/Set the enable state of the audio visualization. If enabled, the audio visualization is active, and on-board LEDs reflect the audio signal and not the GPIO state."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Read/Set the enable state of the audio visualization. If enabled, the audio visualization is active, and on-board LEDs reflect the audio signal and not the GPIO state."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read/Set the enable state of the audio visualization. If enabled, the audio visualization is active, and on-board LEDs reflect the audio signal and not the GPIO state."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, AudioVizSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Control of the audio viz pins\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_viz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_viz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioVizSpec;
impl crate::RegisterSpec for AudioVizSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`audio_viz::R`](R) reader structure"]
impl crate::Readable for AudioVizSpec {}
#[doc = "`write(|w| ..)` method takes [`audio_viz::W`](W) writer structure"]
impl crate::Writable for AudioVizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUDIO_VIZ to value 0"]
impl crate::Resettable for AudioVizSpec {}
