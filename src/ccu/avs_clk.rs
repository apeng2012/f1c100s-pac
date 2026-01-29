#[doc = "Register `AVS_CLK` reader"]
pub type R = crate::R<AvsClkSpec>;
#[doc = "Register `AVS_CLK` writer"]
pub type W = crate::W<AvsClkSpec>;
#[doc = "Field `SCLK_GATING` reader - Gating Special Clock (SCLK=OSC24M)"]
pub type SclkGatingR = crate::BitReader;
#[doc = "Field `SCLK_GATING` writer - Gating Special Clock (SCLK=OSC24M)"]
pub type SclkGatingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Gating Special Clock (SCLK=OSC24M)"]
    #[inline(always)]
    pub fn sclk_gating(&self) -> SclkGatingR {
        SclkGatingR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Gating Special Clock (SCLK=OSC24M)"]
    #[inline(always)]
    pub fn sclk_gating(&mut self) -> SclkGatingW<'_, AvsClkSpec> {
        SclkGatingW::new(self, 31)
    }
}
#[doc = "AVS Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`avs_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`avs_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AvsClkSpec;
impl crate::RegisterSpec for AvsClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`avs_clk::R`](R) reader structure"]
impl crate::Readable for AvsClkSpec {}
#[doc = "`write(|w| ..)` method takes [`avs_clk::W`](W) writer structure"]
impl crate::Writable for AvsClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AVS_CLK to value 0"]
impl crate::Resettable for AvsClkSpec {}
