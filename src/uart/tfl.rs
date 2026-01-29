#[doc = "Register `TFL` reader"]
pub type R = crate::R<TflSpec>;
#[doc = "Field `TFL` reader - TX FIFO data count"]
pub type TflR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - TX FIFO data count"]
    #[inline(always)]
    pub fn tfl(&self) -> TflR {
        TflR::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "Transmit FIFO Level\n\nYou can [`read`](crate::Reg::read) this register and get [`tfl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TflSpec;
impl crate::RegisterSpec for TflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfl::R`](R) reader structure"]
impl crate::Readable for TflSpec {}
#[doc = "`reset()` method sets TFL to value 0"]
impl crate::Resettable for TflSpec {}
