#[doc = "Register `CSI_CLK` reader"]
pub type R = crate::R<CsiClkSpec>;
#[doc = "Register `CSI_CLK` writer"]
pub type W = crate::W<CsiClkSpec>;
#[doc = "Field `CSI_MCLK_DIV_M` reader - CSI Master Clock Divide Ratio M (m+1, divider 1-16)"]
pub type CsiMclkDivMR = crate::FieldReader;
#[doc = "Field `CSI_MCLK_DIV_M` writer - CSI Master Clock Divide Ratio M (m+1, divider 1-16)"]
pub type CsiMclkDivMW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCLK_SRC_SEL` reader - Master Clock Source: 000=PLL_VIDEO(1X), 101=OSC24M"]
pub type MclkSrcSelR = crate::FieldReader;
#[doc = "Field `MCLK_SRC_SEL` writer - Master Clock Source: 000=PLL_VIDEO(1X), 101=OSC24M"]
pub type MclkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSI_MCLK_GATING` reader - Gating Master Clock"]
pub type CsiMclkGatingR = crate::BitReader;
#[doc = "Field `CSI_MCLK_GATING` writer - Gating Master Clock"]
pub type CsiMclkGatingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - CSI Master Clock Divide Ratio M (m+1, divider 1-16)"]
    #[inline(always)]
    pub fn csi_mclk_div_m(&self) -> CsiMclkDivMR {
        CsiMclkDivMR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Master Clock Source: 000=PLL_VIDEO(1X), 101=OSC24M"]
    #[inline(always)]
    pub fn mclk_src_sel(&self) -> MclkSrcSelR {
        MclkSrcSelR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Gating Master Clock"]
    #[inline(always)]
    pub fn csi_mclk_gating(&self) -> CsiMclkGatingR {
        CsiMclkGatingR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - CSI Master Clock Divide Ratio M (m+1, divider 1-16)"]
    #[inline(always)]
    pub fn csi_mclk_div_m(&mut self) -> CsiMclkDivMW<'_, CsiClkSpec> {
        CsiMclkDivMW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Master Clock Source: 000=PLL_VIDEO(1X), 101=OSC24M"]
    #[inline(always)]
    pub fn mclk_src_sel(&mut self) -> MclkSrcSelW<'_, CsiClkSpec> {
        MclkSrcSelW::new(self, 8)
    }
    #[doc = "Bit 15 - Gating Master Clock"]
    #[inline(always)]
    pub fn csi_mclk_gating(&mut self) -> CsiMclkGatingW<'_, CsiClkSpec> {
        CsiMclkGatingW::new(self, 15)
    }
}
#[doc = "CSI Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csi_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsiClkSpec;
impl crate::RegisterSpec for CsiClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi_clk::R`](R) reader structure"]
impl crate::Readable for CsiClkSpec {}
#[doc = "`write(|w| ..)` method takes [`csi_clk::W`](W) writer structure"]
impl crate::Writable for CsiClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSI_CLK to value 0"]
impl crate::Resettable for CsiClkSpec {}
