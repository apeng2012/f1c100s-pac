#[doc = "Register `DAUDIO_CLK` reader"]
pub type R = crate::R<DaudioClkSpec>;
#[doc = "Register `DAUDIO_CLK` writer"]
pub type W = crate::W<DaudioClkSpec>;
#[doc = "Field `CLK_SRC_SEL` reader - Clock Source: 00=PLL_AUDIO(8X), 01=PLL_AUDIO(8X)/2, 10=PLL_AUDIO(8X)/4, 11=PLL_AUDIO(8X)/8"]
pub type ClkSrcSelR = crate::FieldReader;
#[doc = "Field `CLK_SRC_SEL` writer - Clock Source: 00=PLL_AUDIO(8X), 01=PLL_AUDIO(8X)/2, 10=PLL_AUDIO(8X)/4, 11=PLL_AUDIO(8X)/8"]
pub type ClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCLK_GATING` reader - Gating Special Clock"]
pub type SclkGatingR = crate::BitReader;
#[doc = "Field `SCLK_GATING` writer - Gating Special Clock"]
pub type SclkGatingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:17 - Clock Source: 00=PLL_AUDIO(8X), 01=PLL_AUDIO(8X)/2, 10=PLL_AUDIO(8X)/4, 11=PLL_AUDIO(8X)/8"]
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
    #[doc = "Bits 16:17 - Clock Source: 00=PLL_AUDIO(8X), 01=PLL_AUDIO(8X)/2, 10=PLL_AUDIO(8X)/4, 11=PLL_AUDIO(8X)/8"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> ClkSrcSelW<'_, DaudioClkSpec> {
        ClkSrcSelW::new(self, 16)
    }
    #[doc = "Bit 31 - Gating Special Clock"]
    #[inline(always)]
    pub fn sclk_gating(&mut self) -> SclkGatingW<'_, DaudioClkSpec> {
        SclkGatingW::new(self, 31)
    }
}
#[doc = "DAUDIO Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`daudio_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daudio_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaudioClkSpec;
impl crate::RegisterSpec for DaudioClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daudio_clk::R`](R) reader structure"]
impl crate::Readable for DaudioClkSpec {}
#[doc = "`write(|w| ..)` method takes [`daudio_clk::W`](W) writer structure"]
impl crate::Writable for DaudioClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAUDIO_CLK to value 0"]
impl crate::Resettable for DaudioClkSpec {}
