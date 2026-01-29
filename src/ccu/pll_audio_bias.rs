#[doc = "Register `PLL_AUDIO_BIAS` reader"]
pub type R = crate::R<PllAudioBiasSpec>;
#[doc = "Register `PLL_AUDIO_BIAS` writer"]
pub type W = crate::W<PllAudioBiasSpec>;
#[doc = "Field `PLL_BIAS_CUR` reader - PLL Bias Current\\[4:0\\]"]
pub type PllBiasCurR = crate::FieldReader;
#[doc = "Field `PLL_BIAS_CUR` writer - PLL Bias Current\\[4:0\\]"]
pub type PllBiasCurW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PLL_VCO_BIAS` reader - PLL VCO Bias Current\\[4:0\\]"]
pub type PllVcoBiasR = crate::FieldReader;
#[doc = "Field `PLL_VCO_BIAS` writer - PLL VCO Bias Current\\[4:0\\]"]
pub type PllVcoBiasW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 16:20 - PLL Bias Current\\[4:0\\]"]
    #[inline(always)]
    pub fn pll_bias_cur(&self) -> PllBiasCurR {
        PllBiasCurR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - PLL VCO Bias Current\\[4:0\\]"]
    #[inline(always)]
    pub fn pll_vco_bias(&self) -> PllVcoBiasR {
        PllVcoBiasR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:20 - PLL Bias Current\\[4:0\\]"]
    #[inline(always)]
    pub fn pll_bias_cur(&mut self) -> PllBiasCurW<'_, PllAudioBiasSpec> {
        PllBiasCurW::new(self, 16)
    }
    #[doc = "Bits 24:28 - PLL VCO Bias Current\\[4:0\\]"]
    #[inline(always)]
    pub fn pll_vco_bias(&mut self) -> PllVcoBiasW<'_, PllAudioBiasSpec> {
        PllVcoBiasW::new(self, 24)
    }
}
#[doc = "PLL Audio Bias Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_audio_bias::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_audio_bias::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllAudioBiasSpec;
impl crate::RegisterSpec for PllAudioBiasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_audio_bias::R`](R) reader structure"]
impl crate::Readable for PllAudioBiasSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_audio_bias::W`](W) writer structure"]
impl crate::Writable for PllAudioBiasSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_AUDIO_BIAS to value 0x1010_0000"]
impl crate::Resettable for PllAudioBiasSpec {
    const RESET_VALUE: u32 = 0x1010_0000;
}
