#[doc = "Register `SDMMC1_CLK` reader"]
pub type R = crate::R<Sdmmc1ClkSpec>;
#[doc = "Register `SDMMC1_CLK` writer"]
pub type W = crate::W<Sdmmc1ClkSpec>;
#[doc = "Field `CLK_DIV_RATIO_M` reader - Clock Divide Ratio M (m+1, divider 1-16)"]
pub type ClkDivRatioMR = crate::FieldReader;
#[doc = "Field `CLK_DIV_RATIO_M` writer - Clock Divide Ratio M (m+1, divider 1-16)"]
pub type ClkDivRatioMW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OUTPUT_CLK_PHASE_CTR` reader - Output Clock Phase Control (0-7)"]
pub type OutputClkPhaseCtrR = crate::FieldReader;
#[doc = "Field `OUTPUT_CLK_PHASE_CTR` writer - Output Clock Phase Control (0-7)"]
pub type OutputClkPhaseCtrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLK_DIV_RATIO_N` reader - Clock Pre-Divide Ratio N (2^n, divider 1/2/4/8)"]
pub type ClkDivRatioNR = crate::FieldReader;
#[doc = "Field `CLK_DIV_RATIO_N` writer - Clock Pre-Divide Ratio N (2^n, divider 1/2/4/8)"]
pub type ClkDivRatioNW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAMPLE_CLK_PHASE_CTR` reader - Sample Clock Phase Control (0-7)"]
pub type SampleClkPhaseCtrR = crate::FieldReader;
#[doc = "Field `SAMPLE_CLK_PHASE_CTR` writer - Sample Clock Phase Control (0-7)"]
pub type SampleClkPhaseCtrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLK_SRC_SEL` reader - Clock Source Select: 00=OSC24M, 01=PLL_PERIPH"]
pub type ClkSrcSelR = crate::FieldReader;
#[doc = "Field `CLK_SRC_SEL` writer - Clock Source Select: 00=OSC24M, 01=PLL_PERIPH"]
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
    #[doc = "Bits 8:10 - Output Clock Phase Control (0-7)"]
    #[inline(always)]
    pub fn output_clk_phase_ctr(&self) -> OutputClkPhaseCtrR {
        OutputClkPhaseCtrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Clock Pre-Divide Ratio N (2^n, divider 1/2/4/8)"]
    #[inline(always)]
    pub fn clk_div_ratio_n(&self) -> ClkDivRatioNR {
        ClkDivRatioNR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Sample Clock Phase Control (0-7)"]
    #[inline(always)]
    pub fn sample_clk_phase_ctr(&self) -> SampleClkPhaseCtrR {
        SampleClkPhaseCtrR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Clock Source Select: 00=OSC24M, 01=PLL_PERIPH"]
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
    pub fn clk_div_ratio_m(&mut self) -> ClkDivRatioMW<'_, Sdmmc1ClkSpec> {
        ClkDivRatioMW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Output Clock Phase Control (0-7)"]
    #[inline(always)]
    pub fn output_clk_phase_ctr(&mut self) -> OutputClkPhaseCtrW<'_, Sdmmc1ClkSpec> {
        OutputClkPhaseCtrW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Clock Pre-Divide Ratio N (2^n, divider 1/2/4/8)"]
    #[inline(always)]
    pub fn clk_div_ratio_n(&mut self) -> ClkDivRatioNW<'_, Sdmmc1ClkSpec> {
        ClkDivRatioNW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Sample Clock Phase Control (0-7)"]
    #[inline(always)]
    pub fn sample_clk_phase_ctr(&mut self) -> SampleClkPhaseCtrW<'_, Sdmmc1ClkSpec> {
        SampleClkPhaseCtrW::new(self, 20)
    }
    #[doc = "Bits 24:25 - Clock Source Select: 00=OSC24M, 01=PLL_PERIPH"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> ClkSrcSelW<'_, Sdmmc1ClkSpec> {
        ClkSrcSelW::new(self, 24)
    }
    #[doc = "Bit 31 - Gating Special Clock"]
    #[inline(always)]
    pub fn sclk_gating(&mut self) -> SclkGatingW<'_, Sdmmc1ClkSpec> {
        SclkGatingW::new(self, 31)
    }
}
#[doc = "SDMMC1 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdmmc1_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc1_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdmmc1ClkSpec;
impl crate::RegisterSpec for Sdmmc1ClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc1_clk::R`](R) reader structure"]
impl crate::Readable for Sdmmc1ClkSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc1_clk::W`](W) writer structure"]
impl crate::Writable for Sdmmc1ClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDMMC1_CLK to value 0"]
impl crate::Resettable for Sdmmc1ClkSpec {}
