#[doc = "Register `RFL` reader"]
pub type R = crate::R<RflSpec>;
#[doc = "Field `RFL` reader - RX FIFO data count"]
pub type RflR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - RX FIFO data count"]
    #[inline(always)]
    pub fn rfl(&self) -> RflR {
        RflR::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "Receive FIFO Level\n\nYou can [`read`](crate::Reg::read) this register and get [`rfl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RflSpec;
impl crate::RegisterSpec for RflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfl::R`](R) reader structure"]
impl crate::Readable for RflSpec {}
#[doc = "`reset()` method sets RFL to value 0"]
impl crate::Resettable for RflSpec {}
