#[doc = "Register `PLL_DDR_CTRL` reader"]
pub type R = crate::R<PllDdrCtrlSpec>;
#[doc = "Register `PLL_DDR_CTRL` writer"]
pub type W = crate::W<PllDdrCtrlSpec>;
#[doc = "Field `PLL_FACTOR_M` reader - PLL Factor M (M=Factor+1, range 1-4)"]
pub type PllFactorMR = crate::FieldReader;
#[doc = "Field `PLL_FACTOR_M` writer - PLL Factor M (M=Factor+1, range 1-4)"]
pub type PllFactorMW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLL_FACTOR_K` reader - PLL Factor K (K=Factor+1, range 1-4)"]
pub type PllFactorKR = crate::FieldReader;
#[doc = "Field `PLL_FACTOR_K` writer - PLL Factor K (K=Factor+1, range 1-4)"]
pub type PllFactorKW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLL_FACTOR_N` reader - PLL Factor N (N=Factor+1, range 1-32)"]
pub type PllFactorNR = crate::FieldReader;
#[doc = "Field `PLL_FACTOR_N` writer - PLL Factor N (N=Factor+1, range 1-32)"]
pub type PllFactorNW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PLL_DDR_CFG_UPDATE` reader - PLL DDR Configuration Update"]
pub type PllDdrCfgUpdateR = crate::BitReader;
#[doc = "Field `PLL_DDR_CFG_UPDATE` writer - PLL DDR Configuration Update"]
pub type PllDdrCfgUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDRAM_SIGMA_DELTA_EN` reader - SDRAM Sigma-Delta Enable"]
pub type SdramSigmaDeltaEnR = crate::BitReader;
#[doc = "Field `SDRAM_SIGMA_DELTA_EN` writer - SDRAM Sigma-Delta Enable"]
pub type SdramSigmaDeltaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - PLL Lock Status"]
pub type LockR = crate::BitReader;
#[doc = "Field `PLL_EN` reader - PLL Enable"]
pub type PllEnR = crate::BitReader;
#[doc = "Field `PLL_EN` writer - PLL Enable"]
pub type PllEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - PLL Factor M (M=Factor+1, range 1-4)"]
    #[inline(always)]
    pub fn pll_factor_m(&self) -> PllFactorMR {
        PllFactorMR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - PLL Factor K (K=Factor+1, range 1-4)"]
    #[inline(always)]
    pub fn pll_factor_k(&self) -> PllFactorKR {
        PllFactorKR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:12 - PLL Factor N (N=Factor+1, range 1-32)"]
    #[inline(always)]
    pub fn pll_factor_n(&self) -> PllFactorNR {
        PllFactorNR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 20 - PLL DDR Configuration Update"]
    #[inline(always)]
    pub fn pll_ddr_cfg_update(&self) -> PllDdrCfgUpdateR {
        PllDdrCfgUpdateR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - SDRAM Sigma-Delta Enable"]
    #[inline(always)]
    pub fn sdram_sigma_delta_en(&self) -> SdramSigmaDeltaEnR {
        SdramSigmaDeltaEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - PLL Lock Status"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - PLL Enable"]
    #[inline(always)]
    pub fn pll_en(&self) -> PllEnR {
        PllEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL Factor M (M=Factor+1, range 1-4)"]
    #[inline(always)]
    pub fn pll_factor_m(&mut self) -> PllFactorMW<'_, PllDdrCtrlSpec> {
        PllFactorMW::new(self, 0)
    }
    #[doc = "Bits 4:5 - PLL Factor K (K=Factor+1, range 1-4)"]
    #[inline(always)]
    pub fn pll_factor_k(&mut self) -> PllFactorKW<'_, PllDdrCtrlSpec> {
        PllFactorKW::new(self, 4)
    }
    #[doc = "Bits 8:12 - PLL Factor N (N=Factor+1, range 1-32)"]
    #[inline(always)]
    pub fn pll_factor_n(&mut self) -> PllFactorNW<'_, PllDdrCtrlSpec> {
        PllFactorNW::new(self, 8)
    }
    #[doc = "Bit 20 - PLL DDR Configuration Update"]
    #[inline(always)]
    pub fn pll_ddr_cfg_update(&mut self) -> PllDdrCfgUpdateW<'_, PllDdrCtrlSpec> {
        PllDdrCfgUpdateW::new(self, 20)
    }
    #[doc = "Bit 24 - SDRAM Sigma-Delta Enable"]
    #[inline(always)]
    pub fn sdram_sigma_delta_en(&mut self) -> SdramSigmaDeltaEnW<'_, PllDdrCtrlSpec> {
        SdramSigmaDeltaEnW::new(self, 24)
    }
    #[doc = "Bit 31 - PLL Enable"]
    #[inline(always)]
    pub fn pll_en(&mut self) -> PllEnW<'_, PllDdrCtrlSpec> {
        PllEnW::new(self, 31)
    }
}
#[doc = "PLL DDR Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_ddr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_ddr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllDdrCtrlSpec;
impl crate::RegisterSpec for PllDdrCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_ddr_ctrl::R`](R) reader structure"]
impl crate::Readable for PllDdrCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_ddr_ctrl::W`](W) writer structure"]
impl crate::Writable for PllDdrCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_DDR_CTRL to value 0x0c11"]
impl crate::Resettable for PllDdrCtrlSpec {
    const RESET_VALUE: u32 = 0x0c11;
}
