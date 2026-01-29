#[doc = "Register `BUS_SOFT_RST0` reader"]
pub type R = crate::R<BusSoftRst0Spec>;
#[doc = "Register `BUS_SOFT_RST0` writer"]
pub type W = crate::W<BusSoftRst0Spec>;
#[doc = "Field `DMA_RST` reader - DMA Reset: 0=Assert, 1=De-assert"]
pub type DmaRstR = crate::BitReader;
#[doc = "Field `DMA_RST` writer - DMA Reset: 0=Assert, 1=De-assert"]
pub type DmaRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD0_RST` reader - SD/MMC 0 Reset: 0=Assert, 1=De-assert"]
pub type Sd0RstR = crate::BitReader;
#[doc = "Field `SD0_RST` writer - SD/MMC 0 Reset: 0=Assert, 1=De-assert"]
pub type Sd0RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD1_RST` reader - SD/MMC 1 Reset: 0=Assert, 1=De-assert"]
pub type Sd1RstR = crate::BitReader;
#[doc = "Field `SD1_RST` writer - SD/MMC 1 Reset: 0=Assert, 1=De-assert"]
pub type Sd1RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDRAM_RST` reader - SDRAM Reset: 0=Assert, 1=De-assert"]
pub type SdramRstR = crate::BitReader;
#[doc = "Field `SDRAM_RST` writer - SDRAM Reset: 0=Assert, 1=De-assert"]
pub type SdramRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI0_RST` reader - SPI0 Reset: 0=Assert, 1=De-assert"]
pub type Spi0RstR = crate::BitReader;
#[doc = "Field `SPI0_RST` writer - SPI0 Reset: 0=Assert, 1=De-assert"]
pub type Spi0RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1_RST` reader - SPI1 Reset: 0=Assert, 1=De-assert"]
pub type Spi1RstR = crate::BitReader;
#[doc = "Field `SPI1_RST` writer - SPI1 Reset: 0=Assert, 1=De-assert"]
pub type Spi1RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_OTG_RST` reader - USB OTG Reset: 0=Assert, 1=De-assert"]
pub type UsbOtgRstR = crate::BitReader;
#[doc = "Field `USB_OTG_RST` writer - USB OTG Reset: 0=Assert, 1=De-assert"]
pub type UsbOtgRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - DMA Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn dma_rst(&self) -> DmaRstR {
        DmaRstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - SD/MMC 0 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn sd0_rst(&self) -> Sd0RstR {
        Sd0RstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SD/MMC 1 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn sd1_rst(&self) -> Sd1RstR {
        Sd1RstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - SDRAM Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn sdram_rst(&self) -> SdramRstR {
        SdramRstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI0 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn spi0_rst(&self) -> Spi0RstR {
        Spi0RstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SPI1 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn spi1_rst(&self) -> Spi1RstR {
        Spi1RstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - USB OTG Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn usb_otg_rst(&self) -> UsbOtgRstR {
        UsbOtgRstR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - DMA Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn dma_rst(&mut self) -> DmaRstW<'_, BusSoftRst0Spec> {
        DmaRstW::new(self, 6)
    }
    #[doc = "Bit 8 - SD/MMC 0 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn sd0_rst(&mut self) -> Sd0RstW<'_, BusSoftRst0Spec> {
        Sd0RstW::new(self, 8)
    }
    #[doc = "Bit 9 - SD/MMC 1 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn sd1_rst(&mut self) -> Sd1RstW<'_, BusSoftRst0Spec> {
        Sd1RstW::new(self, 9)
    }
    #[doc = "Bit 14 - SDRAM Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn sdram_rst(&mut self) -> SdramRstW<'_, BusSoftRst0Spec> {
        SdramRstW::new(self, 14)
    }
    #[doc = "Bit 20 - SPI0 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn spi0_rst(&mut self) -> Spi0RstW<'_, BusSoftRst0Spec> {
        Spi0RstW::new(self, 20)
    }
    #[doc = "Bit 21 - SPI1 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn spi1_rst(&mut self) -> Spi1RstW<'_, BusSoftRst0Spec> {
        Spi1RstW::new(self, 21)
    }
    #[doc = "Bit 24 - USB OTG Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn usb_otg_rst(&mut self) -> UsbOtgRstW<'_, BusSoftRst0Spec> {
        UsbOtgRstW::new(self, 24)
    }
}
#[doc = "Bus Software Reset Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_soft_rst0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_soft_rst0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusSoftRst0Spec;
impl crate::RegisterSpec for BusSoftRst0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_soft_rst0::R`](R) reader structure"]
impl crate::Readable for BusSoftRst0Spec {}
#[doc = "`write(|w| ..)` method takes [`bus_soft_rst0::W`](W) writer structure"]
impl crate::Writable for BusSoftRst0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUS_SOFT_RST0 to value 0"]
impl crate::Resettable for BusSoftRst0Spec {}
