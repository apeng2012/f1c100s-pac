#[doc = "Register `FCR` writer"]
pub type W = crate::W<FcrSpec>;
#[doc = "Field `FIFOE` writer - FIFO Enable"]
pub type FifoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIFOR` writer - RX FIFO Reset"]
pub type RfiforW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFIFOR` writer - TX FIFO Reset"]
pub type XfiforW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAM` writer - DMA Mode"]
pub type DmamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TET` writer - TX Empty Trigger Level"]
pub type TetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RT` writer - Receiver Trigger Level"]
pub type RtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl W {
    #[doc = "Bit 0 - FIFO Enable"]
    #[inline(always)]
    pub fn fifoe(&mut self) -> FifoeW<'_, FcrSpec> {
        FifoeW::new(self, 0)
    }
    #[doc = "Bit 1 - RX FIFO Reset"]
    #[inline(always)]
    pub fn rfifor(&mut self) -> RfiforW<'_, FcrSpec> {
        RfiforW::new(self, 1)
    }
    #[doc = "Bit 2 - TX FIFO Reset"]
    #[inline(always)]
    pub fn xfifor(&mut self) -> XfiforW<'_, FcrSpec> {
        XfiforW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Mode"]
    #[inline(always)]
    pub fn dmam(&mut self) -> DmamW<'_, FcrSpec> {
        DmamW::new(self, 3)
    }
    #[doc = "Bits 4:5 - TX Empty Trigger Level"]
    #[inline(always)]
    pub fn tet(&mut self) -> TetW<'_, FcrSpec> {
        TetW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Receiver Trigger Level"]
    #[inline(always)]
    pub fn rt(&mut self) -> RtW<'_, FcrSpec> {
        RtW::new(self, 6)
    }
}
#[doc = "FIFO Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcrSpec;
impl crate::RegisterSpec for FcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FcrSpec {}
