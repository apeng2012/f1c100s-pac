#[doc = "Register `RBR` reader"]
pub type R = crate::R<RbrSpec>;
#[doc = "Field `DATA` reader - Data byte received"]
pub type DataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Data byte received"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rbr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbrSpec;
impl crate::RegisterSpec for RbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbr::R`](R) reader structure"]
impl crate::Readable for RbrSpec {}
#[doc = "`reset()` method sets RBR to value 0"]
impl crate::Resettable for RbrSpec {}
