#[doc = "Register `AUDIO_CODEC_CLK` reader"]
pub type R = crate::R<AudioCodecClkSpec>;
#[doc = "Register `AUDIO_CODEC_CLK` writer"]
pub type W = crate::W<AudioCodecClkSpec>;
#[doc = "Field `SCLK_GATING` reader - Gating Special Clock (SCLK=PLL_AUDIO output)"]
pub type SclkGatingR = crate::BitReader;
#[doc = "Field `SCLK_GATING` writer - Gating Special Clock (SCLK=PLL_AUDIO output)"]
pub type SclkGatingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Gating Special Clock (SCLK=PLL_AUDIO output)"]
    #[inline(always)]
    pub fn sclk_gating(&self) -> SclkGatingR {
        SclkGatingR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Gating Special Clock (SCLK=PLL_AUDIO output)"]
    #[inline(always)]
    pub fn sclk_gating(&mut self) -> SclkGatingW<'_, AudioCodecClkSpec> {
        SclkGatingW::new(self, 31)
    }
}
#[doc = "Audio Codec Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_codec_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_codec_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioCodecClkSpec;
impl crate::RegisterSpec for AudioCodecClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_codec_clk::R`](R) reader structure"]
impl crate::Readable for AudioCodecClkSpec {}
#[doc = "`write(|w| ..)` method takes [`audio_codec_clk::W`](W) writer structure"]
impl crate::Writable for AudioCodecClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUDIO_CODEC_CLK to value 0"]
impl crate::Resettable for AudioCodecClkSpec {}
