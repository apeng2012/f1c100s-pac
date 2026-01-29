#[doc = "Register `TMR1_INTV` reader"]
pub type R = crate::R<Tmr1IntvSpec>;
#[doc = "Register `TMR1_INTV` writer"]
pub type W = crate::W<Tmr1IntvSpec>;
#[doc = "Field `TMR1_INTV` reader - Timer 1 Interval Value"]
pub type Tmr1IntvR = crate::FieldReader<u32>;
#[doc = "Field `TMR1_INTV` writer - Timer 1 Interval Value"]
pub type Tmr1IntvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer 1 Interval Value"]
    #[inline(always)]
    pub fn tmr1_intv(&self) -> Tmr1IntvR {
        Tmr1IntvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer 1 Interval Value"]
    #[inline(always)]
    pub fn tmr1_intv(&mut self) -> Tmr1IntvW<'_, Tmr1IntvSpec> {
        Tmr1IntvW::new(self, 0)
    }
}
#[doc = "Timer 1 Interval Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr1_intv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr1_intv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr1IntvSpec;
impl crate::RegisterSpec for Tmr1IntvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr1_intv::R`](R) reader structure"]
impl crate::Readable for Tmr1IntvSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr1_intv::W`](W) writer structure"]
impl crate::Writable for Tmr1IntvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMR1_INTV to value 0"]
impl crate::Resettable for Tmr1IntvSpec {}
