#[doc = "Register `TMR2_CUR` reader"]
pub type R = crate::R<Tmr2CurSpec>;
#[doc = "Field `TMR2_CUR` reader - Timer 2 Current Value"]
pub type Tmr2CurR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timer 2 Current Value"]
    #[inline(always)]
    pub fn tmr2_cur(&self) -> Tmr2CurR {
        Tmr2CurR::new(self.bits)
    }
}
#[doc = "Timer 2 Current Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr2_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr2CurSpec;
impl crate::RegisterSpec for Tmr2CurSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr2_cur::R`](R) reader structure"]
impl crate::Readable for Tmr2CurSpec {}
#[doc = "`reset()` method sets TMR2_CUR to value 0"]
impl crate::Resettable for Tmr2CurSpec {}
