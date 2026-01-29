#[doc = "Register `PLL_STABLE_TIME1` reader"]
pub type R = crate::R<PllStableTime1Spec>;
#[doc = "Register `PLL_STABLE_TIME1` writer"]
pub type W = crate::W<PllStableTime1Spec>;
#[doc = "Field `PLL_CPU_LOCK_TIME` reader - PLL CPU Lock Time (Unit: us)"]
pub type PllCpuLockTimeR = crate::FieldReader<u16>;
#[doc = "Field `PLL_CPU_LOCK_TIME` writer - PLL CPU Lock Time (Unit: us)"]
pub type PllCpuLockTimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PLL CPU Lock Time (Unit: us)"]
    #[inline(always)]
    pub fn pll_cpu_lock_time(&self) -> PllCpuLockTimeR {
        PllCpuLockTimeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PLL CPU Lock Time (Unit: us)"]
    #[inline(always)]
    pub fn pll_cpu_lock_time(&mut self) -> PllCpuLockTimeW<'_, PllStableTime1Spec> {
        PllCpuLockTimeW::new(self, 0)
    }
}
#[doc = "PLL Stable Time Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_stable_time1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_stable_time1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllStableTime1Spec;
impl crate::RegisterSpec for PllStableTime1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_stable_time1::R`](R) reader structure"]
impl crate::Readable for PllStableTime1Spec {}
#[doc = "`write(|w| ..)` method takes [`pll_stable_time1::W`](W) writer structure"]
impl crate::Writable for PllStableTime1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_STABLE_TIME1 to value 0xff"]
impl crate::Resettable for PllStableTime1Spec {
    const RESET_VALUE: u32 = 0xff;
}
