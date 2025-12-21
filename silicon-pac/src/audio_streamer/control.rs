#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `ENABLE` reader - Enable the peripheral (and link it to the DAC)."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable the peripheral (and link it to the DAC)."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - Set the operating mode of the peripheral (MONO=0 or STEREO=1)."]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - Set the operating mode of the peripheral (MONO=0 or STEREO=1)."]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUEUE_FULL` reader - Indicates if the internal stream queue is full."]
pub type QueueFullR = crate::BitReader;
#[doc = "Field `QUEUE_ALMOST_FULL` reader - Indicates if the internal stream queue is almost full (at least 32 bytes can be safely written)."]
pub type QueueAlmostFullR = crate::BitReader;
#[doc = "Field `QUEUE_ALMOST_EMPTY` reader - Indicates if the internal stream queue is almost empty (less than 32 bytes are still unprocessed -- safe to write 480 bytes)."]
pub type QueueAlmostEmptyR = crate::BitReader;
#[doc = "Field `QUEUE_EMPTY` reader - Indicates if the internal stream queue is empty."]
pub type QueueEmptyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enable the peripheral (and link it to the DAC)."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set the operating mode of the peripheral (MONO=0 or STEREO=1)."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates if the internal stream queue is full."]
    #[inline(always)]
    pub fn queue_full(&self) -> QueueFullR {
        QueueFullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates if the internal stream queue is almost full (at least 32 bytes can be safely written)."]
    #[inline(always)]
    pub fn queue_almost_full(&self) -> QueueAlmostFullR {
        QueueAlmostFullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates if the internal stream queue is almost empty (less than 32 bytes are still unprocessed -- safe to write 480 bytes)."]
    #[inline(always)]
    pub fn queue_almost_empty(&self) -> QueueAlmostEmptyR {
        QueueAlmostEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates if the internal stream queue is empty."]
    #[inline(always)]
    pub fn queue_empty(&self) -> QueueEmptyR {
        QueueEmptyR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the peripheral (and link it to the DAC)."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, ControlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Set the operating mode of the peripheral (MONO=0 or STEREO=1)."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, ControlSpec> {
        ModeW::new(self, 1)
    }
}
#[doc = "Control and configure the peripheral.\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for ControlSpec {}
