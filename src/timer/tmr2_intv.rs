#[doc = "Register `TMR2_INTV` reader"]
pub type R = crate::R<Tmr2IntvSpec>;
#[doc = "Register `TMR2_INTV` writer"]
pub type W = crate::W<Tmr2IntvSpec>;
#[doc = "Field `TMR2_INTV` reader - Timer 2 Interval Value"]
pub type Tmr2IntvR = crate::FieldReader<u32>;
#[doc = "Field `TMR2_INTV` writer - Timer 2 Interval Value"]
pub type Tmr2IntvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer 2 Interval Value"]
    #[inline(always)]
    pub fn tmr2_intv(&self) -> Tmr2IntvR {
        Tmr2IntvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer 2 Interval Value"]
    #[inline(always)]
    pub fn tmr2_intv(&mut self) -> Tmr2IntvW<'_, Tmr2IntvSpec> {
        Tmr2IntvW::new(self, 0)
    }
}
#[doc = "Timer 2 Interval Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr2_intv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr2_intv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr2IntvSpec;
impl crate::RegisterSpec for Tmr2IntvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr2_intv::R`](R) reader structure"]
impl crate::Readable for Tmr2IntvSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr2_intv::W`](W) writer structure"]
impl crate::Writable for Tmr2IntvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMR2_INTV to value 0"]
impl crate::Resettable for Tmr2IntvSpec {}
