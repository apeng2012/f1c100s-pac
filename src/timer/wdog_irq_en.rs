#[doc = "Register `WDOG_IRQ_EN` reader"]
pub type R = crate::R<WdogIrqEnSpec>;
#[doc = "Register `WDOG_IRQ_EN` writer"]
pub type W = crate::W<WdogIrqEnSpec>;
#[doc = "Field `WDOG_IRQ_EN` reader - Watchdog Interrupt Enable"]
pub type WdogIrqEnR = crate::BitReader;
#[doc = "Field `WDOG_IRQ_EN` writer - Watchdog Interrupt Enable"]
pub type WdogIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdog_irq_en(&self) -> WdogIrqEnR {
        WdogIrqEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdog_irq_en(&mut self) -> WdogIrqEnW<'_, WdogIrqEnSpec> {
        WdogIrqEnW::new(self, 0)
    }
}
#[doc = "Watchdog IRQ Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdog_irq_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdog_irq_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdogIrqEnSpec;
impl crate::RegisterSpec for WdogIrqEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdog_irq_en::R`](R) reader structure"]
impl crate::Readable for WdogIrqEnSpec {}
#[doc = "`write(|w| ..)` method takes [`wdog_irq_en::W`](W) writer structure"]
impl crate::Writable for WdogIrqEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDOG_IRQ_EN to value 0"]
impl crate::Resettable for WdogIrqEnSpec {}
