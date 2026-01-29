#[doc = "Register `TVE_CLK` reader"]
pub type R = crate::R<TveClkSpec>;
#[doc = "Register `TVE_CLK` writer"]
pub type W = crate::W<TveClkSpec>;
#[doc = "Field `CLK_DIV_RATIO_M` reader - Clock Divide Ratio M (m+1, divider 1-16)"]
pub type ClkDivRatioMR = crate::FieldReader;
#[doc = "Field `CLK_DIV_RATIO_M` writer - Clock Divide Ratio M (m+1, divider 1-16)"]
pub type ClkDivRatioMW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCLK1_SRC_SEL` reader - SClock1 Source: 0=TVE_SCLK2, 1=TVE_SCLK2/2"]
pub type Sclk1SrcSelR = crate::BitReader;
#[doc = "Field `SCLK1_SRC_SEL` writer - SClock1 Source: 0=TVE_SCLK2, 1=TVE_SCLK2/2"]
pub type Sclk1SrcSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLK1_GATING` reader - Gating Special Clock 1"]
pub type Sclk1GatingR = crate::BitReader;
#[doc = "Field `SCLK1_GATING` writer - Gating Special Clock 1"]
pub type Sclk1GatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLK2_SRC_SEL` reader - SClock2 Source: 000=PLL_VIDEO(1X), 010=PLL_VIDEO(2X)"]
pub type Sclk2SrcSelR = crate::FieldReader;
#[doc = "Field `SCLK2_SRC_SEL` writer - SClock2 Source: 000=PLL_VIDEO(1X), 010=PLL_VIDEO(2X)"]
pub type Sclk2SrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SCLK2_GATING` reader - Gating Special Clock 2"]
pub type Sclk2GatingR = crate::BitReader;
#[doc = "Field `SCLK2_GATING` writer - Gating Special Clock 2"]
pub type Sclk2GatingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Clock Divide Ratio M (m+1, divider 1-16)"]
    #[inline(always)]
    pub fn clk_div_ratio_m(&self) -> ClkDivRatioMR {
        ClkDivRatioMR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SClock1 Source: 0=TVE_SCLK2, 1=TVE_SCLK2/2"]
    #[inline(always)]
    pub fn sclk1_src_sel(&self) -> Sclk1SrcSelR {
        Sclk1SrcSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - Gating Special Clock 1"]
    #[inline(always)]
    pub fn sclk1_gating(&self) -> Sclk1GatingR {
        Sclk1GatingR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SClock2 Source: 000=PLL_VIDEO(1X), 010=PLL_VIDEO(2X)"]
    #[inline(always)]
    pub fn sclk2_src_sel(&self) -> Sclk2SrcSelR {
        Sclk2SrcSelR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - Gating Special Clock 2"]
    #[inline(always)]
    pub fn sclk2_gating(&self) -> Sclk2GatingR {
        Sclk2GatingR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Divide Ratio M (m+1, divider 1-16)"]
    #[inline(always)]
    pub fn clk_div_ratio_m(&mut self) -> ClkDivRatioMW<'_, TveClkSpec> {
        ClkDivRatioMW::new(self, 0)
    }
    #[doc = "Bit 8 - SClock1 Source: 0=TVE_SCLK2, 1=TVE_SCLK2/2"]
    #[inline(always)]
    pub fn sclk1_src_sel(&mut self) -> Sclk1SrcSelW<'_, TveClkSpec> {
        Sclk1SrcSelW::new(self, 8)
    }
    #[doc = "Bit 15 - Gating Special Clock 1"]
    #[inline(always)]
    pub fn sclk1_gating(&mut self) -> Sclk1GatingW<'_, TveClkSpec> {
        Sclk1GatingW::new(self, 15)
    }
    #[doc = "Bits 24:26 - SClock2 Source: 000=PLL_VIDEO(1X), 010=PLL_VIDEO(2X)"]
    #[inline(always)]
    pub fn sclk2_src_sel(&mut self) -> Sclk2SrcSelW<'_, TveClkSpec> {
        Sclk2SrcSelW::new(self, 24)
    }
    #[doc = "Bit 31 - Gating Special Clock 2"]
    #[inline(always)]
    pub fn sclk2_gating(&mut self) -> Sclk2GatingW<'_, TveClkSpec> {
        Sclk2GatingW::new(self, 31)
    }
}
#[doc = "TVE Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tve_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tve_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TveClkSpec;
impl crate::RegisterSpec for TveClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tve_clk::R`](R) reader structure"]
impl crate::Readable for TveClkSpec {}
#[doc = "`write(|w| ..)` method takes [`tve_clk::W`](W) writer structure"]
impl crate::Writable for TveClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TVE_CLK to value 0"]
impl crate::Resettable for TveClkSpec {}
