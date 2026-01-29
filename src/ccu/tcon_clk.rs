#[doc = "Register `TCON_CLK` reader"]
pub type R = crate::R<TconClkSpec>;
#[doc = "Register `TCON_CLK` writer"]
pub type W = crate::W<TconClkSpec>;
#[doc = "Field `CLK_SRC_SEL` reader - Clock Source: 000=PLL_VIDEO(1X), 010=PLL_VIDEO(2X)"]
pub type ClkSrcSelR = crate::FieldReader;
#[doc = "Field `CLK_SRC_SEL` writer - Clock Source: 000=PLL_VIDEO(1X), 010=PLL_VIDEO(2X)"]
pub type ClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SCLK_GATING` reader - Gating Special Clock"]
pub type SclkGatingR = crate::BitReader;
#[doc = "Field `SCLK_GATING` writer - Gating Special Clock"]
pub type SclkGatingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 24:26 - Clock Source: 000=PLL_VIDEO(1X), 010=PLL_VIDEO(2X)"]
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
    #[doc = "Bits 24:26 - Clock Source: 000=PLL_VIDEO(1X), 010=PLL_VIDEO(2X)"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> ClkSrcSelW<'_, TconClkSpec> {
        ClkSrcSelW::new(self, 24)
    }
    #[doc = "Bit 31 - Gating Special Clock"]
    #[inline(always)]
    pub fn sclk_gating(&mut self) -> SclkGatingW<'_, TconClkSpec> {
        SclkGatingW::new(self, 31)
    }
}
#[doc = "TCON Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcon_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcon_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TconClkSpec;
impl crate::RegisterSpec for TconClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcon_clk::R`](R) reader structure"]
impl crate::Readable for TconClkSpec {}
#[doc = "`write(|w| ..)` method takes [`tcon_clk::W`](W) writer structure"]
impl crate::Writable for TconClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCON_CLK to value 0"]
impl crate::Resettable for TconClkSpec {}
