#[doc = "Register `DMA_INT_CTRL` reader"]
pub type R = crate::R<DmaIntCtrlSpec>;
#[doc = "Register `DMA_INT_CTRL` writer"]
pub type W = crate::W<DmaIntCtrlSpec>;
#[doc = "Field `NDMA0_HALF_IRQ_EN` reader - Normal DMA 0 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ndma0HalfIrqEnR = crate::BitReader;
#[doc = "Field `NDMA0_HALF_IRQ_EN` writer - Normal DMA 0 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ndma0HalfIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDMA0_FULL_IRQ_EN` reader - Normal DMA 0 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ndma0FullIrqEnR = crate::BitReader;
#[doc = "Field `NDMA0_FULL_IRQ_EN` writer - Normal DMA 0 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ndma0FullIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDMA1_HALF_IRQ_EN` reader - Normal DMA 1 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ndma1HalfIrqEnR = crate::BitReader;
#[doc = "Field `NDMA1_HALF_IRQ_EN` writer - Normal DMA 1 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ndma1HalfIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDMA1_FULL_IRQ_EN` reader - Normal DMA 1 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ndma1FullIrqEnR = crate::BitReader;
#[doc = "Field `NDMA1_FULL_IRQ_EN` writer - Normal DMA 1 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ndma1FullIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDMA2_HALF_IRQ_EN` reader - Normal DMA 2 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ndma2HalfIrqEnR = crate::BitReader;
#[doc = "Field `NDMA2_HALF_IRQ_EN` writer - Normal DMA 2 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ndma2HalfIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDMA2_FULL_IRQ_EN` reader - Normal DMA 2 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ndma2FullIrqEnR = crate::BitReader;
#[doc = "Field `NDMA2_FULL_IRQ_EN` writer - Normal DMA 2 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ndma2FullIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDMA3_HALF_IRQ_EN` reader - Normal DMA 3 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ndma3HalfIrqEnR = crate::BitReader;
#[doc = "Field `NDMA3_HALF_IRQ_EN` writer - Normal DMA 3 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ndma3HalfIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDMA3_FULL_IRQ_EN` reader - Normal DMA 3 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ndma3FullIrqEnR = crate::BitReader;
#[doc = "Field `NDMA3_FULL_IRQ_EN` writer - Normal DMA 3 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ndma3FullIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDMA0_HALF_IRQ_EN` reader - Dedicated DMA 0 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ddma0HalfIrqEnR = crate::BitReader;
#[doc = "Field `DDMA0_HALF_IRQ_EN` writer - Dedicated DMA 0 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ddma0HalfIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDMA0_FULL_IRQ_EN` reader - Dedicated DMA 0 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ddma0FullIrqEnR = crate::BitReader;
#[doc = "Field `DDMA0_FULL_IRQ_EN` writer - Dedicated DMA 0 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ddma0FullIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDMA1_HALF_IRQ_EN` reader - Dedicated DMA 1 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ddma1HalfIrqEnR = crate::BitReader;
#[doc = "Field `DDMA1_HALF_IRQ_EN` writer - Dedicated DMA 1 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ddma1HalfIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDMA1_FULL_IRQ_EN` reader - Dedicated DMA 1 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ddma1FullIrqEnR = crate::BitReader;
#[doc = "Field `DDMA1_FULL_IRQ_EN` writer - Dedicated DMA 1 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ddma1FullIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDMA2_HALF_IRQ_EN` reader - Dedicated DMA 2 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ddma2HalfIrqEnR = crate::BitReader;
#[doc = "Field `DDMA2_HALF_IRQ_EN` writer - Dedicated DMA 2 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ddma2HalfIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDMA2_FULL_IRQ_EN` reader - Dedicated DMA 2 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ddma2FullIrqEnR = crate::BitReader;
#[doc = "Field `DDMA2_FULL_IRQ_EN` writer - Dedicated DMA 2 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ddma2FullIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDMA3_HALF_IRQ_EN` reader - Dedicated DMA 3 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ddma3HalfIrqEnR = crate::BitReader;
#[doc = "Field `DDMA3_HALF_IRQ_EN` writer - Dedicated DMA 3 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ddma3HalfIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDMA3_FULL_IRQ_EN` reader - Dedicated DMA 3 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ddma3FullIrqEnR = crate::BitReader;
#[doc = "Field `DDMA3_FULL_IRQ_EN` writer - Dedicated DMA 3 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
pub type Ddma3FullIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Normal DMA 0 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ndma0_half_irq_en(&self) -> Ndma0HalfIrqEnR {
        Ndma0HalfIrqEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Normal DMA 0 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ndma0_full_irq_en(&self) -> Ndma0FullIrqEnR {
        Ndma0FullIrqEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Normal DMA 1 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ndma1_half_irq_en(&self) -> Ndma1HalfIrqEnR {
        Ndma1HalfIrqEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Normal DMA 1 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ndma1_full_irq_en(&self) -> Ndma1FullIrqEnR {
        Ndma1FullIrqEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Normal DMA 2 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ndma2_half_irq_en(&self) -> Ndma2HalfIrqEnR {
        Ndma2HalfIrqEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Normal DMA 2 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ndma2_full_irq_en(&self) -> Ndma2FullIrqEnR {
        Ndma2FullIrqEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Normal DMA 3 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ndma3_half_irq_en(&self) -> Ndma3HalfIrqEnR {
        Ndma3HalfIrqEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Normal DMA 3 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ndma3_full_irq_en(&self) -> Ndma3FullIrqEnR {
        Ndma3FullIrqEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Dedicated DMA 0 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ddma0_half_irq_en(&self) -> Ddma0HalfIrqEnR {
        Ddma0HalfIrqEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Dedicated DMA 0 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ddma0_full_irq_en(&self) -> Ddma0FullIrqEnR {
        Ddma0FullIrqEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Dedicated DMA 1 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ddma1_half_irq_en(&self) -> Ddma1HalfIrqEnR {
        Ddma1HalfIrqEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Dedicated DMA 1 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ddma1_full_irq_en(&self) -> Ddma1FullIrqEnR {
        Ddma1FullIrqEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Dedicated DMA 2 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ddma2_half_irq_en(&self) -> Ddma2HalfIrqEnR {
        Ddma2HalfIrqEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Dedicated DMA 2 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ddma2_full_irq_en(&self) -> Ddma2FullIrqEnR {
        Ddma2FullIrqEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Dedicated DMA 3 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ddma3_half_irq_en(&self) -> Ddma3HalfIrqEnR {
        Ddma3HalfIrqEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Dedicated DMA 3 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ddma3_full_irq_en(&self) -> Ddma3FullIrqEnR {
        Ddma3FullIrqEnR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Normal DMA 0 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ndma0_half_irq_en(&mut self) -> Ndma0HalfIrqEnW<'_, DmaIntCtrlSpec> {
        Ndma0HalfIrqEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Normal DMA 0 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ndma0_full_irq_en(&mut self) -> Ndma0FullIrqEnW<'_, DmaIntCtrlSpec> {
        Ndma0FullIrqEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Normal DMA 1 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ndma1_half_irq_en(&mut self) -> Ndma1HalfIrqEnW<'_, DmaIntCtrlSpec> {
        Ndma1HalfIrqEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Normal DMA 1 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ndma1_full_irq_en(&mut self) -> Ndma1FullIrqEnW<'_, DmaIntCtrlSpec> {
        Ndma1FullIrqEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Normal DMA 2 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ndma2_half_irq_en(&mut self) -> Ndma2HalfIrqEnW<'_, DmaIntCtrlSpec> {
        Ndma2HalfIrqEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Normal DMA 2 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ndma2_full_irq_en(&mut self) -> Ndma2FullIrqEnW<'_, DmaIntCtrlSpec> {
        Ndma2FullIrqEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Normal DMA 3 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ndma3_half_irq_en(&mut self) -> Ndma3HalfIrqEnW<'_, DmaIntCtrlSpec> {
        Ndma3HalfIrqEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Normal DMA 3 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ndma3_full_irq_en(&mut self) -> Ndma3FullIrqEnW<'_, DmaIntCtrlSpec> {
        Ndma3FullIrqEnW::new(self, 7)
    }
    #[doc = "Bit 16 - Dedicated DMA 0 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ddma0_half_irq_en(&mut self) -> Ddma0HalfIrqEnW<'_, DmaIntCtrlSpec> {
        Ddma0HalfIrqEnW::new(self, 16)
    }
    #[doc = "Bit 17 - Dedicated DMA 0 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ddma0_full_irq_en(&mut self) -> Ddma0FullIrqEnW<'_, DmaIntCtrlSpec> {
        Ddma0FullIrqEnW::new(self, 17)
    }
    #[doc = "Bit 18 - Dedicated DMA 1 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ddma1_half_irq_en(&mut self) -> Ddma1HalfIrqEnW<'_, DmaIntCtrlSpec> {
        Ddma1HalfIrqEnW::new(self, 18)
    }
    #[doc = "Bit 19 - Dedicated DMA 1 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ddma1_full_irq_en(&mut self) -> Ddma1FullIrqEnW<'_, DmaIntCtrlSpec> {
        Ddma1FullIrqEnW::new(self, 19)
    }
    #[doc = "Bit 20 - Dedicated DMA 2 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ddma2_half_irq_en(&mut self) -> Ddma2HalfIrqEnW<'_, DmaIntCtrlSpec> {
        Ddma2HalfIrqEnW::new(self, 20)
    }
    #[doc = "Bit 21 - Dedicated DMA 2 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ddma2_full_irq_en(&mut self) -> Ddma2FullIrqEnW<'_, DmaIntCtrlSpec> {
        Ddma2FullIrqEnW::new(self, 21)
    }
    #[doc = "Bit 22 - Dedicated DMA 3 Half Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ddma3_half_irq_en(&mut self) -> Ddma3HalfIrqEnW<'_, DmaIntCtrlSpec> {
        Ddma3HalfIrqEnW::new(self, 22)
    }
    #[doc = "Bit 23 - Dedicated DMA 3 Full Transfer Interrupt Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn ddma3_full_irq_en(&mut self) -> Ddma3FullIrqEnW<'_, DmaIntCtrlSpec> {
        Ddma3FullIrqEnW::new(self, 23)
    }
}
#[doc = "DMA Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIntCtrlSpec;
impl crate::RegisterSpec for DmaIntCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_ctrl::R`](R) reader structure"]
impl crate::Readable for DmaIntCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_int_ctrl::W`](W) writer structure"]
impl crate::Writable for DmaIntCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_INT_CTRL to value 0"]
impl crate::Resettable for DmaIntCtrlSpec {}
