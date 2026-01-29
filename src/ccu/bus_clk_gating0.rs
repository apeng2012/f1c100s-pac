#[doc = "Register `BUS_CLK_GATING0` reader"]
pub type R = crate::R<BusClkGating0Spec>;
#[doc = "Register `BUS_CLK_GATING0` writer"]
pub type W = crate::W<BusClkGating0Spec>;
#[doc = "Field `DMA_GATING` reader - Gating Clock For DMA"]
pub type DmaGatingR = crate::BitReader;
#[doc = "Field `DMA_GATING` writer - Gating Clock For DMA"]
pub type DmaGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD0_GATING` reader - Gating Clock For SD0"]
pub type Sd0GatingR = crate::BitReader;
#[doc = "Field `SD0_GATING` writer - Gating Clock For SD0"]
pub type Sd0GatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD1_GATING` reader - Gating Clock For SD1"]
pub type Sd1GatingR = crate::BitReader;
#[doc = "Field `SD1_GATING` writer - Gating Clock For SD1"]
pub type Sd1GatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDRAM_GATING` reader - Gating Clock For SDRAM"]
pub type SdramGatingR = crate::BitReader;
#[doc = "Field `SDRAM_GATING` writer - Gating Clock For SDRAM"]
pub type SdramGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI0_GATING` reader - Gating Clock For SPI0"]
pub type Spi0GatingR = crate::BitReader;
#[doc = "Field `SPI0_GATING` writer - Gating Clock For SPI0"]
pub type Spi0GatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1_GATING` reader - Gating Clock For SPI1"]
pub type Spi1GatingR = crate::BitReader;
#[doc = "Field `SPI1_GATING` writer - Gating Clock For SPI1"]
pub type Spi1GatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_OTG_GATING` reader - Gating Clock For USB-OTG"]
pub type UsbOtgGatingR = crate::BitReader;
#[doc = "Field `USB_OTG_GATING` writer - Gating Clock For USB-OTG"]
pub type UsbOtgGatingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - Gating Clock For DMA"]
    #[inline(always)]
    pub fn dma_gating(&self) -> DmaGatingR {
        DmaGatingR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Gating Clock For SD0"]
    #[inline(always)]
    pub fn sd0_gating(&self) -> Sd0GatingR {
        Sd0GatingR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Gating Clock For SD1"]
    #[inline(always)]
    pub fn sd1_gating(&self) -> Sd1GatingR {
        Sd1GatingR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Gating Clock For SDRAM"]
    #[inline(always)]
    pub fn sdram_gating(&self) -> SdramGatingR {
        SdramGatingR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 20 - Gating Clock For SPI0"]
    #[inline(always)]
    pub fn spi0_gating(&self) -> Spi0GatingR {
        Spi0GatingR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Gating Clock For SPI1"]
    #[inline(always)]
    pub fn spi1_gating(&self) -> Spi1GatingR {
        Spi1GatingR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Gating Clock For USB-OTG"]
    #[inline(always)]
    pub fn usb_otg_gating(&self) -> UsbOtgGatingR {
        UsbOtgGatingR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Gating Clock For DMA"]
    #[inline(always)]
    pub fn dma_gating(&mut self) -> DmaGatingW<'_, BusClkGating0Spec> {
        DmaGatingW::new(self, 6)
    }
    #[doc = "Bit 8 - Gating Clock For SD0"]
    #[inline(always)]
    pub fn sd0_gating(&mut self) -> Sd0GatingW<'_, BusClkGating0Spec> {
        Sd0GatingW::new(self, 8)
    }
    #[doc = "Bit 9 - Gating Clock For SD1"]
    #[inline(always)]
    pub fn sd1_gating(&mut self) -> Sd1GatingW<'_, BusClkGating0Spec> {
        Sd1GatingW::new(self, 9)
    }
    #[doc = "Bit 14 - Gating Clock For SDRAM"]
    #[inline(always)]
    pub fn sdram_gating(&mut self) -> SdramGatingW<'_, BusClkGating0Spec> {
        SdramGatingW::new(self, 14)
    }
    #[doc = "Bit 20 - Gating Clock For SPI0"]
    #[inline(always)]
    pub fn spi0_gating(&mut self) -> Spi0GatingW<'_, BusClkGating0Spec> {
        Spi0GatingW::new(self, 20)
    }
    #[doc = "Bit 21 - Gating Clock For SPI1"]
    #[inline(always)]
    pub fn spi1_gating(&mut self) -> Spi1GatingW<'_, BusClkGating0Spec> {
        Spi1GatingW::new(self, 21)
    }
    #[doc = "Bit 24 - Gating Clock For USB-OTG"]
    #[inline(always)]
    pub fn usb_otg_gating(&mut self) -> UsbOtgGatingW<'_, BusClkGating0Spec> {
        UsbOtgGatingW::new(self, 24)
    }
}
#[doc = "Bus Clock Gating Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_clk_gating0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_clk_gating0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusClkGating0Spec;
impl crate::RegisterSpec for BusClkGating0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_clk_gating0::R`](R) reader structure"]
impl crate::Readable for BusClkGating0Spec {}
#[doc = "`write(|w| ..)` method takes [`bus_clk_gating0::W`](W) writer structure"]
impl crate::Writable for BusClkGating0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUS_CLK_GATING0 to value 0"]
impl crate::Resettable for BusClkGating0Spec {}
