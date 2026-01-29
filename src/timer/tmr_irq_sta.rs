#[doc = "Register `TMR_IRQ_STA` reader"]
pub type R = crate::R<TmrIrqStaSpec>;
#[doc = "Register `TMR_IRQ_STA` writer"]
pub type W = crate::W<TmrIrqStaSpec>;
#[doc = "Field `TMR0_IRQ_PEND` reader - Timer 0 IRQ Pending (write 1 to clear)"]
pub type Tmr0IrqPendR = crate::BitReader;
#[doc = "Field `TMR0_IRQ_PEND` writer - Timer 0 IRQ Pending (write 1 to clear)"]
pub type Tmr0IrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1_IRQ_PEND` reader - Timer 1 IRQ Pending (write 1 to clear)"]
pub type Tmr1IrqPendR = crate::BitReader;
#[doc = "Field `TMR1_IRQ_PEND` writer - Timer 1 IRQ Pending (write 1 to clear)"]
pub type Tmr1IrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR2_IRQ_PEND` reader - Timer 2 IRQ Pending (write 1 to clear)"]
pub type Tmr2IrqPendR = crate::BitReader;
#[doc = "Field `TMR2_IRQ_PEND` writer - Timer 2 IRQ Pending (write 1 to clear)"]
pub type Tmr2IrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer 0 IRQ Pending (write 1 to clear)"]
    #[inline(always)]
    pub fn tmr0_irq_pend(&self) -> Tmr0IrqPendR {
        Tmr0IrqPendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1 IRQ Pending (write 1 to clear)"]
    #[inline(always)]
    pub fn tmr1_irq_pend(&self) -> Tmr1IrqPendR {
        Tmr1IrqPendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 2 IRQ Pending (write 1 to clear)"]
    #[inline(always)]
    pub fn tmr2_irq_pend(&self) -> Tmr2IrqPendR {
        Tmr2IrqPendR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 0 IRQ Pending (write 1 to clear)"]
    #[inline(always)]
    pub fn tmr0_irq_pend(&mut self) -> Tmr0IrqPendW<'_, TmrIrqStaSpec> {
        Tmr0IrqPendW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer 1 IRQ Pending (write 1 to clear)"]
    #[inline(always)]
    pub fn tmr1_irq_pend(&mut self) -> Tmr1IrqPendW<'_, TmrIrqStaSpec> {
        Tmr1IrqPendW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer 2 IRQ Pending (write 1 to clear)"]
    #[inline(always)]
    pub fn tmr2_irq_pend(&mut self) -> Tmr2IrqPendW<'_, TmrIrqStaSpec> {
        Tmr2IrqPendW::new(self, 2)
    }
}
#[doc = "Timer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr_irq_sta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr_irq_sta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmrIrqStaSpec;
impl crate::RegisterSpec for TmrIrqStaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr_irq_sta::R`](R) reader structure"]
impl crate::Readable for TmrIrqStaSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr_irq_sta::W`](W) writer structure"]
impl crate::Writable for TmrIrqStaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMR_IRQ_STA to value 0"]
impl crate::Resettable for TmrIrqStaSpec {}
