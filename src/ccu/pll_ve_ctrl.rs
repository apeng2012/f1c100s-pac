#[doc = "Register `PLL_VE_CTRL` reader"]
pub type R = crate::R<PllVeCtrlSpec>;
#[doc = "Register `PLL_VE_CTRL` writer"]
pub type W = crate::W<PllVeCtrlSpec>;
#[doc = "Field `PLL_PREDIV_M` reader - PLL Pre-div Factor M (M=Factor+1, range 1-16)"]
pub type PllPredivMR = crate::FieldReader;
#[doc = "Field `PLL_PREDIV_M` writer - PLL Pre-div Factor M (M=Factor+1, range 1-16)"]
pub type PllPredivMW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLL_FACTOR_N` reader - PLL Factor N (N=Factor+1, range 1-128)"]
pub type PllFactorNR = crate::FieldReader;
#[doc = "Field `PLL_FACTOR_N` writer - PLL Factor N (N=Factor+1, range 1-128)"]
pub type PllFactorNW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PLL_MODE_SEL` reader - PLL Mode Select: 0=Fractional, 1=Integer"]
pub type PllModeSelR = crate::BitReader;
#[doc = "Field `PLL_MODE_SEL` writer - PLL Mode Select: 0=Fractional, 1=Integer"]
pub type PllModeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAC_CLK_OUT` reader - Fractional Clock Output: 0=270MHz, 1=297MHz"]
pub type FracClkOutR = crate::BitReader;
#[doc = "Field `FRAC_CLK_OUT` writer - Fractional Clock Output: 0=270MHz, 1=297MHz"]
pub type FracClkOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - PLL Lock Status"]
pub type LockR = crate::BitReader;
#[doc = "Field `PLL_EN` reader - PLL Enable"]
pub type PllEnR = crate::BitReader;
#[doc = "Field `PLL_EN` writer - PLL Enable"]
pub type PllEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - PLL Pre-div Factor M (M=Factor+1, range 1-16)"]
    #[inline(always)]
    pub fn pll_prediv_m(&self) -> PllPredivMR {
        PllPredivMR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - PLL Factor N (N=Factor+1, range 1-128)"]
    #[inline(always)]
    pub fn pll_factor_n(&self) -> PllFactorNR {
        PllFactorNR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - PLL Mode Select: 0=Fractional, 1=Integer"]
    #[inline(always)]
    pub fn pll_mode_sel(&self) -> PllModeSelR {
        PllModeSelR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Fractional Clock Output: 0=270MHz, 1=297MHz"]
    #[inline(always)]
    pub fn frac_clk_out(&self) -> FracClkOutR {
        FracClkOutR::new(((self.bits >> 25) & 1) != 0)
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
    #[doc = "Bits 0:3 - PLL Pre-div Factor M (M=Factor+1, range 1-16)"]
    #[inline(always)]
    pub fn pll_prediv_m(&mut self) -> PllPredivMW<'_, PllVeCtrlSpec> {
        PllPredivMW::new(self, 0)
    }
    #[doc = "Bits 8:14 - PLL Factor N (N=Factor+1, range 1-128)"]
    #[inline(always)]
    pub fn pll_factor_n(&mut self) -> PllFactorNW<'_, PllVeCtrlSpec> {
        PllFactorNW::new(self, 8)
    }
    #[doc = "Bit 24 - PLL Mode Select: 0=Fractional, 1=Integer"]
    #[inline(always)]
    pub fn pll_mode_sel(&mut self) -> PllModeSelW<'_, PllVeCtrlSpec> {
        PllModeSelW::new(self, 24)
    }
    #[doc = "Bit 25 - Fractional Clock Output: 0=270MHz, 1=297MHz"]
    #[inline(always)]
    pub fn frac_clk_out(&mut self) -> FracClkOutW<'_, PllVeCtrlSpec> {
        FracClkOutW::new(self, 25)
    }
    #[doc = "Bit 31 - PLL Enable"]
    #[inline(always)]
    pub fn pll_en(&mut self) -> PllEnW<'_, PllVeCtrlSpec> {
        PllEnW::new(self, 31)
    }
}
#[doc = "PLL VE Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_ve_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_ve_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllVeCtrlSpec;
impl crate::RegisterSpec for PllVeCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_ve_ctrl::R`](R) reader structure"]
impl crate::Readable for PllVeCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_ve_ctrl::W`](W) writer structure"]
impl crate::Writable for PllVeCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_VE_CTRL to value 0x0300_4507"]
impl crate::Resettable for PllVeCtrlSpec {
    const RESET_VALUE: u32 = 0x0300_4507;
}
