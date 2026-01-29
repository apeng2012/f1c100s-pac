#[doc = "Register `PLL_AUDIO_CTRL` reader"]
pub type R = crate::R<PllAudioCtrlSpec>;
#[doc = "Register `PLL_AUDIO_CTRL` writer"]
pub type W = crate::W<PllAudioCtrlSpec>;
#[doc = "Field `PLL_PREDIV_M` reader - PLL Pre-div Factor M (M=Factor+1, range 1-32)"]
pub type PllPredivMR = crate::FieldReader;
#[doc = "Field `PLL_PREDIV_M` writer - PLL Pre-div Factor M (M=Factor+1, range 1-32)"]
pub type PllPredivMW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PLL_FACTOR_N` reader - PLL Factor N (N=Factor+1, range 1-128)"]
pub type PllFactorNR = crate::FieldReader;
#[doc = "Field `PLL_FACTOR_N` writer - PLL Factor N (N=Factor+1, range 1-128)"]
pub type PllFactorNW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PLL_SDM_EN` reader - PLL Sigma-Delta Modulation Enable"]
pub type PllSdmEnR = crate::BitReader;
#[doc = "Field `PLL_SDM_EN` writer - PLL Sigma-Delta Modulation Enable"]
pub type PllSdmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - PLL Lock Status"]
pub type LockR = crate::BitReader;
#[doc = "Field `PLL_EN` reader - PLL Enable"]
pub type PllEnR = crate::BitReader;
#[doc = "Field `PLL_EN` writer - PLL Enable"]
pub type PllEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - PLL Pre-div Factor M (M=Factor+1, range 1-32)"]
    #[inline(always)]
    pub fn pll_prediv_m(&self) -> PllPredivMR {
        PllPredivMR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:14 - PLL Factor N (N=Factor+1, range 1-128)"]
    #[inline(always)]
    pub fn pll_factor_n(&self) -> PllFactorNR {
        PllFactorNR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - PLL Sigma-Delta Modulation Enable"]
    #[inline(always)]
    pub fn pll_sdm_en(&self) -> PllSdmEnR {
        PllSdmEnR::new(((self.bits >> 24) & 1) != 0)
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
    #[doc = "Bits 0:4 - PLL Pre-div Factor M (M=Factor+1, range 1-32)"]
    #[inline(always)]
    pub fn pll_prediv_m(&mut self) -> PllPredivMW<'_, PllAudioCtrlSpec> {
        PllPredivMW::new(self, 0)
    }
    #[doc = "Bits 8:14 - PLL Factor N (N=Factor+1, range 1-128)"]
    #[inline(always)]
    pub fn pll_factor_n(&mut self) -> PllFactorNW<'_, PllAudioCtrlSpec> {
        PllFactorNW::new(self, 8)
    }
    #[doc = "Bit 24 - PLL Sigma-Delta Modulation Enable"]
    #[inline(always)]
    pub fn pll_sdm_en(&mut self) -> PllSdmEnW<'_, PllAudioCtrlSpec> {
        PllSdmEnW::new(self, 24)
    }
    #[doc = "Bit 31 - PLL Enable"]
    #[inline(always)]
    pub fn pll_en(&mut self) -> PllEnW<'_, PllAudioCtrlSpec> {
        PllEnW::new(self, 31)
    }
}
#[doc = "PLL Audio Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_audio_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_audio_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllAudioCtrlSpec;
impl crate::RegisterSpec for PllAudioCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_audio_ctrl::R`](R) reader structure"]
impl crate::Readable for PllAudioCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_audio_ctrl::W`](W) writer structure"]
impl crate::Writable for PllAudioCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_AUDIO_CTRL to value 0x5514"]
impl crate::Resettable for PllAudioCtrlSpec {
    const RESET_VALUE: u32 = 0x5514;
}
