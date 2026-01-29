#[doc = "Register `PLL_CPU_BIAS` reader"]
pub type R = crate::R<PllCpuBiasSpec>;
#[doc = "Register `PLL_CPU_BIAS` writer"]
pub type W = crate::W<PllCpuBiasSpec>;
#[doc = "Field `PLL_DAMP_FACT_CTRL` reader - PLL Damping Factor Control\\[3:0\\]"]
pub type PllDampFactCtrlR = crate::FieldReader;
#[doc = "Field `PLL_DAMP_FACT_CTRL` writer - PLL Damping Factor Control\\[3:0\\]"]
pub type PllDampFactCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLL_LOCK_CTRL` reader - PLL Lock Time Control\\[2:0\\]"]
pub type PllLockCtrlR = crate::FieldReader;
#[doc = "Field `PLL_LOCK_CTRL` writer - PLL Lock Time Control\\[2:0\\]"]
pub type PllLockCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLL_BIAS_CUR_CTRL` reader - PLL Bias Current Control\\[4:0\\]"]
pub type PllBiasCurCtrlR = crate::FieldReader;
#[doc = "Field `PLL_BIAS_CUR_CTRL` writer - PLL Bias Current Control\\[4:0\\]"]
pub type PllBiasCurCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PLL_VCO_BIAS_CTRL` reader - PLL VCO Bias Control\\[3:0\\]"]
pub type PllVcoBiasCtrlR = crate::FieldReader;
#[doc = "Field `PLL_VCO_BIAS_CTRL` writer - PLL VCO Bias Control\\[3:0\\]"]
pub type PllVcoBiasCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VCO_RST` reader - VCO Reset In"]
pub type VcoRstR = crate::BitReader;
#[doc = "Field `VCO_RST` writer - VCO Reset In"]
pub type VcoRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - PLL Damping Factor Control\\[3:0\\]"]
    #[inline(always)]
    pub fn pll_damp_fact_ctrl(&self) -> PllDampFactCtrlR {
        PllDampFactCtrlR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - PLL Lock Time Control\\[2:0\\]"]
    #[inline(always)]
    pub fn pll_lock_ctrl(&self) -> PllLockCtrlR {
        PllLockCtrlR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:20 - PLL Bias Current Control\\[4:0\\]"]
    #[inline(always)]
    pub fn pll_bias_cur_ctrl(&self) -> PllBiasCurCtrlR {
        PllBiasCurCtrlR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - PLL VCO Bias Control\\[3:0\\]"]
    #[inline(always)]
    pub fn pll_vco_bias_ctrl(&self) -> PllVcoBiasCtrlR {
        PllVcoBiasCtrlR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - VCO Reset In"]
    #[inline(always)]
    pub fn vco_rst(&self) -> VcoRstR {
        VcoRstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PLL Damping Factor Control\\[3:0\\]"]
    #[inline(always)]
    pub fn pll_damp_fact_ctrl(&mut self) -> PllDampFactCtrlW<'_, PllCpuBiasSpec> {
        PllDampFactCtrlW::new(self, 0)
    }
    #[doc = "Bits 8:10 - PLL Lock Time Control\\[2:0\\]"]
    #[inline(always)]
    pub fn pll_lock_ctrl(&mut self) -> PllLockCtrlW<'_, PllCpuBiasSpec> {
        PllLockCtrlW::new(self, 8)
    }
    #[doc = "Bits 16:20 - PLL Bias Current Control\\[4:0\\]"]
    #[inline(always)]
    pub fn pll_bias_cur_ctrl(&mut self) -> PllBiasCurCtrlW<'_, PllCpuBiasSpec> {
        PllBiasCurCtrlW::new(self, 16)
    }
    #[doc = "Bits 24:27 - PLL VCO Bias Control\\[3:0\\]"]
    #[inline(always)]
    pub fn pll_vco_bias_ctrl(&mut self) -> PllVcoBiasCtrlW<'_, PllCpuBiasSpec> {
        PllVcoBiasCtrlW::new(self, 24)
    }
    #[doc = "Bit 31 - VCO Reset In"]
    #[inline(always)]
    pub fn vco_rst(&mut self) -> VcoRstW<'_, PllCpuBiasSpec> {
        VcoRstW::new(self, 31)
    }
}
#[doc = "PLL CPU Bias Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_cpu_bias::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cpu_bias::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllCpuBiasSpec;
impl crate::RegisterSpec for PllCpuBiasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_cpu_bias::R`](R) reader structure"]
impl crate::Readable for PllCpuBiasSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_cpu_bias::W`](W) writer structure"]
impl crate::Writable for PllCpuBiasSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_CPU_BIAS to value 0x0810_0200"]
impl crate::Resettable for PllCpuBiasSpec {
    const RESET_VALUE: u32 = 0x0810_0200;
}
