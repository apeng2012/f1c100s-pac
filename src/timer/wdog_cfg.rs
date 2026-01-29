#[doc = "Register `WDOG_CFG` reader"]
pub type R = crate::R<WdogCfgSpec>;
#[doc = "Register `WDOG_CFG` writer"]
pub type W = crate::W<WdogCfgSpec>;
#[doc = "Field `WDOG_CONFIG` reader - Watchdog Config (1: Reset whole system, 2: Only interrupt)"]
pub type WdogConfigR = crate::FieldReader;
#[doc = "Field `WDOG_CONFIG` writer - Watchdog Config (1: Reset whole system, 2: Only interrupt)"]
pub type WdogConfigW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Watchdog Config (1: Reset whole system, 2: Only interrupt)"]
    #[inline(always)]
    pub fn wdog_config(&self) -> WdogConfigR {
        WdogConfigR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Watchdog Config (1: Reset whole system, 2: Only interrupt)"]
    #[inline(always)]
    pub fn wdog_config(&mut self) -> WdogConfigW<'_, WdogCfgSpec> {
        WdogConfigW::new(self, 0)
    }
}
#[doc = "Watchdog Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdog_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdog_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogCfgSpec;
impl crate::RegisterSpec for WdogCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdog_cfg::R`](R) reader structure"]
impl crate::Readable for WdogCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`wdog_cfg::W`](W) writer structure"]
impl crate::Writable for WdogCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDOG_CFG to value 0x01"]
impl crate::Resettable for WdogCfgSpec {
    const RESET_VALUE: u32 = 0x01;
}
