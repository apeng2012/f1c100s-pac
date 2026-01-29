#[doc = "Register `PLL_VIDEO_BIAS` reader"]
pub type R = crate::R<PllVideoBiasSpec>;
#[doc = "Register `PLL_VIDEO_BIAS` writer"]
pub type W = crate::W<PllVideoBiasSpec>;
#[doc = "Field `PLL_DAMP_FACTOR_CTRL` reader - PLL Damping Factor Control\\[2:0\\]"]
pub type PllDampFactorCtrlR = crate::FieldReader;
#[doc = "Field `PLL_DAMP_FACTOR_CTRL` writer - PLL Damping Factor Control\\[2:0\\]"]
pub type PllDampFactorCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLL_BIAS_CTRL` reader - PLL Bias Control\\[4:0\\]"]
pub type PllBiasCtrlR = crate::FieldReader;
#[doc = "Field `PLL_BIAS_CTRL` writer - PLL Bias Control\\[4:0\\]"]
pub type PllBiasCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PLL_VCO_BIAS_CTRL` reader - PLL VCO Bias Control\\[4:0\\]"]
pub type PllVcoBiasCtrlR = crate::FieldReader;
#[doc = "Field `PLL_VCO_BIAS_CTRL` writer - PLL VCO Bias Control\\[4:0\\]"]
pub type PllVcoBiasCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - PLL Damping Factor Control\\[2:0\\]"]
    #[inline(always)]
    pub fn pll_damp_factor_ctrl(&self) -> PllDampFactorCtrlR {
        PllDampFactorCtrlR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:20 - PLL Bias Control\\[4:0\\]"]
    #[inline(always)]
    pub fn pll_bias_ctrl(&self) -> PllBiasCtrlR {
        PllBiasCtrlR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - PLL VCO Bias Control\\[4:0\\]"]
    #[inline(always)]
    pub fn pll_vco_bias_ctrl(&self) -> PllVcoBiasCtrlR {
        PllVcoBiasCtrlR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PLL Damping Factor Control\\[2:0\\]"]
    #[inline(always)]
    pub fn pll_damp_factor_ctrl(&mut self) -> PllDampFactorCtrlW<'_, PllVideoBiasSpec> {
        PllDampFactorCtrlW::new(self, 0)
    }
    #[doc = "Bits 16:20 - PLL Bias Control\\[4:0\\]"]
    #[inline(always)]
    pub fn pll_bias_ctrl(&mut self) -> PllBiasCtrlW<'_, PllVideoBiasSpec> {
        PllBiasCtrlW::new(self, 16)
    }
    #[doc = "Bits 24:28 - PLL VCO Bias Control\\[4:0\\]"]
    #[inline(always)]
    pub fn pll_vco_bias_ctrl(&mut self) -> PllVcoBiasCtrlW<'_, PllVideoBiasSpec> {
        PllVcoBiasCtrlW::new(self, 24)
    }
}
#[doc = "PLL Video Bias Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_video_bias::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_video_bias::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllVideoBiasSpec;
impl crate::RegisterSpec for PllVideoBiasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_video_bias::R`](R) reader structure"]
impl crate::Readable for PllVideoBiasSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_video_bias::W`](W) writer structure"]
impl crate::Writable for PllVideoBiasSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_VIDEO_BIAS to value 0x1010_0000"]
impl crate::Resettable for PllVideoBiasSpec {
    const RESET_VALUE: u32 = 0x1010_0000;
}
