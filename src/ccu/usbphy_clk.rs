#[doc = "Register `USBPHY_CLK` reader"]
pub type R = crate::R<UsbphyClkSpec>;
#[doc = "Register `USBPHY_CLK` writer"]
pub type W = crate::W<UsbphyClkSpec>;
#[doc = "Field `USBPHY_RST` reader - USB PHY Reset Control: 0=Assert, 1=De-assert"]
pub type UsbphyRstR = crate::BitReader;
#[doc = "Field `USBPHY_RST` writer - USB PHY Reset Control: 0=Assert, 1=De-assert"]
pub type UsbphyRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLK_GATING` reader - USBPHY 24MHz Clock Gating"]
pub type SclkGatingR = crate::BitReader;
#[doc = "Field `SCLK_GATING` writer - USBPHY 24MHz Clock Gating"]
pub type SclkGatingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB PHY Reset Control: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn usbphy_rst(&self) -> UsbphyRstR {
        UsbphyRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USBPHY 24MHz Clock Gating"]
    #[inline(always)]
    pub fn sclk_gating(&self) -> SclkGatingR {
        SclkGatingR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB PHY Reset Control: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn usbphy_rst(&mut self) -> UsbphyRstW<'_, UsbphyClkSpec> {
        UsbphyRstW::new(self, 0)
    }
    #[doc = "Bit 1 - USBPHY 24MHz Clock Gating"]
    #[inline(always)]
    pub fn sclk_gating(&mut self) -> SclkGatingW<'_, UsbphyClkSpec> {
        SclkGatingW::new(self, 1)
    }
}
#[doc = "USBPHY Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbphy_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphy_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbphyClkSpec;
impl crate::RegisterSpec for UsbphyClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbphy_clk::R`](R) reader structure"]
impl crate::Readable for UsbphyClkSpec {}
#[doc = "`write(|w| ..)` method takes [`usbphy_clk::W`](W) writer structure"]
impl crate::Writable for UsbphyClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBPHY_CLK to value 0"]
impl crate::Resettable for UsbphyClkSpec {}
