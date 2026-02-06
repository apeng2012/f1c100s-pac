#[doc = "Register `DMA_INT_STA` reader"]
pub type R = crate::R<DmaIntStaSpec>;
#[doc = "Register `DMA_INT_STA` writer"]
pub type W = crate::W<DmaIntStaSpec>;
#[doc = "Field `NDMA0_HALF_IRQ_PEND` reader - Normal DMA 0 Half Transfer Interrupt Pending. Write 1 to clear."]
pub type Ndma0HalfIrqPendR = crate::BitReader;
#[doc = "Field `NDMA0_HALF_IRQ_PEND` writer - Normal DMA 0 Half Transfer Interrupt Pending. Write 1 to clear."]
pub type Ndma0HalfIrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDMA0_FULL_IRQ_PEND` reader - Normal DMA 0 Full Transfer Interrupt Pending. Write 1 to clear."]
pub type Ndma0FullIrqPendR = crate::BitReader;
#[doc = "Field `NDMA0_FULL_IRQ_PEND` writer - Normal DMA 0 Full Transfer Interrupt Pending. Write 1 to clear."]
pub type Ndma0FullIrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDMA1_HALF_IRQ_PEND` reader - Normal DMA 1 Half Transfer Interrupt Pending. Write 1 to clear."]
pub type Ndma1HalfIrqPendR = crate::BitReader;
#[doc = "Field `NDMA1_HALF_IRQ_PEND` writer - Normal DMA 1 Half Transfer Interrupt Pending. Write 1 to clear."]
pub type Ndma1HalfIrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDMA1_FULL_IRQ_PEND` reader - Normal DMA 1 Full Transfer Interrupt Pending. Write 1 to clear."]
pub type Ndma1FullIrqPendR = crate::BitReader;
#[doc = "Field `NDMA1_FULL_IRQ_PEND` writer - Normal DMA 1 Full Transfer Interrupt Pending. Write 1 to clear."]
pub type Ndma1FullIrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDMA2_HALF_IRQ_PEND` reader - Normal DMA 2 Half Transfer Interrupt Pending. Write 1 to clear."]
pub type Ndma2HalfIrqPendR = crate::BitReader;
#[doc = "Field `NDMA2_HALF_IRQ_PEND` writer - Normal DMA 2 Half Transfer Interrupt Pending. Write 1 to clear."]
pub type Ndma2HalfIrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDMA2_FULL_IRQ_PEND` reader - Normal DMA 2 Full Transfer Interrupt Pending. Write 1 to clear."]
pub type Ndma2FullIrqPendR = crate::BitReader;
#[doc = "Field `NDMA2_FULL_IRQ_PEND` writer - Normal DMA 2 Full Transfer Interrupt Pending. Write 1 to clear."]
pub type Ndma2FullIrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDMA3_HALF_IRQ_PEND` reader - Normal DMA 3 Half Transfer Interrupt Pending. Write 1 to clear."]
pub type Ndma3HalfIrqPendR = crate::BitReader;
#[doc = "Field `NDMA3_HALF_IRQ_PEND` writer - Normal DMA 3 Half Transfer Interrupt Pending. Write 1 to clear."]
pub type Ndma3HalfIrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDMA3_FULL_IRQ_PEND` reader - Normal DMA 3 Full Transfer Interrupt Pending. Write 1 to clear."]
pub type Ndma3FullIrqPendR = crate::BitReader;
#[doc = "Field `NDMA3_FULL_IRQ_PEND` writer - Normal DMA 3 Full Transfer Interrupt Pending. Write 1 to clear."]
pub type Ndma3FullIrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDMA0_HALF_IRQ_PEND` reader - Dedicated DMA 0 Half Transfer Interrupt Pending. Write 1 to clear."]
pub type Ddma0HalfIrqPendR = crate::BitReader;
#[doc = "Field `DDMA0_HALF_IRQ_PEND` writer - Dedicated DMA 0 Half Transfer Interrupt Pending. Write 1 to clear."]
pub type Ddma0HalfIrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDMA0_FULL_IRQ_PEND` reader - Dedicated DMA 0 Full Transfer Interrupt Pending. Write 1 to clear."]
pub type Ddma0FullIrqPendR = crate::BitReader;
#[doc = "Field `DDMA0_FULL_IRQ_PEND` writer - Dedicated DMA 0 Full Transfer Interrupt Pending. Write 1 to clear."]
pub type Ddma0FullIrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDMA1_HALF_IRQ_PEND` reader - Dedicated DMA 1 Half Transfer Interrupt Pending. Write 1 to clear."]
pub type Ddma1HalfIrqPendR = crate::BitReader;
#[doc = "Field `DDMA1_HALF_IRQ_PEND` writer - Dedicated DMA 1 Half Transfer Interrupt Pending. Write 1 to clear."]
pub type Ddma1HalfIrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDMA1_FULL_IRQ_PEND` reader - Dedicated DMA 1 Full Transfer Interrupt Pending. Write 1 to clear."]
pub type Ddma1FullIrqPendR = crate::BitReader;
#[doc = "Field `DDMA1_FULL_IRQ_PEND` writer - Dedicated DMA 1 Full Transfer Interrupt Pending. Write 1 to clear."]
pub type Ddma1FullIrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDMA2_HALF_IRQ_PEND` reader - Dedicated DMA 2 Half Transfer Interrupt Pending. Write 1 to clear."]
pub type Ddma2HalfIrqPendR = crate::BitReader;
#[doc = "Field `DDMA2_HALF_IRQ_PEND` writer - Dedicated DMA 2 Half Transfer Interrupt Pending. Write 1 to clear."]
pub type Ddma2HalfIrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDMA2_FULL_IRQ_PEND` reader - Dedicated DMA 2 Full Transfer Interrupt Pending. Write 1 to clear."]
pub type Ddma2FullIrqPendR = crate::BitReader;
#[doc = "Field `DDMA2_FULL_IRQ_PEND` writer - Dedicated DMA 2 Full Transfer Interrupt Pending. Write 1 to clear."]
pub type Ddma2FullIrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDMA3_HALF_IRQ_PEND` reader - Dedicated DMA 3 Half Transfer Interrupt Pending. Write 1 to clear."]
pub type Ddma3HalfIrqPendR = crate::BitReader;
#[doc = "Field `DDMA3_HALF_IRQ_PEND` writer - Dedicated DMA 3 Half Transfer Interrupt Pending. Write 1 to clear."]
pub type Ddma3HalfIrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDMA3_FULL_IRQ_PEND` reader - Dedicated DMA 3 Full Transfer Interrupt Pending. Write 1 to clear."]
pub type Ddma3FullIrqPendR = crate::BitReader;
#[doc = "Field `DDMA3_FULL_IRQ_PEND` writer - Dedicated DMA 3 Full Transfer Interrupt Pending. Write 1 to clear."]
pub type Ddma3FullIrqPendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Normal DMA 0 Half Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ndma0_half_irq_pend(&self) -> Ndma0HalfIrqPendR {
        Ndma0HalfIrqPendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Normal DMA 0 Full Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ndma0_full_irq_pend(&self) -> Ndma0FullIrqPendR {
        Ndma0FullIrqPendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Normal DMA 1 Half Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ndma1_half_irq_pend(&self) -> Ndma1HalfIrqPendR {
        Ndma1HalfIrqPendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Normal DMA 1 Full Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ndma1_full_irq_pend(&self) -> Ndma1FullIrqPendR {
        Ndma1FullIrqPendR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Normal DMA 2 Half Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ndma2_half_irq_pend(&self) -> Ndma2HalfIrqPendR {
        Ndma2HalfIrqPendR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Normal DMA 2 Full Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ndma2_full_irq_pend(&self) -> Ndma2FullIrqPendR {
        Ndma2FullIrqPendR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Normal DMA 3 Half Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ndma3_half_irq_pend(&self) -> Ndma3HalfIrqPendR {
        Ndma3HalfIrqPendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Normal DMA 3 Full Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ndma3_full_irq_pend(&self) -> Ndma3FullIrqPendR {
        Ndma3FullIrqPendR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Dedicated DMA 0 Half Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ddma0_half_irq_pend(&self) -> Ddma0HalfIrqPendR {
        Ddma0HalfIrqPendR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Dedicated DMA 0 Full Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ddma0_full_irq_pend(&self) -> Ddma0FullIrqPendR {
        Ddma0FullIrqPendR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Dedicated DMA 1 Half Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ddma1_half_irq_pend(&self) -> Ddma1HalfIrqPendR {
        Ddma1HalfIrqPendR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Dedicated DMA 1 Full Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ddma1_full_irq_pend(&self) -> Ddma1FullIrqPendR {
        Ddma1FullIrqPendR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Dedicated DMA 2 Half Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ddma2_half_irq_pend(&self) -> Ddma2HalfIrqPendR {
        Ddma2HalfIrqPendR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Dedicated DMA 2 Full Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ddma2_full_irq_pend(&self) -> Ddma2FullIrqPendR {
        Ddma2FullIrqPendR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Dedicated DMA 3 Half Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ddma3_half_irq_pend(&self) -> Ddma3HalfIrqPendR {
        Ddma3HalfIrqPendR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Dedicated DMA 3 Full Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ddma3_full_irq_pend(&self) -> Ddma3FullIrqPendR {
        Ddma3FullIrqPendR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Normal DMA 0 Half Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ndma0_half_irq_pend(&mut self) -> Ndma0HalfIrqPendW<'_, DmaIntStaSpec> {
        Ndma0HalfIrqPendW::new(self, 0)
    }
    #[doc = "Bit 1 - Normal DMA 0 Full Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ndma0_full_irq_pend(&mut self) -> Ndma0FullIrqPendW<'_, DmaIntStaSpec> {
        Ndma0FullIrqPendW::new(self, 1)
    }
    #[doc = "Bit 2 - Normal DMA 1 Half Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ndma1_half_irq_pend(&mut self) -> Ndma1HalfIrqPendW<'_, DmaIntStaSpec> {
        Ndma1HalfIrqPendW::new(self, 2)
    }
    #[doc = "Bit 3 - Normal DMA 1 Full Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ndma1_full_irq_pend(&mut self) -> Ndma1FullIrqPendW<'_, DmaIntStaSpec> {
        Ndma1FullIrqPendW::new(self, 3)
    }
    #[doc = "Bit 4 - Normal DMA 2 Half Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ndma2_half_irq_pend(&mut self) -> Ndma2HalfIrqPendW<'_, DmaIntStaSpec> {
        Ndma2HalfIrqPendW::new(self, 4)
    }
    #[doc = "Bit 5 - Normal DMA 2 Full Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ndma2_full_irq_pend(&mut self) -> Ndma2FullIrqPendW<'_, DmaIntStaSpec> {
        Ndma2FullIrqPendW::new(self, 5)
    }
    #[doc = "Bit 6 - Normal DMA 3 Half Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ndma3_half_irq_pend(&mut self) -> Ndma3HalfIrqPendW<'_, DmaIntStaSpec> {
        Ndma3HalfIrqPendW::new(self, 6)
    }
    #[doc = "Bit 7 - Normal DMA 3 Full Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ndma3_full_irq_pend(&mut self) -> Ndma3FullIrqPendW<'_, DmaIntStaSpec> {
        Ndma3FullIrqPendW::new(self, 7)
    }
    #[doc = "Bit 16 - Dedicated DMA 0 Half Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ddma0_half_irq_pend(&mut self) -> Ddma0HalfIrqPendW<'_, DmaIntStaSpec> {
        Ddma0HalfIrqPendW::new(self, 16)
    }
    #[doc = "Bit 17 - Dedicated DMA 0 Full Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ddma0_full_irq_pend(&mut self) -> Ddma0FullIrqPendW<'_, DmaIntStaSpec> {
        Ddma0FullIrqPendW::new(self, 17)
    }
    #[doc = "Bit 18 - Dedicated DMA 1 Half Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ddma1_half_irq_pend(&mut self) -> Ddma1HalfIrqPendW<'_, DmaIntStaSpec> {
        Ddma1HalfIrqPendW::new(self, 18)
    }
    #[doc = "Bit 19 - Dedicated DMA 1 Full Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ddma1_full_irq_pend(&mut self) -> Ddma1FullIrqPendW<'_, DmaIntStaSpec> {
        Ddma1FullIrqPendW::new(self, 19)
    }
    #[doc = "Bit 20 - Dedicated DMA 2 Half Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ddma2_half_irq_pend(&mut self) -> Ddma2HalfIrqPendW<'_, DmaIntStaSpec> {
        Ddma2HalfIrqPendW::new(self, 20)
    }
    #[doc = "Bit 21 - Dedicated DMA 2 Full Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ddma2_full_irq_pend(&mut self) -> Ddma2FullIrqPendW<'_, DmaIntStaSpec> {
        Ddma2FullIrqPendW::new(self, 21)
    }
    #[doc = "Bit 22 - Dedicated DMA 3 Half Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ddma3_half_irq_pend(&mut self) -> Ddma3HalfIrqPendW<'_, DmaIntStaSpec> {
        Ddma3HalfIrqPendW::new(self, 22)
    }
    #[doc = "Bit 23 - Dedicated DMA 3 Full Transfer Interrupt Pending. Write 1 to clear."]
    #[inline(always)]
    pub fn ddma3_full_irq_pend(&mut self) -> Ddma3FullIrqPendW<'_, DmaIntStaSpec> {
        Ddma3FullIrqPendW::new(self, 23)
    }
}
#[doc = "DMA Interrupt Status Register (write 1 to clear)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_sta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_sta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIntStaSpec;
impl crate::RegisterSpec for DmaIntStaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_sta::R`](R) reader structure"]
impl crate::Readable for DmaIntStaSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_int_sta::W`](W) writer structure"]
impl crate::Writable for DmaIntStaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_INT_STA to value 0"]
impl crate::Resettable for DmaIntStaSpec {}
