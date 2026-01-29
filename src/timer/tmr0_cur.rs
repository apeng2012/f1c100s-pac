#[doc = "Register `TMR0_CUR` reader"]
pub type R = crate::R<Tmr0CurSpec>;
#[doc = "Field `TMR0_CUR` reader - Timer 0 Current Value"]
pub type Tmr0CurR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timer 0 Current Value"]
    #[inline(always)]
    pub fn tmr0_cur(&self) -> Tmr0CurR {
        Tmr0CurR::new(self.bits)
    }
}
#[doc = "Timer 0 Current Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr0_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr0CurSpec;
impl crate::RegisterSpec for Tmr0CurSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr0_cur::R`](R) reader structure"]
impl crate::Readable for Tmr0CurSpec {}
#[doc = "`reset()` method sets TMR0_CUR to value 0"]
impl crate::Resettable for Tmr0CurSpec {}
