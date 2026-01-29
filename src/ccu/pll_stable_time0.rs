#[doc = "Register `PLL_STABLE_TIME0` reader"]
pub type R = crate::R<PllStableTime0Spec>;
#[doc = "Register `PLL_STABLE_TIME0` writer"]
pub type W = crate::W<PllStableTime0Spec>;
#[doc = "Field `PLL_LOCK_TIME` reader - PLL Lock Time (Unit: us)"]
pub type PllLockTimeR = crate::FieldReader<u16>;
#[doc = "Field `PLL_LOCK_TIME` writer - PLL Lock Time (Unit: us)"]
pub type PllLockTimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PLL Lock Time (Unit: us)"]
    #[inline(always)]
    pub fn pll_lock_time(&self) -> PllLockTimeR {
        PllLockTimeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PLL Lock Time (Unit: us)"]
    #[inline(always)]
    pub fn pll_lock_time(&mut self) -> PllLockTimeW<'_, PllStableTime0Spec> {
        PllLockTimeW::new(self, 0)
    }
}
#[doc = "PLL Stable Time Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_stable_time0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_stable_time0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllStableTime0Spec;
impl crate::RegisterSpec for PllStableTime0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_stable_time0::R`](R) reader structure"]
impl crate::Readable for PllStableTime0Spec {}
#[doc = "`write(|w| ..)` method takes [`pll_stable_time0::W`](W) writer structure"]
impl crate::Writable for PllStableTime0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_STABLE_TIME0 to value 0xff"]
impl crate::Resettable for PllStableTime0Spec {
    const RESET_VALUE: u32 = 0xff;
}
