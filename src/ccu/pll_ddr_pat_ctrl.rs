#[doc = "Register `PLL_DDR_PAT_CTRL` reader"]
pub type R = crate::R<PllDdrPatCtrlSpec>;
#[doc = "Register `PLL_DDR_PAT_CTRL` writer"]
pub type W = crate::W<PllDdrPatCtrlSpec>;
#[doc = "Field `WAVE_BOT` reader - Wave Bottom"]
pub type WaveBotR = crate::FieldReader<u32>;
#[doc = "Field `WAVE_BOT` writer - Wave Bottom"]
pub type WaveBotW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `FREQ` reader - Frequency: 00=31.5KHz, 01=32KHz, 10=32.5KHz, 11=33KHz"]
pub type FreqR = crate::FieldReader;
#[doc = "Field `FREQ` writer - Frequency: 00=31.5KHz, 01=32KHz, 10=32.5KHz, 11=33KHz"]
pub type FreqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WAVE_STEP` reader - Wave Step"]
pub type WaveStepR = crate::FieldReader<u16>;
#[doc = "Field `WAVE_STEP` writer - Wave Step"]
pub type WaveStepW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SPR_FREQ_MODE` reader - Spread Frequency Mode: 00=DC=0, 01=DC=1, 1X=Triangular"]
pub type SprFreqModeR = crate::FieldReader;
#[doc = "Field `SPR_FREQ_MODE` writer - Spread Frequency Mode: 00=DC=0, 01=DC=1, 1X=Triangular"]
pub type SprFreqModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SIG_DELT_PAT_EN` reader - Sigma-Delta Pattern Enable"]
pub type SigDeltPatEnR = crate::BitReader;
#[doc = "Field `SIG_DELT_PAT_EN` writer - Sigma-Delta Pattern Enable"]
pub type SigDeltPatEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:16 - Wave Bottom"]
    #[inline(always)]
    pub fn wave_bot(&self) -> WaveBotR {
        WaveBotR::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 17:18 - Frequency: 00=31.5KHz, 01=32KHz, 10=32.5KHz, 11=33KHz"]
    #[inline(always)]
    pub fn freq(&self) -> FreqR {
        FreqR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 20:28 - Wave Step"]
    #[inline(always)]
    pub fn wave_step(&self) -> WaveStepR {
        WaveStepR::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bits 29:30 - Spread Frequency Mode: 00=DC=0, 01=DC=1, 1X=Triangular"]
    #[inline(always)]
    pub fn spr_freq_mode(&self) -> SprFreqModeR {
        SprFreqModeR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Sigma-Delta Pattern Enable"]
    #[inline(always)]
    pub fn sig_delt_pat_en(&self) -> SigDeltPatEnR {
        SigDeltPatEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - Wave Bottom"]
    #[inline(always)]
    pub fn wave_bot(&mut self) -> WaveBotW<'_, PllDdrPatCtrlSpec> {
        WaveBotW::new(self, 0)
    }
    #[doc = "Bits 17:18 - Frequency: 00=31.5KHz, 01=32KHz, 10=32.5KHz, 11=33KHz"]
    #[inline(always)]
    pub fn freq(&mut self) -> FreqW<'_, PllDdrPatCtrlSpec> {
        FreqW::new(self, 17)
    }
    #[doc = "Bits 20:28 - Wave Step"]
    #[inline(always)]
    pub fn wave_step(&mut self) -> WaveStepW<'_, PllDdrPatCtrlSpec> {
        WaveStepW::new(self, 20)
    }
    #[doc = "Bits 29:30 - Spread Frequency Mode: 00=DC=0, 01=DC=1, 1X=Triangular"]
    #[inline(always)]
    pub fn spr_freq_mode(&mut self) -> SprFreqModeW<'_, PllDdrPatCtrlSpec> {
        SprFreqModeW::new(self, 29)
    }
    #[doc = "Bit 31 - Sigma-Delta Pattern Enable"]
    #[inline(always)]
    pub fn sig_delt_pat_en(&mut self) -> SigDeltPatEnW<'_, PllDdrPatCtrlSpec> {
        SigDeltPatEnW::new(self, 31)
    }
}
#[doc = "PLL DDR Pattern Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_ddr_pat_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_ddr_pat_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllDdrPatCtrlSpec;
impl crate::RegisterSpec for PllDdrPatCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_ddr_pat_ctrl::R`](R) reader structure"]
impl crate::Readable for PllDdrPatCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_ddr_pat_ctrl::W`](W) writer structure"]
impl crate::Writable for PllDdrPatCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_DDR_PAT_CTRL to value 0"]
impl crate::Resettable for PllDdrPatCtrlSpec {}
