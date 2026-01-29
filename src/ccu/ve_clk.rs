#[doc = "Register `VE_CLK` reader"]
pub type R = crate::R<VeClkSpec>;
#[doc = "Register `VE_CLK` writer"]
pub type W = crate::W<VeClkSpec>;
#[doc = "Field `SCLK_GATING` reader - Gating Special Clock (SCLK=PLL_VE output)"]
pub type SclkGatingR = crate::BitReader;
#[doc = "Field `SCLK_GATING` writer - Gating Special Clock (SCLK=PLL_VE output)"]
pub type SclkGatingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Gating Special Clock (SCLK=PLL_VE output)"]
    #[inline(always)]
    pub fn sclk_gating(&self) -> SclkGatingR {
        SclkGatingR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Gating Special Clock (SCLK=PLL_VE output)"]
    #[inline(always)]
    pub fn sclk_gating(&mut self) -> SclkGatingW<'_, VeClkSpec> {
        SclkGatingW::new(self, 31)
    }
}
#[doc = "VE Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ve_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ve_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VeClkSpec;
impl crate::RegisterSpec for VeClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ve_clk::R`](R) reader structure"]
impl crate::Readable for VeClkSpec {}
#[doc = "`write(|w| ..)` method takes [`ve_clk::W`](W) writer structure"]
impl crate::Writable for VeClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VE_CLK to value 0"]
impl crate::Resettable for VeClkSpec {}
