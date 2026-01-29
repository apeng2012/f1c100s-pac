#[doc = "Register `PLL_DDR_BIAS` reader"]
pub type R = crate::R<PllDdrBiasSpec>;
#[doc = "Register `PLL_DDR_BIAS` writer"]
pub type W = crate::W<PllDdrBiasSpec>;
#[doc = "Field `PLL_DAMP_FACTOR_CTRL` reader - PLL Damping Factor Control\\[3:0\\]"]
pub type PllDampFactorCtrlR = crate::FieldReader;
#[doc = "Field `PLL_DAMP_FACTOR_CTRL` writer - PLL Damping Factor Control\\[3:0\\]"]
pub type PllDampFactorCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLL_VCO_GAIN_CTRL` reader - PLL VCO Gain Control Bit\\[2:0\\]"]
pub type PllVcoGainCtrlR = crate::FieldReader;
#[doc = "Field `PLL_VCO_GAIN_CTRL` writer - PLL VCO Gain Control Bit\\[2:0\\]"]
pub type PllVcoGainCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLL_BIAS_CUR_CTRL` reader - PLL Bias Current Control"]
pub type PllBiasCurCtrlR = crate::FieldReader;
#[doc = "Field `PLL_BIAS_CUR_CTRL` writer - PLL Bias Current Control"]
pub type PllBiasCurCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PLL_BANDW_CTRL` reader - PLL Band Width Control: 0=Narrow, 1=Wide"]
pub type PllBandwCtrlR = crate::BitReader;
#[doc = "Field `PLL_BANDW_CTRL` writer - PLL Band Width Control: 0=Narrow, 1=Wide"]
pub type PllBandwCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_VCO_GAIN_CTRL_EN` reader - PLL VCO Gain Control Enable"]
pub type PllVcoGainCtrlEnR = crate::BitReader;
#[doc = "Field `PLL_VCO_GAIN_CTRL_EN` writer - PLL VCO Gain Control Enable"]
pub type PllVcoGainCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_VCO_BIAS` reader - PLL VCO Bias\\[3:0\\]"]
pub type PllVcoBiasR = crate::FieldReader;
#[doc = "Field `PLL_VCO_BIAS` writer - PLL VCO Bias\\[3:0\\]"]
pub type PllVcoBiasW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - PLL Damping Factor Control\\[3:0\\]"]
    #[inline(always)]
    pub fn pll_damp_factor_ctrl(&self) -> PllDampFactorCtrlR {
        PllDampFactorCtrlR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - PLL VCO Gain Control Bit\\[2:0\\]"]
    #[inline(always)]
    pub fn pll_vco_gain_ctrl(&self) -> PllVcoGainCtrlR {
        PllVcoGainCtrlR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:20 - PLL Bias Current Control"]
    #[inline(always)]
    pub fn pll_bias_cur_ctrl(&self) -> PllBiasCurCtrlR {
        PllBiasCurCtrlR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - PLL Band Width Control: 0=Narrow, 1=Wide"]
    #[inline(always)]
    pub fn pll_bandw_ctrl(&self) -> PllBandwCtrlR {
        PllBandwCtrlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL VCO Gain Control Enable"]
    #[inline(always)]
    pub fn pll_vco_gain_ctrl_en(&self) -> PllVcoGainCtrlEnR {
        PllVcoGainCtrlEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:31 - PLL VCO Bias\\[3:0\\]"]
    #[inline(always)]
    pub fn pll_vco_bias(&self) -> PllVcoBiasR {
        PllVcoBiasR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PLL Damping Factor Control\\[3:0\\]"]
    #[inline(always)]
    pub fn pll_damp_factor_ctrl(&mut self) -> PllDampFactorCtrlW<'_, PllDdrBiasSpec> {
        PllDampFactorCtrlW::new(self, 0)
    }
    #[doc = "Bits 12:14 - PLL VCO Gain Control Bit\\[2:0\\]"]
    #[inline(always)]
    pub fn pll_vco_gain_ctrl(&mut self) -> PllVcoGainCtrlW<'_, PllDdrBiasSpec> {
        PllVcoGainCtrlW::new(self, 12)
    }
    #[doc = "Bits 16:20 - PLL Bias Current Control"]
    #[inline(always)]
    pub fn pll_bias_cur_ctrl(&mut self) -> PllBiasCurCtrlW<'_, PllDdrBiasSpec> {
        PllBiasCurCtrlW::new(self, 16)
    }
    #[doc = "Bit 24 - PLL Band Width Control: 0=Narrow, 1=Wide"]
    #[inline(always)]
    pub fn pll_bandw_ctrl(&mut self) -> PllBandwCtrlW<'_, PllDdrBiasSpec> {
        PllBandwCtrlW::new(self, 24)
    }
    #[doc = "Bit 25 - PLL VCO Gain Control Enable"]
    #[inline(always)]
    pub fn pll_vco_gain_ctrl_en(&mut self) -> PllVcoGainCtrlEnW<'_, PllDdrBiasSpec> {
        PllVcoGainCtrlEnW::new(self, 25)
    }
    #[doc = "Bits 28:31 - PLL VCO Bias\\[3:0\\]"]
    #[inline(always)]
    pub fn pll_vco_bias(&mut self) -> PllVcoBiasW<'_, PllDdrBiasSpec> {
        PllVcoBiasW::new(self, 28)
    }
}
#[doc = "PLL DDR Bias Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_ddr_bias::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_ddr_bias::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllDdrBiasSpec;
impl crate::RegisterSpec for PllDdrBiasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_ddr_bias::R`](R) reader structure"]
impl crate::Readable for PllDdrBiasSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_ddr_bias::W`](W) writer structure"]
impl crate::Writable for PllDdrBiasSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_DDR_BIAS to value 0x8110_4000"]
impl crate::Resettable for PllDdrBiasSpec {
    const RESET_VALUE: u32 = 0x8110_4000;
}
