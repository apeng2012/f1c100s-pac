#[doc = "Register `LSR` reader"]
pub type R = crate::R<LsrSpec>;
#[doc = "Field `DR` reader - Data Ready"]
pub type DrR = crate::BitReader;
#[doc = "Field `OE` reader - Overrun Error"]
pub type OeR = crate::BitReader;
#[doc = "Field `PE` reader - Parity Error"]
pub type PeR = crate::BitReader;
#[doc = "Field `FE` reader - Framing Error"]
pub type FeR = crate::BitReader;
#[doc = "Field `BI` reader - Break Interrupt"]
pub type BiR = crate::BitReader;
#[doc = "Field `THRE` reader - TX Holding Register Empty"]
pub type ThreR = crate::BitReader;
#[doc = "Field `TEMT` reader - Transmitter Empty"]
pub type TemtR = crate::BitReader;
#[doc = "Field `RFE` reader - RX FIFO Error"]
pub type RfeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data Ready"]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Error"]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Framing Error"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Break Interrupt"]
    #[inline(always)]
    pub fn bi(&self) -> BiR {
        BiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX Holding Register Empty"]
    #[inline(always)]
    pub fn thre(&self) -> ThreR {
        ThreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty"]
    #[inline(always)]
    pub fn temt(&self) -> TemtR {
        TemtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX FIFO Error"]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Line Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LsrSpec {}
#[doc = "`reset()` method sets LSR to value 0"]
impl crate::Resettable for LsrSpec {}
