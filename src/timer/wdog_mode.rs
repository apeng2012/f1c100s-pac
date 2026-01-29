#[doc = "Register `WDOG_MODE` reader"]
pub type R = crate::R<WdogModeSpec>;
#[doc = "Register `WDOG_MODE` writer"]
pub type W = crate::W<WdogModeSpec>;
#[doc = "Field `WDOG_EN` reader - Watchdog Enable"]
pub type WdogEnR = crate::BitReader;
#[doc = "Field `WDOG_EN` writer - Watchdog Enable"]
pub type WdogEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG_INTV` reader - Watchdog Interval Value"]
pub type WdogIntvR = crate::FieldReader;
#[doc = "Field `WDOG_INTV` writer - Watchdog Interval Value"]
pub type WdogIntvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Watchdog Enable"]
    #[inline(always)]
    pub fn wdog_en(&self) -> WdogEnR {
        WdogEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Watchdog Interval Value"]
    #[inline(always)]
    pub fn wdog_intv(&self) -> WdogIntvR {
        WdogIntvR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Enable"]
    #[inline(always)]
    pub fn wdog_en(&mut self) -> WdogEnW<'_, WdogModeSpec> {
        WdogEnW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Watchdog Interval Value"]
    #[inline(always)]
    pub fn wdog_intv(&mut self) -> WdogIntvW<'_, WdogModeSpec> {
        WdogIntvW::new(self, 4)
    }
}
#[doc = "Watchdog Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdog_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdog_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogModeSpec;
impl crate::RegisterSpec for WdogModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdog_mode::R`](R) reader structure"]
impl crate::Readable for WdogModeSpec {}
#[doc = "`write(|w| ..)` method takes [`wdog_mode::W`](W) writer structure"]
impl crate::Writable for WdogModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDOG_MODE to value 0"]
impl crate::Resettable for WdogModeSpec {}
