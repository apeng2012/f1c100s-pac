#[doc = "Register `FE_CLK` reader"]
pub type R = crate::R<FeClkSpec>;
#[doc = "Register `FE_CLK` writer"]
pub type W = crate::W<FeClkSpec>;
#[doc = "Field `CLK_DIV_RATIO_M` reader - Clock Divide Ratio M (m+1, divider 1-16)"]
pub type ClkDivRatioMR = crate::FieldReader;
#[doc = "Field `CLK_DIV_RATIO_M` writer - Clock Divide Ratio M (m+1, divider 1-16)"]
pub type ClkDivRatioMW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLK_SRC_SEL` reader - Clock Source: 000=PLL_VIDEO, 010=PLL_PERIPH"]
pub type ClkSrcSelR = crate::FieldReader;
#[doc = "Field `CLK_SRC_SEL` writer - Clock Source: 000=PLL_VIDEO, 010=PLL_PERIPH"]
pub type ClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SCLK_GATING` reader - Gating Special Clock"]
pub type SclkGatingR = crate::BitReader;
#[doc = "Field `SCLK_GATING` writer - Gating Special Clock"]
pub type SclkGatingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Clock Divide Ratio M (m+1, divider 1-16)"]
    #[inline(always)]
    pub fn clk_div_ratio_m(&self) -> ClkDivRatioMR {
        ClkDivRatioMR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Clock Source: 000=PLL_VIDEO, 010=PLL_PERIPH"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> ClkSrcSelR {
        ClkSrcSelR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - Gating Special Clock"]
    #[inline(always)]
    pub fn sclk_gating(&self) -> SclkGatingR {
        SclkGatingR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Divide Ratio M (m+1, divider 1-16)"]
    #[inline(always)]
    pub fn clk_div_ratio_m(&mut self) -> ClkDivRatioMW<'_, FeClkSpec> {
        ClkDivRatioMW::new(self, 0)
    }
    #[doc = "Bits 24:26 - Clock Source: 000=PLL_VIDEO, 010=PLL_PERIPH"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> ClkSrcSelW<'_, FeClkSpec> {
        ClkSrcSelW::new(self, 24)
    }
    #[doc = "Bit 31 - Gating Special Clock"]
    #[inline(always)]
    pub fn sclk_gating(&mut self) -> SclkGatingW<'_, FeClkSpec> {
        SclkGatingW::new(self, 31)
    }
}
#[doc = "FE Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fe_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fe_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeClkSpec;
impl crate::RegisterSpec for FeClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fe_clk::R`](R) reader structure"]
impl crate::Readable for FeClkSpec {}
#[doc = "`write(|w| ..)` method takes [`fe_clk::W`](W) writer structure"]
impl crate::Writable for FeClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FE_CLK to value 0"]
impl crate::Resettable for FeClkSpec {}
