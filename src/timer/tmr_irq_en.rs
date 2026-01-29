#[doc = "Register `TMR_IRQ_EN` reader"]
pub type R = crate::R<TmrIrqEnSpec>;
#[doc = "Register `TMR_IRQ_EN` writer"]
pub type W = crate::W<TmrIrqEnSpec>;
#[doc = "Field `TMR0_IRQ_EN` reader - Timer 0 Interrupt Enable"]
pub type Tmr0IrqEnR = crate::BitReader;
#[doc = "Field `TMR0_IRQ_EN` writer - Timer 0 Interrupt Enable"]
pub type Tmr0IrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1_IRQ_EN` reader - Timer 1 Interrupt Enable"]
pub type Tmr1IrqEnR = crate::BitReader;
#[doc = "Field `TMR1_IRQ_EN` writer - Timer 1 Interrupt Enable"]
pub type Tmr1IrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR2_IRQ_EN` reader - Timer 2 Interrupt Enable"]
pub type Tmr2IrqEnR = crate::BitReader;
#[doc = "Field `TMR2_IRQ_EN` writer - Timer 2 Interrupt Enable"]
pub type Tmr2IrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer 0 Interrupt Enable"]
    #[inline(always)]
    pub fn tmr0_irq_en(&self) -> Tmr0IrqEnR {
        Tmr0IrqEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1 Interrupt Enable"]
    #[inline(always)]
    pub fn tmr1_irq_en(&self) -> Tmr1IrqEnR {
        Tmr1IrqEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer 2 Interrupt Enable"]
    #[inline(always)]
    pub fn tmr2_irq_en(&self) -> Tmr2IrqEnR {
        Tmr2IrqEnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 0 Interrupt Enable"]
    #[inline(always)]
    pub fn tmr0_irq_en(&mut self) -> Tmr0IrqEnW<'_, TmrIrqEnSpec> {
        Tmr0IrqEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer 1 Interrupt Enable"]
    #[inline(always)]
    pub fn tmr1_irq_en(&mut self) -> Tmr1IrqEnW<'_, TmrIrqEnSpec> {
        Tmr1IrqEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer 2 Interrupt Enable"]
    #[inline(always)]
    pub fn tmr2_irq_en(&mut self) -> Tmr2IrqEnW<'_, TmrIrqEnSpec> {
        Tmr2IrqEnW::new(self, 2)
    }
}
#[doc = "Timer IRQ Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr_irq_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr_irq_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmrIrqEnSpec;
impl crate::RegisterSpec for TmrIrqEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr_irq_en::R`](R) reader structure"]
impl crate::Readable for TmrIrqEnSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr_irq_en::W`](W) writer structure"]
impl crate::Writable for TmrIrqEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMR_IRQ_EN to value 0"]
impl crate::Resettable for TmrIrqEnSpec {}
