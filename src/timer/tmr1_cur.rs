#[doc = "Register `TMR1_CUR` reader"]
pub type R = crate::R<Tmr1CurSpec>;
#[doc = "Field `TMR1_CUR` reader - Timer 1 Current Value"]
pub type Tmr1CurR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timer 1 Current Value"]
    #[inline(always)]
    pub fn tmr1_cur(&self) -> Tmr1CurR {
        Tmr1CurR::new(self.bits)
    }
}
#[doc = "Timer 1 Current Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr1_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr1CurSpec;
impl crate::RegisterSpec for Tmr1CurSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr1_cur::R`](R) reader structure"]
impl crate::Readable for Tmr1CurSpec {}
#[doc = "`reset()` method sets TMR1_CUR to value 0"]
impl crate::Resettable for Tmr1CurSpec {}
