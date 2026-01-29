#[doc = "Register `CIR_CLK` reader"]
pub type R = crate::R<CirClkSpec>;
#[doc = "Register `CIR_CLK` writer"]
pub type W = crate::W<CirClkSpec>;
#[doc = "Field `CLK_DIV_RATIO_M` reader - Clock Divide Ratio M (m+1, divider 1-16)"]
pub type ClkDivRatioMR = crate::FieldReader;
#[doc = "Field `CLK_DIV_RATIO_M` writer - Clock Divide Ratio M (m+1, divider 1-16)"]
pub type ClkDivRatioMW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLK_DIV_RATIO_N` reader - Clock Pre-divide Ratio N (2^n, divider 1/2/4/8)"]
pub type ClkDivRatioNR = crate::FieldReader;
#[doc = "Field `CLK_DIV_RATIO_N` writer - Clock Pre-divide Ratio N (2^n, divider 1/2/4/8)"]
pub type ClkDivRatioNW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK_SRC_SEL` reader - Clock Source Select: 00=LOSC, 01=OSC24M"]
pub type ClkSrcSelR = crate::FieldReader;
#[doc = "Field `CLK_SRC_SEL` writer - Clock Source Select: 00=LOSC, 01=OSC24M"]
pub type ClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    #[doc = "Bits 16:17 - Clock Pre-divide Ratio N (2^n, divider 1/2/4/8)"]
    #[inline(always)]
    pub fn clk_div_ratio_n(&self) -> ClkDivRatioNR {
        ClkDivRatioNR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Clock Source Select: 00=LOSC, 01=OSC24M"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> ClkSrcSelR {
        ClkSrcSelR::new(((self.bits >> 24) & 3) as u8)
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
    pub fn clk_div_ratio_m(&mut self) -> ClkDivRatioMW<'_, CirClkSpec> {
        ClkDivRatioMW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Clock Pre-divide Ratio N (2^n, divider 1/2/4/8)"]
    #[inline(always)]
    pub fn clk_div_ratio_n(&mut self) -> ClkDivRatioNW<'_, CirClkSpec> {
        ClkDivRatioNW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Clock Source Select: 00=LOSC, 01=OSC24M"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> ClkSrcSelW<'_, CirClkSpec> {
        ClkSrcSelW::new(self, 24)
    }
    #[doc = "Bit 31 - Gating Special Clock"]
    #[inline(always)]
    pub fn sclk_gating(&mut self) -> SclkGatingW<'_, CirClkSpec> {
        SclkGatingW::new(self, 31)
    }
}
#[doc = "CIR Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cir_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cir_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CirClkSpec;
impl crate::RegisterSpec for CirClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cir_clk::R`](R) reader structure"]
impl crate::Readable for CirClkSpec {}
#[doc = "`write(|w| ..)` method takes [`cir_clk::W`](W) writer structure"]
impl crate::Writable for CirClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CIR_CLK to value 0"]
impl crate::Resettable for CirClkSpec {}
