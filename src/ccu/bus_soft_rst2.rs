#[doc = "Register `BUS_SOFT_RST2` reader"]
pub type R = crate::R<BusSoftRst2Spec>;
#[doc = "Register `BUS_SOFT_RST2` writer"]
pub type W = crate::W<BusSoftRst2Spec>;
#[doc = "Field `AUDIO_CODEC_RST` reader - AUDIOCODEC Reset: 0=Assert, 1=De-assert"]
pub type AudioCodecRstR = crate::BitReader;
#[doc = "Field `AUDIO_CODEC_RST` writer - AUDIOCODEC Reset: 0=Assert, 1=De-assert"]
pub type AudioCodecRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OWA_RST` reader - OWA Reset: 0=Assert, 1=De-assert"]
pub type OwaRstR = crate::BitReader;
#[doc = "Field `OWA_RST` writer - OWA Reset: 0=Assert, 1=De-assert"]
pub type OwaRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIR_RST` reader - CIR Reset: 0=Assert, 1=De-assert"]
pub type CirRstR = crate::BitReader;
#[doc = "Field `CIR_RST` writer - CIR Reset: 0=Assert, 1=De-assert"]
pub type CirRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSB_RST` reader - RSB Reset: 0=Assert, 1=De-assert"]
pub type RsbRstR = crate::BitReader;
#[doc = "Field `RSB_RST` writer - RSB Reset: 0=Assert, 1=De-assert"]
pub type RsbRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAUDIO_RST` reader - DAUDIO Reset: 0=Assert, 1=De-assert"]
pub type DaudioRstR = crate::BitReader;
#[doc = "Field `DAUDIO_RST` writer - DAUDIO Reset: 0=Assert, 1=De-assert"]
pub type DaudioRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWI0_RST` reader - TWI0 Reset: 0=Assert, 1=De-assert"]
pub type Twi0RstR = crate::BitReader;
#[doc = "Field `TWI0_RST` writer - TWI0 Reset: 0=Assert, 1=De-assert"]
pub type Twi0RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWI1_RST` reader - TWI1 Reset: 0=Assert, 1=De-assert"]
pub type Twi1RstR = crate::BitReader;
#[doc = "Field `TWI1_RST` writer - TWI1 Reset: 0=Assert, 1=De-assert"]
pub type Twi1RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWI2_RST` reader - TWI2 Reset: 0=Assert, 1=De-assert"]
pub type Twi2RstR = crate::BitReader;
#[doc = "Field `TWI2_RST` writer - TWI2 Reset: 0=Assert, 1=De-assert"]
pub type Twi2RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0_RST` reader - UART0 Reset: 0=Assert, 1=De-assert"]
pub type Uart0RstR = crate::BitReader;
#[doc = "Field `UART0_RST` writer - UART0 Reset: 0=Assert, 1=De-assert"]
pub type Uart0RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1_RST` reader - UART1 Reset: 0=Assert, 1=De-assert"]
pub type Uart1RstR = crate::BitReader;
#[doc = "Field `UART1_RST` writer - UART1 Reset: 0=Assert, 1=De-assert"]
pub type Uart1RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2_RST` reader - UART2 Reset: 0=Assert, 1=De-assert"]
pub type Uart2RstR = crate::BitReader;
#[doc = "Field `UART2_RST` writer - UART2 Reset: 0=Assert, 1=De-assert"]
pub type Uart2RstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AUDIOCODEC Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn audio_codec_rst(&self) -> AudioCodecRstR {
        AudioCodecRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OWA Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn owa_rst(&self) -> OwaRstR {
        OwaRstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CIR Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn cir_rst(&self) -> CirRstR {
        CirRstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSB Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn rsb_rst(&self) -> RsbRstR {
        RsbRstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 12 - DAUDIO Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn daudio_rst(&self) -> DaudioRstR {
        DaudioRstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - TWI0 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn twi0_rst(&self) -> Twi0RstR {
        Twi0RstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TWI1 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn twi1_rst(&self) -> Twi1RstR {
        Twi1RstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TWI2 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn twi2_rst(&self) -> Twi2RstR {
        Twi2RstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - UART0 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn uart0_rst(&self) -> Uart0RstR {
        Uart0RstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - UART1 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn uart1_rst(&self) -> Uart1RstR {
        Uart1RstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - UART2 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn uart2_rst(&self) -> Uart2RstR {
        Uart2RstR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AUDIOCODEC Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn audio_codec_rst(&mut self) -> AudioCodecRstW<'_, BusSoftRst2Spec> {
        AudioCodecRstW::new(self, 0)
    }
    #[doc = "Bit 1 - OWA Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn owa_rst(&mut self) -> OwaRstW<'_, BusSoftRst2Spec> {
        OwaRstW::new(self, 1)
    }
    #[doc = "Bit 2 - CIR Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn cir_rst(&mut self) -> CirRstW<'_, BusSoftRst2Spec> {
        CirRstW::new(self, 2)
    }
    #[doc = "Bit 3 - RSB Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn rsb_rst(&mut self) -> RsbRstW<'_, BusSoftRst2Spec> {
        RsbRstW::new(self, 3)
    }
    #[doc = "Bit 12 - DAUDIO Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn daudio_rst(&mut self) -> DaudioRstW<'_, BusSoftRst2Spec> {
        DaudioRstW::new(self, 12)
    }
    #[doc = "Bit 16 - TWI0 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn twi0_rst(&mut self) -> Twi0RstW<'_, BusSoftRst2Spec> {
        Twi0RstW::new(self, 16)
    }
    #[doc = "Bit 17 - TWI1 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn twi1_rst(&mut self) -> Twi1RstW<'_, BusSoftRst2Spec> {
        Twi1RstW::new(self, 17)
    }
    #[doc = "Bit 18 - TWI2 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn twi2_rst(&mut self) -> Twi2RstW<'_, BusSoftRst2Spec> {
        Twi2RstW::new(self, 18)
    }
    #[doc = "Bit 20 - UART0 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn uart0_rst(&mut self) -> Uart0RstW<'_, BusSoftRst2Spec> {
        Uart0RstW::new(self, 20)
    }
    #[doc = "Bit 21 - UART1 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn uart1_rst(&mut self) -> Uart1RstW<'_, BusSoftRst2Spec> {
        Uart1RstW::new(self, 21)
    }
    #[doc = "Bit 22 - UART2 Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn uart2_rst(&mut self) -> Uart2RstW<'_, BusSoftRst2Spec> {
        Uart2RstW::new(self, 22)
    }
}
#[doc = "Bus Software Reset Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_soft_rst2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_soft_rst2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusSoftRst2Spec;
impl crate::RegisterSpec for BusSoftRst2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_soft_rst2::R`](R) reader structure"]
impl crate::Readable for BusSoftRst2Spec {}
#[doc = "`write(|w| ..)` method takes [`bus_soft_rst2::W`](W) writer structure"]
impl crate::Writable for BusSoftRst2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUS_SOFT_RST2 to value 0"]
impl crate::Resettable for BusSoftRst2Spec {}
