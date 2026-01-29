#[doc = "Register `OWA_CLK` reader"]
pub type R = crate::R<OwaClkSpec>;
#[doc = "Register `OWA_CLK` writer"]
pub type W = crate::W<OwaClkSpec>;
#[doc = "Field `CLK_SRC_SEL` reader - Clock Source: 00=PLL2(8X), 01=PLL2(8X)/2, 10=PLL2(8X)/4, 11=PLL2(8X)/8"]
pub type ClkSrcSelR = crate::FieldReader;
#[doc = "Field `CLK_SRC_SEL` writer - Clock Source: 00=PLL2(8X), 01=PLL2(8X)/2, 10=PLL2(8X)/4, 11=PLL2(8X)/8"]
pub type ClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCLK_GATING` reader - Gating Special Clock"]
pub type SclkGatingR = crate::BitReader;
#[doc = "Field `SCLK_GATING` writer - Gating Special Clock"]
pub type SclkGatingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:17 - Clock Source: 00=PLL2(8X), 01=PLL2(8X)/2, 10=PLL2(8X)/4, 11=PLL2(8X)/8"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> ClkSrcSelR {
        ClkSrcSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 31 - Gating Special Clock"]
    #[inline(always)]
    pub fn sclk_gating(&self) -> SclkGatingR {
        SclkGatingR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Clock Source: 00=PLL2(8X), 01=PLL2(8X)/2, 10=PLL2(8X)/4, 11=PLL2(8X)/8"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> ClkSrcSelW<'_, OwaClkSpec> {
        ClkSrcSelW::new(self, 16)
    }
    #[doc = "Bit 31 - Gating Special Clock"]
    #[inline(always)]
    pub fn sclk_gating(&mut self) -> SclkGatingW<'_, OwaClkSpec> {
        SclkGatingW::new(self, 31)
    }
}
#[doc = "OWA Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`owa_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`owa_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OwaClkSpec;
impl crate::RegisterSpec for OwaClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`owa_clk::R`](R) reader structure"]
impl crate::Readable for OwaClkSpec {}
#[doc = "`write(|w| ..)` method takes [`owa_clk::W`](W) writer structure"]
impl crate::Writable for OwaClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OWA_CLK to value 0x0001_0000"]
impl crate::Resettable for OwaClkSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
