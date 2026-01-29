#[doc = "Register `BUS_CLK_GATING2` reader"]
pub type R = crate::R<BusClkGating2Spec>;
#[doc = "Register `BUS_CLK_GATING2` writer"]
pub type W = crate::W<BusClkGating2Spec>;
#[doc = "Field `AUDIO_CODEC_GATING` reader - Gating Clock For AUDIOCODEC"]
pub type AudioCodecGatingR = crate::BitReader;
#[doc = "Field `AUDIO_CODEC_GATING` writer - Gating Clock For AUDIOCODEC"]
pub type AudioCodecGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OWA_GATING` reader - Gating Clock For OWA"]
pub type OwaGatingR = crate::BitReader;
#[doc = "Field `OWA_GATING` writer - Gating Clock For OWA"]
pub type OwaGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIR_GATING` reader - Gating Clock For CIR"]
pub type CirGatingR = crate::BitReader;
#[doc = "Field `CIR_GATING` writer - Gating Clock For CIR"]
pub type CirGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSB_GATING` reader - Gating Clock For RSB"]
pub type RsbGatingR = crate::BitReader;
#[doc = "Field `RSB_GATING` writer - Gating Clock For RSB"]
pub type RsbGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAUDIO_GATING` reader - Gating Clock For DAUDIO"]
pub type DaudioGatingR = crate::BitReader;
#[doc = "Field `DAUDIO_GATING` writer - Gating Clock For DAUDIO"]
pub type DaudioGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWI0_GATING` reader - Gating Clock For TWI0"]
pub type Twi0GatingR = crate::BitReader;
#[doc = "Field `TWI0_GATING` writer - Gating Clock For TWI0"]
pub type Twi0GatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWI1_GATING` reader - Gating Clock For TWI1"]
pub type Twi1GatingR = crate::BitReader;
#[doc = "Field `TWI1_GATING` writer - Gating Clock For TWI1"]
pub type Twi1GatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWI2_GATING` reader - Gating Clock For TWI2"]
pub type Twi2GatingR = crate::BitReader;
#[doc = "Field `TWI2_GATING` writer - Gating Clock For TWI2"]
pub type Twi2GatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0_GATING` reader - Gating Clock For UART0"]
pub type Uart0GatingR = crate::BitReader;
#[doc = "Field `UART0_GATING` writer - Gating Clock For UART0"]
pub type Uart0GatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_GATING` reader - Gating Clock For UART1"]
pub type Uart1GatingR = crate::BitReader;
#[doc = "Field `UART1_GATING` writer - Gating Clock For UART1"]
pub type Uart1GatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2_GATING` reader - Gating Clock For UART2"]
pub type Uart2GatingR = crate::BitReader;
#[doc = "Field `UART2_GATING` writer - Gating Clock For UART2"]
pub type Uart2GatingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Gating Clock For AUDIOCODEC"]
    #[inline(always)]
    pub fn audio_codec_gating(&self) -> AudioCodecGatingR {
        AudioCodecGatingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gating Clock For OWA"]
    #[inline(always)]
    pub fn owa_gating(&self) -> OwaGatingR {
        OwaGatingR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gating Clock For CIR"]
    #[inline(always)]
    pub fn cir_gating(&self) -> CirGatingR {
        CirGatingR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Gating Clock For RSB"]
    #[inline(always)]
    pub fn rsb_gating(&self) -> RsbGatingR {
        RsbGatingR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 12 - Gating Clock For DAUDIO"]
    #[inline(always)]
    pub fn daudio_gating(&self) -> DaudioGatingR {
        DaudioGatingR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Gating Clock For TWI0"]
    #[inline(always)]
    pub fn twi0_gating(&self) -> Twi0GatingR {
        Twi0GatingR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Gating Clock For TWI1"]
    #[inline(always)]
    pub fn twi1_gating(&self) -> Twi1GatingR {
        Twi1GatingR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Gating Clock For TWI2"]
    #[inline(always)]
    pub fn twi2_gating(&self) -> Twi2GatingR {
        Twi2GatingR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Gating Clock For UART0"]
    #[inline(always)]
    pub fn uart0_gating(&self) -> Uart0GatingR {
        Uart0GatingR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Gating Clock For UART1"]
    #[inline(always)]
    pub fn uart1_gating(&self) -> Uart1GatingR {
        Uart1GatingR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Gating Clock For UART2"]
    #[inline(always)]
    pub fn uart2_gating(&self) -> Uart2GatingR {
        Uart2GatingR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gating Clock For AUDIOCODEC"]
    #[inline(always)]
    pub fn audio_codec_gating(&mut self) -> AudioCodecGatingW<'_, BusClkGating2Spec> {
        AudioCodecGatingW::new(self, 0)
    }
    #[doc = "Bit 1 - Gating Clock For OWA"]
    #[inline(always)]
    pub fn owa_gating(&mut self) -> OwaGatingW<'_, BusClkGating2Spec> {
        OwaGatingW::new(self, 1)
    }
    #[doc = "Bit 2 - Gating Clock For CIR"]
    #[inline(always)]
    pub fn cir_gating(&mut self) -> CirGatingW<'_, BusClkGating2Spec> {
        CirGatingW::new(self, 2)
    }
    #[doc = "Bit 3 - Gating Clock For RSB"]
    #[inline(always)]
    pub fn rsb_gating(&mut self) -> RsbGatingW<'_, BusClkGating2Spec> {
        RsbGatingW::new(self, 3)
    }
    #[doc = "Bit 12 - Gating Clock For DAUDIO"]
    #[inline(always)]
    pub fn daudio_gating(&mut self) -> DaudioGatingW<'_, BusClkGating2Spec> {
        DaudioGatingW::new(self, 12)
    }
    #[doc = "Bit 16 - Gating Clock For TWI0"]
    #[inline(always)]
    pub fn twi0_gating(&mut self) -> Twi0GatingW<'_, BusClkGating2Spec> {
        Twi0GatingW::new(self, 16)
    }
    #[doc = "Bit 17 - Gating Clock For TWI1"]
    #[inline(always)]
    pub fn twi1_gating(&mut self) -> Twi1GatingW<'_, BusClkGating2Spec> {
        Twi1GatingW::new(self, 17)
    }
    #[doc = "Bit 18 - Gating Clock For TWI2"]
    #[inline(always)]
    pub fn twi2_gating(&mut self) -> Twi2GatingW<'_, BusClkGating2Spec> {
        Twi2GatingW::new(self, 18)
    }
    #[doc = "Bit 20 - Gating Clock For UART0"]
    #[inline(always)]
    pub fn uart0_gating(&mut self) -> Uart0GatingW<'_, BusClkGating2Spec> {
        Uart0GatingW::new(self, 20)
    }
    #[doc = "Bit 21 - Gating Clock For UART1"]
    #[inline(always)]
    pub fn uart1_gating(&mut self) -> Uart1GatingW<'_, BusClkGating2Spec> {
        Uart1GatingW::new(self, 21)
    }
    #[doc = "Bit 22 - Gating Clock For UART2"]
    #[inline(always)]
    pub fn uart2_gating(&mut self) -> Uart2GatingW<'_, BusClkGating2Spec> {
        Uart2GatingW::new(self, 22)
    }
}
#[doc = "Bus Clock Gating Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_clk_gating2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_clk_gating2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusClkGating2Spec;
impl crate::RegisterSpec for BusClkGating2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_clk_gating2::R`](R) reader structure"]
impl crate::Readable for BusClkGating2Spec {}
#[doc = "`write(|w| ..)` method takes [`bus_clk_gating2::W`](W) writer structure"]
impl crate::Writable for BusClkGating2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUS_CLK_GATING2 to value 0"]
impl crate::Resettable for BusClkGating2Spec {}
