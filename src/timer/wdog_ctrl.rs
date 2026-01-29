#[doc = "Register `WDOG_CTRL` reader"]
pub type R = crate::R<WdogCtrlSpec>;
#[doc = "Register `WDOG_CTRL` writer"]
pub type W = crate::W<WdogCtrlSpec>;
#[doc = "Field `WDOG_RSTART` reader - Watchdog Restart"]
pub type WdogRstartR = crate::BitReader;
#[doc = "Field `WDOG_RSTART` writer - Watchdog Restart"]
pub type WdogRstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG_KEY_FIELD` reader - Watchdog Key Field (must write 0xA57)"]
pub type WdogKeyFieldR = crate::FieldReader<u16>;
#[doc = "Field `WDOG_KEY_FIELD` writer - Watchdog Key Field (must write 0xA57)"]
pub type WdogKeyFieldW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Watchdog Restart"]
    #[inline(always)]
    pub fn wdog_rstart(&self) -> WdogRstartR {
        WdogRstartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:12 - Watchdog Key Field (must write 0xA57)"]
    #[inline(always)]
    pub fn wdog_key_field(&self) -> WdogKeyFieldR {
        WdogKeyFieldR::new(((self.bits >> 1) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Restart"]
    #[inline(always)]
    pub fn wdog_rstart(&mut self) -> WdogRstartW<'_, WdogCtrlSpec> {
        WdogRstartW::new(self, 0)
    }
    #[doc = "Bits 1:12 - Watchdog Key Field (must write 0xA57)"]
    #[inline(always)]
    pub fn wdog_key_field(&mut self) -> WdogKeyFieldW<'_, WdogCtrlSpec> {
        WdogKeyFieldW::new(self, 1)
    }
}
#[doc = "Watchdog Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdog_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdog_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogCtrlSpec;
impl crate::RegisterSpec for WdogCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdog_ctrl::R`](R) reader structure"]
impl crate::Readable for WdogCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wdog_ctrl::W`](W) writer structure"]
impl crate::Writable for WdogCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDOG_CTRL to value 0"]
impl crate::Resettable for WdogCtrlSpec {}
