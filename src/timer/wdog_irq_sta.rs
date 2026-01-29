#[doc = "Register `WDOG_IRQ_STA` reader"]
pub type R = crate::R<WdogIrqStaSpec>;
#[doc = "Register `WDOG_IRQ_STA` writer"]
pub type W = crate::W<WdogIrqStaSpec>;
#[doc = "Field `WDOG_IRQ_PEND` reader - Watchdog IRQ Pending (write 1 to clear)"]
pub type WdogIrqPendR = crate::BitReader;
#[doc = "Field `WDOG_IRQ_PEND` writer - Watchdog IRQ Pending (write 1 to clear)"]
pub type WdogIrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Watchdog IRQ Pending (write 1 to clear)"]
    #[inline(always)]
    pub fn wdog_irq_pend(&self) -> WdogIrqPendR {
        WdogIrqPendR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog IRQ Pending (write 1 to clear)"]
    #[inline(always)]
    pub fn wdog_irq_pend(&mut self) -> WdogIrqPendW<'_, WdogIrqStaSpec> {
        WdogIrqPendW::new(self, 0)
    }
}
#[doc = "Watchdog Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdog_irq_sta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdog_irq_sta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogIrqStaSpec;
impl crate::RegisterSpec for WdogIrqStaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdog_irq_sta::R`](R) reader structure"]
impl crate::Readable for WdogIrqStaSpec {}
#[doc = "`write(|w| ..)` method takes [`wdog_irq_sta::W`](W) writer structure"]
impl crate::Writable for WdogIrqStaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDOG_IRQ_STA to value 0"]
impl crate::Resettable for WdogIrqStaSpec {}
