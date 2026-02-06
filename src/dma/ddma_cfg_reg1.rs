#[doc = "Register `DDMA_CFG_REG1` reader"]
pub type R = crate::R<DdmaCfgReg1Spec>;
#[doc = "Register `DDMA_CFG_REG1` writer"]
pub type W = crate::W<DdmaCfgReg1Spec>;
#[doc = "Field `SRC_DRQ_TYPE` reader - Source DRQ Type"]
pub type SrcDrqTypeR = crate::FieldReader;
#[doc = "Field `SRC_DRQ_TYPE` writer - Source DRQ Type"]
pub type SrcDrqTypeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SRC_ADDR_MODE` reader - Source Address Mode"]
pub type SrcAddrModeR = crate::FieldReader;
#[doc = "Field `SRC_ADDR_MODE` writer - Source Address Mode"]
pub type SrcAddrModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SRC_BURST_LEN` reader - Source Burst Length"]
pub type SrcBurstLenR = crate::BitReader;
#[doc = "Field `SRC_BURST_LEN` writer - Source Burst Length"]
pub type SrcBurstLenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_DATA_WIDTH` reader - Source Data Width"]
pub type SrcDataWidthR = crate::FieldReader;
#[doc = "Field `SRC_DATA_WIDTH` writer - Source Data Width"]
pub type SrcDataWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SRC_INCR8_EN` reader - Source INCR8 enable"]
pub type SrcIncr8EnR = crate::BitReader;
#[doc = "Field `SRC_INCR8_EN` writer - Source INCR8 enable"]
pub type SrcIncr8EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REMAIN_BYTE_READ_EN` reader - Remain byte counter read enable"]
pub type RemainByteReadEnR = crate::BitReader;
#[doc = "Field `REMAIN_BYTE_READ_EN` writer - Remain byte counter read enable"]
pub type RemainByteReadEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_DRQ_TYPE` reader - Destination DRQ Type"]
pub type DstDrqTypeR = crate::FieldReader;
#[doc = "Field `DST_DRQ_TYPE` writer - Destination DRQ Type"]
pub type DstDrqTypeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DST_ADDR_MODE` reader - Destination Address Mode"]
pub type DstAddrModeR = crate::FieldReader;
#[doc = "Field `DST_ADDR_MODE` writer - Destination Address Mode"]
pub type DstAddrModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DST_BURST_LEN` reader - Destination Burst Length"]
pub type DstBurstLenR = crate::BitReader;
#[doc = "Field `DST_BURST_LEN` writer - Destination Burst Length"]
pub type DstBurstLenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_DATA_WIDTH` reader - Destination Data Width"]
pub type DstDataWidthR = crate::FieldReader;
#[doc = "Field `DST_DATA_WIDTH` writer - Destination Data Width"]
pub type DstDataWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DST_INCR8_EN` reader - Destination INCR8 enable"]
pub type DstIncr8EnR = crate::BitReader;
#[doc = "Field `DST_INCR8_EN` writer - Destination INCR8 enable"]
pub type DstIncr8EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_CONTINUOUS` reader - DMA Continuous Mode Enable"]
pub type DmaContinuousR = crate::BitReader;
#[doc = "Field `DMA_CONTINUOUS` writer - DMA Continuous Mode Enable"]
pub type DmaContinuousW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_BUSY` reader - DMA Busy Status"]
pub type DmaBusyR = crate::BitReader;
#[doc = "Field `DMA_LOADING` reader - DMA Loading"]
pub type DmaLoadingR = crate::BitReader;
#[doc = "Field `DMA_LOADING` writer - DMA Loading"]
pub type DmaLoadingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Source DRQ Type"]
    #[inline(always)]
    pub fn src_drq_type(&self) -> SrcDrqTypeR {
        SrcDrqTypeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Source Address Mode"]
    #[inline(always)]
    pub fn src_addr_mode(&self) -> SrcAddrModeR {
        SrcAddrModeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Source Burst Length"]
    #[inline(always)]
    pub fn src_burst_len(&self) -> SrcBurstLenR {
        SrcBurstLenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Source Data Width"]
    #[inline(always)]
    pub fn src_data_width(&self) -> SrcDataWidthR {
        SrcDataWidthR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Source INCR8 enable"]
    #[inline(always)]
    pub fn src_incr8_en(&self) -> SrcIncr8EnR {
        SrcIncr8EnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Remain byte counter read enable"]
    #[inline(always)]
    pub fn remain_byte_read_en(&self) -> RemainByteReadEnR {
        RemainByteReadEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Destination DRQ Type"]
    #[inline(always)]
    pub fn dst_drq_type(&self) -> DstDrqTypeR {
        DstDrqTypeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Destination Address Mode"]
    #[inline(always)]
    pub fn dst_addr_mode(&self) -> DstAddrModeR {
        DstAddrModeR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Destination Burst Length"]
    #[inline(always)]
    pub fn dst_burst_len(&self) -> DstBurstLenR {
        DstBurstLenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Destination Data Width"]
    #[inline(always)]
    pub fn dst_data_width(&self) -> DstDataWidthR {
        DstDataWidthR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Destination INCR8 enable"]
    #[inline(always)]
    pub fn dst_incr8_en(&self) -> DstIncr8EnR {
        DstIncr8EnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA Continuous Mode Enable"]
    #[inline(always)]
    pub fn dma_continuous(&self) -> DmaContinuousR {
        DmaContinuousR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA Busy Status"]
    #[inline(always)]
    pub fn dma_busy(&self) -> DmaBusyR {
        DmaBusyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA Loading"]
    #[inline(always)]
    pub fn dma_loading(&self) -> DmaLoadingR {
        DmaLoadingR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Source DRQ Type"]
    #[inline(always)]
    pub fn src_drq_type(&mut self) -> SrcDrqTypeW<'_, DdmaCfgReg1Spec> {
        SrcDrqTypeW::new(self, 0)
    }
    #[doc = "Bits 5:6 - Source Address Mode"]
    #[inline(always)]
    pub fn src_addr_mode(&mut self) -> SrcAddrModeW<'_, DdmaCfgReg1Spec> {
        SrcAddrModeW::new(self, 5)
    }
    #[doc = "Bit 7 - Source Burst Length"]
    #[inline(always)]
    pub fn src_burst_len(&mut self) -> SrcBurstLenW<'_, DdmaCfgReg1Spec> {
        SrcBurstLenW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Source Data Width"]
    #[inline(always)]
    pub fn src_data_width(&mut self) -> SrcDataWidthW<'_, DdmaCfgReg1Spec> {
        SrcDataWidthW::new(self, 8)
    }
    #[doc = "Bit 10 - Source INCR8 enable"]
    #[inline(always)]
    pub fn src_incr8_en(&mut self) -> SrcIncr8EnW<'_, DdmaCfgReg1Spec> {
        SrcIncr8EnW::new(self, 10)
    }
    #[doc = "Bit 15 - Remain byte counter read enable"]
    #[inline(always)]
    pub fn remain_byte_read_en(&mut self) -> RemainByteReadEnW<'_, DdmaCfgReg1Spec> {
        RemainByteReadEnW::new(self, 15)
    }
    #[doc = "Bits 16:20 - Destination DRQ Type"]
    #[inline(always)]
    pub fn dst_drq_type(&mut self) -> DstDrqTypeW<'_, DdmaCfgReg1Spec> {
        DstDrqTypeW::new(self, 16)
    }
    #[doc = "Bits 21:22 - Destination Address Mode"]
    #[inline(always)]
    pub fn dst_addr_mode(&mut self) -> DstAddrModeW<'_, DdmaCfgReg1Spec> {
        DstAddrModeW::new(self, 21)
    }
    #[doc = "Bit 23 - Destination Burst Length"]
    #[inline(always)]
    pub fn dst_burst_len(&mut self) -> DstBurstLenW<'_, DdmaCfgReg1Spec> {
        DstBurstLenW::new(self, 23)
    }
    #[doc = "Bits 24:25 - Destination Data Width"]
    #[inline(always)]
    pub fn dst_data_width(&mut self) -> DstDataWidthW<'_, DdmaCfgReg1Spec> {
        DstDataWidthW::new(self, 24)
    }
    #[doc = "Bit 26 - Destination INCR8 enable"]
    #[inline(always)]
    pub fn dst_incr8_en(&mut self) -> DstIncr8EnW<'_, DdmaCfgReg1Spec> {
        DstIncr8EnW::new(self, 26)
    }
    #[doc = "Bit 29 - DMA Continuous Mode Enable"]
    #[inline(always)]
    pub fn dma_continuous(&mut self) -> DmaContinuousW<'_, DdmaCfgReg1Spec> {
        DmaContinuousW::new(self, 29)
    }
    #[doc = "Bit 31 - DMA Loading"]
    #[inline(always)]
    pub fn dma_loading(&mut self) -> DmaLoadingW<'_, DdmaCfgReg1Spec> {
        DmaLoadingW::new(self, 31)
    }
}
#[doc = "Dedicated DMA Channel 1 Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_cfg_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_cfg_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdmaCfgReg1Spec;
impl crate::RegisterSpec for DdmaCfgReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddma_cfg_reg1::R`](R) reader structure"]
impl crate::Readable for DdmaCfgReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`ddma_cfg_reg1::W`](W) writer structure"]
impl crate::Writable for DdmaCfgReg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DDMA_CFG_REG1 to value 0"]
impl crate::Resettable for DdmaCfgReg1Spec {}
