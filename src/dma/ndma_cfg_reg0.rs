#[doc = "Register `NDMA_CFG_REG0` reader"]
pub type R = crate::R<NdmaCfgReg0Spec>;
#[doc = "Register `NDMA_CFG_REG0` writer"]
pub type W = crate::W<NdmaCfgReg0Spec>;
#[doc = "Field `SRC_DRQ_TYPE` reader - Source DRQ Type. 0x00=IR_RX, 0x04=SPI0_RX, 0x05=SPI1_RX, 0x08=UART0_RX, 0x09=UART1_RX, 0x0A=UART2_RX, 0x0C=AudioCodec, 0x0D=TP_ADC, 0x0E=DAUDIO, 0x10=SRAM, 0x11=SDRAM, 0x14=USB, 0x15=USB_EP1, 0x16=USB_EP2, 0x17=USB_EP3"]
pub type SrcDrqTypeR = crate::FieldReader;
#[doc = "Field `SRC_DRQ_TYPE` writer - Source DRQ Type. 0x00=IR_RX, 0x04=SPI0_RX, 0x05=SPI1_RX, 0x08=UART0_RX, 0x09=UART1_RX, 0x0A=UART2_RX, 0x0C=AudioCodec, 0x0D=TP_ADC, 0x0E=DAUDIO, 0x10=SRAM, 0x11=SDRAM, 0x14=USB, 0x15=USB_EP1, 0x16=USB_EP2, 0x17=USB_EP3"]
pub type SrcDrqTypeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SRC_ADDR_TYPE` reader - Source Address Type. 00=Linear, 01=IO"]
pub type SrcAddrTypeR = crate::FieldReader;
#[doc = "Field `SRC_ADDR_TYPE` writer - Source Address Type. 00=Linear, 01=IO"]
pub type SrcAddrTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SRC_BURST_LEN` reader - Source Burst Length. 0=1, 1=4"]
pub type SrcBurstLenR = crate::BitReader;
#[doc = "Field `SRC_BURST_LEN` writer - Source Burst Length. 0=1, 1=4"]
pub type SrcBurstLenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_DATA_WIDTH` reader - Source Data Width. 00=8-bit, 01=16-bit, 10=32-bit"]
pub type SrcDataWidthR = crate::FieldReader;
#[doc = "Field `SRC_DATA_WIDTH` writer - Source Data Width. 00=8-bit, 01=16-bit, 10=32-bit"]
pub type SrcDataWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REMAIN_BYTE_READ_EN` reader - Remain byte counter read enable. If set, remain byte counter can be read from NDMA_BYTE_CNT_REG."]
pub type RemainByteReadEnR = crate::BitReader;
#[doc = "Field `REMAIN_BYTE_READ_EN` writer - Remain byte counter read enable. If set, remain byte counter can be read from NDMA_BYTE_CNT_REG."]
pub type RemainByteReadEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_DRQ_TYPE` reader - Destination DRQ Type. 0x01=OWA_TX, 0x04=SPI0_TX, 0x05=SPI1_TX, 0x08=UART0_TX, 0x09=UART1_TX, 0x0A=UART2_TX, 0x0C=AudioCodec_DAC, 0x0E=DAUDIO, 0x10=SRAM, 0x11=SDRAM, 0x14=USB, 0x15=USB_EP1, 0x16=USB_EP2, 0x17=USB_EP3"]
pub type DstDrqTypeR = crate::FieldReader;
#[doc = "Field `DST_DRQ_TYPE` writer - Destination DRQ Type. 0x01=OWA_TX, 0x04=SPI0_TX, 0x05=SPI1_TX, 0x08=UART0_TX, 0x09=UART1_TX, 0x0A=UART2_TX, 0x0C=AudioCodec_DAC, 0x0E=DAUDIO, 0x10=SRAM, 0x11=SDRAM, 0x14=USB, 0x15=USB_EP1, 0x16=USB_EP2, 0x17=USB_EP3"]
pub type DstDrqTypeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DST_ADDR_TYPE` reader - Destination Address Type. 00=Linear, 01=IO"]
pub type DstAddrTypeR = crate::FieldReader;
#[doc = "Field `DST_ADDR_TYPE` writer - Destination Address Type. 00=Linear, 01=IO"]
pub type DstAddrTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DST_BURST_LEN` reader - Destination Burst Length. 0=1, 1=4"]
pub type DstBurstLenR = crate::BitReader;
#[doc = "Field `DST_BURST_LEN` writer - Destination Burst Length. 0=1, 1=4"]
pub type DstBurstLenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_DATA_WIDTH` reader - Destination Data Width. 00=8-bit, 01=16-bit, 10=32-bit"]
pub type DstDataWidthR = crate::FieldReader;
#[doc = "Field `DST_DATA_WIDTH` writer - Destination Data Width. 00=8-bit, 01=16-bit, 10=32-bit"]
pub type DstDataWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMA_WAIT_STATE` reader - DMA Wait State. 0=1, 1=2, 2=4, 3=8, 4=16, 5=32, 6=64, 7=128"]
pub type DmaWaitStateR = crate::FieldReader;
#[doc = "Field `DMA_WAIT_STATE` writer - DMA Wait State. 0=1, 1=2, 2=4, 3=8, 4=16, 5=32, 6=64, 7=128"]
pub type DmaWaitStateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DMA_CONTINUOUS` reader - DMA Continuous Mode Enable. 0=Disable, 1=Enable"]
pub type DmaContinuousR = crate::BitReader;
#[doc = "Field `DMA_CONTINUOUS` writer - DMA Continuous Mode Enable. 0=Disable, 1=Enable"]
pub type DmaContinuousW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_BUSY` reader - DMA Busy Status. 0=Idle, 1=Busy"]
pub type DmaBusyR = crate::BitReader;
#[doc = "Field `DMA_LOADING` reader - DMA Loading. Set 1 to start, auto-cleared when finished. Set 0 to reset channel."]
pub type DmaLoadingR = crate::BitReader;
#[doc = "Field `DMA_LOADING` writer - DMA Loading. Set 1 to start, auto-cleared when finished. Set 0 to reset channel."]
pub type DmaLoadingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Source DRQ Type. 0x00=IR_RX, 0x04=SPI0_RX, 0x05=SPI1_RX, 0x08=UART0_RX, 0x09=UART1_RX, 0x0A=UART2_RX, 0x0C=AudioCodec, 0x0D=TP_ADC, 0x0E=DAUDIO, 0x10=SRAM, 0x11=SDRAM, 0x14=USB, 0x15=USB_EP1, 0x16=USB_EP2, 0x17=USB_EP3"]
    #[inline(always)]
    pub fn src_drq_type(&self) -> SrcDrqTypeR {
        SrcDrqTypeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Source Address Type. 00=Linear, 01=IO"]
    #[inline(always)]
    pub fn src_addr_type(&self) -> SrcAddrTypeR {
        SrcAddrTypeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Source Burst Length. 0=1, 1=4"]
    #[inline(always)]
    pub fn src_burst_len(&self) -> SrcBurstLenR {
        SrcBurstLenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Source Data Width. 00=8-bit, 01=16-bit, 10=32-bit"]
    #[inline(always)]
    pub fn src_data_width(&self) -> SrcDataWidthR {
        SrcDataWidthR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - Remain byte counter read enable. If set, remain byte counter can be read from NDMA_BYTE_CNT_REG."]
    #[inline(always)]
    pub fn remain_byte_read_en(&self) -> RemainByteReadEnR {
        RemainByteReadEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Destination DRQ Type. 0x01=OWA_TX, 0x04=SPI0_TX, 0x05=SPI1_TX, 0x08=UART0_TX, 0x09=UART1_TX, 0x0A=UART2_TX, 0x0C=AudioCodec_DAC, 0x0E=DAUDIO, 0x10=SRAM, 0x11=SDRAM, 0x14=USB, 0x15=USB_EP1, 0x16=USB_EP2, 0x17=USB_EP3"]
    #[inline(always)]
    pub fn dst_drq_type(&self) -> DstDrqTypeR {
        DstDrqTypeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Destination Address Type. 00=Linear, 01=IO"]
    #[inline(always)]
    pub fn dst_addr_type(&self) -> DstAddrTypeR {
        DstAddrTypeR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Destination Burst Length. 0=1, 1=4"]
    #[inline(always)]
    pub fn dst_burst_len(&self) -> DstBurstLenR {
        DstBurstLenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Destination Data Width. 00=8-bit, 01=16-bit, 10=32-bit"]
    #[inline(always)]
    pub fn dst_data_width(&self) -> DstDataWidthR {
        DstDataWidthR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:28 - DMA Wait State. 0=1, 1=2, 2=4, 3=8, 4=16, 5=32, 6=64, 7=128"]
    #[inline(always)]
    pub fn dma_wait_state(&self) -> DmaWaitStateR {
        DmaWaitStateR::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bit 29 - DMA Continuous Mode Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn dma_continuous(&self) -> DmaContinuousR {
        DmaContinuousR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA Busy Status. 0=Idle, 1=Busy"]
    #[inline(always)]
    pub fn dma_busy(&self) -> DmaBusyR {
        DmaBusyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA Loading. Set 1 to start, auto-cleared when finished. Set 0 to reset channel."]
    #[inline(always)]
    pub fn dma_loading(&self) -> DmaLoadingR {
        DmaLoadingR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Source DRQ Type. 0x00=IR_RX, 0x04=SPI0_RX, 0x05=SPI1_RX, 0x08=UART0_RX, 0x09=UART1_RX, 0x0A=UART2_RX, 0x0C=AudioCodec, 0x0D=TP_ADC, 0x0E=DAUDIO, 0x10=SRAM, 0x11=SDRAM, 0x14=USB, 0x15=USB_EP1, 0x16=USB_EP2, 0x17=USB_EP3"]
    #[inline(always)]
    pub fn src_drq_type(&mut self) -> SrcDrqTypeW<'_, NdmaCfgReg0Spec> {
        SrcDrqTypeW::new(self, 0)
    }
    #[doc = "Bits 5:6 - Source Address Type. 00=Linear, 01=IO"]
    #[inline(always)]
    pub fn src_addr_type(&mut self) -> SrcAddrTypeW<'_, NdmaCfgReg0Spec> {
        SrcAddrTypeW::new(self, 5)
    }
    #[doc = "Bit 7 - Source Burst Length. 0=1, 1=4"]
    #[inline(always)]
    pub fn src_burst_len(&mut self) -> SrcBurstLenW<'_, NdmaCfgReg0Spec> {
        SrcBurstLenW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Source Data Width. 00=8-bit, 01=16-bit, 10=32-bit"]
    #[inline(always)]
    pub fn src_data_width(&mut self) -> SrcDataWidthW<'_, NdmaCfgReg0Spec> {
        SrcDataWidthW::new(self, 8)
    }
    #[doc = "Bit 15 - Remain byte counter read enable. If set, remain byte counter can be read from NDMA_BYTE_CNT_REG."]
    #[inline(always)]
    pub fn remain_byte_read_en(&mut self) -> RemainByteReadEnW<'_, NdmaCfgReg0Spec> {
        RemainByteReadEnW::new(self, 15)
    }
    #[doc = "Bits 16:20 - Destination DRQ Type. 0x01=OWA_TX, 0x04=SPI0_TX, 0x05=SPI1_TX, 0x08=UART0_TX, 0x09=UART1_TX, 0x0A=UART2_TX, 0x0C=AudioCodec_DAC, 0x0E=DAUDIO, 0x10=SRAM, 0x11=SDRAM, 0x14=USB, 0x15=USB_EP1, 0x16=USB_EP2, 0x17=USB_EP3"]
    #[inline(always)]
    pub fn dst_drq_type(&mut self) -> DstDrqTypeW<'_, NdmaCfgReg0Spec> {
        DstDrqTypeW::new(self, 16)
    }
    #[doc = "Bits 21:22 - Destination Address Type. 00=Linear, 01=IO"]
    #[inline(always)]
    pub fn dst_addr_type(&mut self) -> DstAddrTypeW<'_, NdmaCfgReg0Spec> {
        DstAddrTypeW::new(self, 21)
    }
    #[doc = "Bit 23 - Destination Burst Length. 0=1, 1=4"]
    #[inline(always)]
    pub fn dst_burst_len(&mut self) -> DstBurstLenW<'_, NdmaCfgReg0Spec> {
        DstBurstLenW::new(self, 23)
    }
    #[doc = "Bits 24:25 - Destination Data Width. 00=8-bit, 01=16-bit, 10=32-bit"]
    #[inline(always)]
    pub fn dst_data_width(&mut self) -> DstDataWidthW<'_, NdmaCfgReg0Spec> {
        DstDataWidthW::new(self, 24)
    }
    #[doc = "Bits 26:28 - DMA Wait State. 0=1, 1=2, 2=4, 3=8, 4=16, 5=32, 6=64, 7=128"]
    #[inline(always)]
    pub fn dma_wait_state(&mut self) -> DmaWaitStateW<'_, NdmaCfgReg0Spec> {
        DmaWaitStateW::new(self, 26)
    }
    #[doc = "Bit 29 - DMA Continuous Mode Enable. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn dma_continuous(&mut self) -> DmaContinuousW<'_, NdmaCfgReg0Spec> {
        DmaContinuousW::new(self, 29)
    }
    #[doc = "Bit 31 - DMA Loading. Set 1 to start, auto-cleared when finished. Set 0 to reset channel."]
    #[inline(always)]
    pub fn dma_loading(&mut self) -> DmaLoadingW<'_, NdmaCfgReg0Spec> {
        DmaLoadingW::new(self, 31)
    }
}
#[doc = "Normal DMA Channel 0 Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_cfg_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_cfg_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NdmaCfgReg0Spec;
impl crate::RegisterSpec for NdmaCfgReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ndma_cfg_reg0::R`](R) reader structure"]
impl crate::Readable for NdmaCfgReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ndma_cfg_reg0::W`](W) writer structure"]
impl crate::Writable for NdmaCfgReg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NDMA_CFG_REG0 to value 0"]
impl crate::Resettable for NdmaCfgReg0Spec {}
