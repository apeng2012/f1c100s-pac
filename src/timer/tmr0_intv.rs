#[doc = "Register `TMR0_INTV` reader"]
pub type R = crate::R<Tmr0IntvSpec>;
#[doc = "Register `TMR0_INTV` writer"]
pub type W = crate::W<Tmr0IntvSpec>;
#[doc = "Field `TMR0_INTV` reader - Timer 0 Interval Value"]
pub type Tmr0IntvR = crate::FieldReader<u32>;
#[doc = "Field `TMR0_INTV` writer - Timer 0 Interval Value"]
pub type Tmr0IntvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer 0 Interval Value"]
    #[inline(always)]
    pub fn tmr0_intv(&self) -> Tmr0IntvR {
        Tmr0IntvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer 0 Interval Value"]
    #[inline(always)]
    pub fn tmr0_intv(&mut self) -> Tmr0IntvW<'_, Tmr0IntvSpec> {
        Tmr0IntvW::new(self, 0)
    }
}
#[doc = "Timer 0 Interval Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr0_intv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr0_intv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr0IntvSpec;
impl crate::RegisterSpec for Tmr0IntvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr0_intv::R`](R) reader structure"]
impl crate::Readable for Tmr0IntvSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr0_intv::W`](W) writer structure"]
impl crate::Writable for Tmr0IntvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMR0_INTV to value 0"]
impl crate::Resettable for Tmr0IntvSpec {}
