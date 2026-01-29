#[doc = "Register `PLL_CPU_TUN` reader"]
pub type R = crate::R<PllCpuTunSpec>;
#[doc = "Register `PLL_CPU_TUN` writer"]
pub type W = crate::W<PllCpuTunSpec>;
#[doc = "Field `C_BOUT` reader - C-B-Out\\[6:0\\] For Verify (read-only)"]
pub type CBoutR = crate::FieldReader;
#[doc = "Field `C_OD1` reader - C-Reg-Od1 For Verify"]
pub type COd1R = crate::BitReader;
#[doc = "Field `C_OD1` writer - C-Reg-Od1 For Verify"]
pub type COd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_BIN` reader - C-B-In\\[6:0\\] For Verify"]
pub type CBinR = crate::FieldReader;
#[doc = "Field `C_BIN` writer - C-B-In\\[6:0\\] For Verify"]
pub type CBinW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `C_OD` reader - C-Reg-Od For Verify"]
pub type COdR = crate::BitReader;
#[doc = "Field `C_OD` writer - C-Reg-Od For Verify"]
pub type COdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_INIT_FREQ_CTRL` reader - PLL Initial Frequency Control\\[6:0\\]"]
pub type PllInitFreqCtrlR = crate::FieldReader;
#[doc = "Field `PLL_INIT_FREQ_CTRL` writer - PLL Initial Frequency Control\\[6:0\\]"]
pub type PllInitFreqCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `VCO_GAIN_CTRL` reader - VCO Gain Control Bits\\[2:0\\]"]
pub type VcoGainCtrlR = crate::FieldReader;
#[doc = "Field `VCO_GAIN_CTRL` writer - VCO Gain Control Bits\\[2:0\\]"]
pub type VcoGainCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VCO_GAIN_CTRL_EN` reader - VCO Gain Control Enable"]
pub type VcoGainCtrlEnR = crate::BitReader;
#[doc = "Field `VCO_GAIN_CTRL_EN` writer - VCO Gain Control Enable"]
pub type VcoGainCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_BANDWID_CTRL` reader - PLL Band Width Control: 0=Narrow, 1=Wide"]
pub type PllBandwidCtrlR = crate::BitReader;
#[doc = "Field `PLL_BANDWID_CTRL` writer - PLL Band Width Control: 0=Narrow, 1=Wide"]
pub type PllBandwidCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - C-B-Out\\[6:0\\] For Verify (read-only)"]
    #[inline(always)]
    pub fn c_bout(&self) -> CBoutR {
        CBoutR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - C-Reg-Od1 For Verify"]
    #[inline(always)]
    pub fn c_od1(&self) -> COd1R {
        COd1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - C-B-In\\[6:0\\] For Verify"]
    #[inline(always)]
    pub fn c_bin(&self) -> CBinR {
        CBinR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - C-Reg-Od For Verify"]
    #[inline(always)]
    pub fn c_od(&self) -> COdR {
        COdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - PLL Initial Frequency Control\\[6:0\\]"]
    #[inline(always)]
    pub fn pll_init_freq_ctrl(&self) -> PllInitFreqCtrlR {
        PllInitFreqCtrlR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:25 - VCO Gain Control Bits\\[2:0\\]"]
    #[inline(always)]
    pub fn vco_gain_ctrl(&self) -> VcoGainCtrlR {
        VcoGainCtrlR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26 - VCO Gain Control Enable"]
    #[inline(always)]
    pub fn vco_gain_ctrl_en(&self) -> VcoGainCtrlEnR {
        VcoGainCtrlEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PLL Band Width Control: 0=Narrow, 1=Wide"]
    #[inline(always)]
    pub fn pll_bandwid_ctrl(&self) -> PllBandwidCtrlR {
        PllBandwidCtrlR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - C-Reg-Od1 For Verify"]
    #[inline(always)]
    pub fn c_od1(&mut self) -> COd1W<'_, PllCpuTunSpec> {
        COd1W::new(self, 7)
    }
    #[doc = "Bits 8:14 - C-B-In\\[6:0\\] For Verify"]
    #[inline(always)]
    pub fn c_bin(&mut self) -> CBinW<'_, PllCpuTunSpec> {
        CBinW::new(self, 8)
    }
    #[doc = "Bit 15 - C-Reg-Od For Verify"]
    #[inline(always)]
    pub fn c_od(&mut self) -> COdW<'_, PllCpuTunSpec> {
        COdW::new(self, 15)
    }
    #[doc = "Bits 16:22 - PLL Initial Frequency Control\\[6:0\\]"]
    #[inline(always)]
    pub fn pll_init_freq_ctrl(&mut self) -> PllInitFreqCtrlW<'_, PllCpuTunSpec> {
        PllInitFreqCtrlW::new(self, 16)
    }
    #[doc = "Bits 23:25 - VCO Gain Control Bits\\[2:0\\]"]
    #[inline(always)]
    pub fn vco_gain_ctrl(&mut self) -> VcoGainCtrlW<'_, PllCpuTunSpec> {
        VcoGainCtrlW::new(self, 23)
    }
    #[doc = "Bit 26 - VCO Gain Control Enable"]
    #[inline(always)]
    pub fn vco_gain_ctrl_en(&mut self) -> VcoGainCtrlEnW<'_, PllCpuTunSpec> {
        VcoGainCtrlEnW::new(self, 26)
    }
    #[doc = "Bit 27 - PLL Band Width Control: 0=Narrow, 1=Wide"]
    #[inline(always)]
    pub fn pll_bandwid_ctrl(&mut self) -> PllBandwidCtrlW<'_, PllCpuTunSpec> {
        PllBandwidCtrlW::new(self, 27)
    }
}
#[doc = "PLL CPU Tuning Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_cpu_tun::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cpu_tun::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllCpuTunSpec;
impl crate::RegisterSpec for PllCpuTunSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_cpu_tun::R`](R) reader structure"]
impl crate::Readable for PllCpuTunSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_cpu_tun::W`](W) writer structure"]
impl crate::Writable for PllCpuTunSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_CPU_TUN to value 0x0a10_1010"]
impl crate::Resettable for PllCpuTunSpec {
    const RESET_VALUE: u32 = 0x0a10_1010;
}
