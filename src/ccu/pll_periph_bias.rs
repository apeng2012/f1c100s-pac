#[doc = "Register `PLL_PERIPH_BIAS` reader"]
pub type R = crate::R<PllPeriphBiasSpec>;
#[doc = "Register `PLL_PERIPH_BIAS` writer"]
pub type W = crate::W<PllPeriphBiasSpec>;
#[doc = "Field `PLL_DAMP_FACTOR_CTRL` reader - PLL Damping Factor Control\\[1:0\\]"]
pub type PllDampFactorCtrlR = crate::FieldReader;
#[doc = "Field `PLL_DAMP_FACTOR_CTRL` writer - PLL Damping Factor Control\\[1:0\\]"]
pub type PllDampFactorCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLL_BANDW_CTRL` reader - PLL Band Width Control: 0=Narrow, 1=Wide"]
pub type PllBandwCtrlR = crate::BitReader;
#[doc = "Field `PLL_BANDW_CTRL` writer - PLL Band Width Control: 0=Narrow, 1=Wide"]
pub type PllBandwCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_BIAS_CUR_CTRL` reader - PLL Bias Current Control"]
pub type PllBiasCurCtrlR = crate::FieldReader;
#[doc = "Field `PLL_BIAS_CUR_CTRL` writer - PLL Bias Current Control"]
pub type PllBiasCurCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PLL_VCO_BIAS` reader - PLL VCO Bias\\[4:0\\]"]
pub type PllVcoBiasR = crate::FieldReader;
#[doc = "Field `PLL_VCO_BIAS` writer - PLL VCO Bias\\[4:0\\]"]
pub type PllVcoBiasW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - PLL Damping Factor Control\\[1:0\\]"]
    #[inline(always)]
    pub fn pll_damp_factor_ctrl(&self) -> PllDampFactorCtrlR {
        PllDampFactorCtrlR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - PLL Band Width Control: 0=Narrow, 1=Wide"]
    #[inline(always)]
    pub fn pll_bandw_ctrl(&self) -> PllBandwCtrlR {
        PllBandwCtrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:20 - PLL Bias Current Control"]
    #[inline(always)]
    pub fn pll_bias_cur_ctrl(&self) -> PllBiasCurCtrlR {
        PllBiasCurCtrlR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - PLL VCO Bias\\[4:0\\]"]
    #[inline(always)]
    pub fn pll_vco_bias(&self) -> PllVcoBiasR {
        PllVcoBiasR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL Damping Factor Control\\[1:0\\]"]
    #[inline(always)]
    pub fn pll_damp_factor_ctrl(&mut self) -> PllDampFactorCtrlW<'_, PllPeriphBiasSpec> {
        PllDampFactorCtrlW::new(self, 0)
    }
    #[doc = "Bit 4 - PLL Band Width Control: 0=Narrow, 1=Wide"]
    #[inline(always)]
    pub fn pll_bandw_ctrl(&mut self) -> PllBandwCtrlW<'_, PllPeriphBiasSpec> {
        PllBandwCtrlW::new(self, 4)
    }
    #[doc = "Bits 16:20 - PLL Bias Current Control"]
    #[inline(always)]
    pub fn pll_bias_cur_ctrl(&mut self) -> PllBiasCurCtrlW<'_, PllPeriphBiasSpec> {
        PllBiasCurCtrlW::new(self, 16)
    }
    #[doc = "Bits 24:28 - PLL VCO Bias\\[4:0\\]"]
    #[inline(always)]
    pub fn pll_vco_bias(&mut self) -> PllVcoBiasW<'_, PllPeriphBiasSpec> {
        PllVcoBiasW::new(self, 24)
    }
}
#[doc = "PLL Peripheral Bias Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_periph_bias::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_periph_bias::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllPeriphBiasSpec;
impl crate::RegisterSpec for PllPeriphBiasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_periph_bias::R`](R) reader structure"]
impl crate::Readable for PllPeriphBiasSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_periph_bias::W`](W) writer structure"]
impl crate::Writable for PllPeriphBiasSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_PERIPH_BIAS to value 0x1010_0010"]
impl crate::Resettable for PllPeriphBiasSpec {
    const RESET_VALUE: u32 = 0x1010_0010;
}
