#[doc = "Register `PLL_DDR_TUN` reader"]
pub type R = crate::R<PllDdrTunSpec>;
#[doc = "Register `PLL_DDR_TUN` writer"]
pub type W = crate::W<PllDdrTunSpec>;
#[doc = "Field `BOUT` reader - B-Out\\[6:0\\] For Verify (read-only)"]
pub type BoutR = crate::FieldReader;
#[doc = "Field `OD` reader - Reg-Od For Verify"]
pub type OdR = crate::BitReader;
#[doc = "Field `OD` writer - Reg-Od For Verify"]
pub type OdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIN` reader - B-In\\[6:0\\] For Verify"]
pub type BinR = crate::FieldReader;
#[doc = "Field `BIN` writer - B-In\\[6:0\\] For Verify"]
pub type BinW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `OD1` reader - Reg-Od1 For Verify"]
pub type Od1R = crate::BitReader;
#[doc = "Field `OD1` writer - Reg-Od1 For Verify"]
pub type Od1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_INIT_FREQ_CTRL` reader - PLL Initial Frequency Control\\[6:0\\]"]
pub type PllInitFreqCtrlR = crate::FieldReader;
#[doc = "Field `PLL_INIT_FREQ_CTRL` writer - PLL Initial Frequency Control\\[6:0\\]"]
pub type PllInitFreqCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `VCO_RST` reader - VCO Reset In"]
pub type VcoRstR = crate::BitReader;
#[doc = "Field `VCO_RST` writer - VCO Reset In"]
pub type VcoRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_LTIME_CTRL` reader - PLL Lock Time Control\\[2:0\\]"]
pub type PllLtimeCtrlR = crate::FieldReader;
#[doc = "Field `PLL_LTIME_CTRL` writer - PLL Lock Time Control\\[2:0\\]"]
pub type PllLtimeCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VREG1_OUT_EN` reader - Vreg1 Out Enable"]
pub type Vreg1OutEnR = crate::BitReader;
#[doc = "Field `VREG1_OUT_EN` writer - Vreg1 Out Enable"]
pub type Vreg1OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - B-Out\\[6:0\\] For Verify (read-only)"]
    #[inline(always)]
    pub fn bout(&self) -> BoutR {
        BoutR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Reg-Od For Verify"]
    #[inline(always)]
    pub fn od(&self) -> OdR {
        OdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - B-In\\[6:0\\] For Verify"]
    #[inline(always)]
    pub fn bin(&self) -> BinR {
        BinR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Reg-Od1 For Verify"]
    #[inline(always)]
    pub fn od1(&self) -> Od1R {
        Od1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - PLL Initial Frequency Control\\[6:0\\]"]
    #[inline(always)]
    pub fn pll_init_freq_ctrl(&self) -> PllInitFreqCtrlR {
        PllInitFreqCtrlR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - VCO Reset In"]
    #[inline(always)]
    pub fn vco_rst(&self) -> VcoRstR {
        VcoRstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - PLL Lock Time Control\\[2:0\\]"]
    #[inline(always)]
    pub fn pll_ltime_ctrl(&self) -> PllLtimeCtrlR {
        PllLtimeCtrlR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Vreg1 Out Enable"]
    #[inline(always)]
    pub fn vreg1_out_en(&self) -> Vreg1OutEnR {
        Vreg1OutEnR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Reg-Od For Verify"]
    #[inline(always)]
    pub fn od(&mut self) -> OdW<'_, PllDdrTunSpec> {
        OdW::new(self, 7)
    }
    #[doc = "Bits 8:14 - B-In\\[6:0\\] For Verify"]
    #[inline(always)]
    pub fn bin(&mut self) -> BinW<'_, PllDdrTunSpec> {
        BinW::new(self, 8)
    }
    #[doc = "Bit 15 - Reg-Od1 For Verify"]
    #[inline(always)]
    pub fn od1(&mut self) -> Od1W<'_, PllDdrTunSpec> {
        Od1W::new(self, 15)
    }
    #[doc = "Bits 16:22 - PLL Initial Frequency Control\\[6:0\\]"]
    #[inline(always)]
    pub fn pll_init_freq_ctrl(&mut self) -> PllInitFreqCtrlW<'_, PllDdrTunSpec> {
        PllInitFreqCtrlW::new(self, 16)
    }
    #[doc = "Bit 23 - VCO Reset In"]
    #[inline(always)]
    pub fn vco_rst(&mut self) -> VcoRstW<'_, PllDdrTunSpec> {
        VcoRstW::new(self, 23)
    }
    #[doc = "Bits 24:26 - PLL Lock Time Control\\[2:0\\]"]
    #[inline(always)]
    pub fn pll_ltime_ctrl(&mut self) -> PllLtimeCtrlW<'_, PllDdrTunSpec> {
        PllLtimeCtrlW::new(self, 24)
    }
    #[doc = "Bit 28 - Vreg1 Out Enable"]
    #[inline(always)]
    pub fn vreg1_out_en(&mut self) -> Vreg1OutEnW<'_, PllDdrTunSpec> {
        Vreg1OutEnW::new(self, 28)
    }
}
#[doc = "PLL DDR Tuning Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_ddr_tun::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_ddr_tun::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllDdrTunSpec;
impl crate::RegisterSpec for PllDdrTunSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_ddr_tun::R`](R) reader structure"]
impl crate::Readable for PllDdrTunSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_ddr_tun::W`](W) writer structure"]
impl crate::Writable for PllDdrTunSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_DDR_TUN to value 0x1410_1010"]
impl crate::Resettable for PllDdrTunSpec {
    const RESET_VALUE: u32 = 0x1410_1010;
}
