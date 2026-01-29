#[doc = "Register `LCR` reader"]
pub type R = crate::R<LcrSpec>;
#[doc = "Register `LCR` writer"]
pub type W = crate::W<LcrSpec>;
#[doc = "Field `DLS` reader - Data Length Select"]
pub type DlsR = crate::FieldReader;
#[doc = "Field `DLS` writer - Data Length Select"]
pub type DlsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STOP` reader - Number of Stop Bits"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Number of Stop Bits"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - Parity Enable"]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - Parity Enable"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPS` reader - Even Parity Select"]
pub type EpsR = crate::FieldReader;
#[doc = "Field `EPS` writer - Even Parity Select"]
pub type EpsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BC` reader - Break Control"]
pub type BcR = crate::BitReader;
#[doc = "Field `BC` writer - Break Control"]
pub type BcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLAB` reader - Divisor Latch Access Bit"]
pub type DlabR = crate::BitReader;
#[doc = "Field `DLAB` writer - Divisor Latch Access Bit"]
pub type DlabW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Data Length Select"]
    #[inline(always)]
    pub fn dls(&self) -> DlsR {
        DlsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Number of Stop Bits"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Enable"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Even Parity Select"]
    #[inline(always)]
    pub fn eps(&self) -> EpsR {
        EpsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Break Control"]
    #[inline(always)]
    pub fn bc(&self) -> BcR {
        BcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn dlab(&self) -> DlabR {
        DlabR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data Length Select"]
    #[inline(always)]
    pub fn dls(&mut self) -> DlsW<'_, LcrSpec> {
        DlsW::new(self, 0)
    }
    #[doc = "Bit 2 - Number of Stop Bits"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, LcrSpec> {
        StopW::new(self, 2)
    }
    #[doc = "Bit 3 - Parity Enable"]
    #[inline(always)]
    pub fn pen(&mut self) -> PenW<'_, LcrSpec> {
        PenW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Even Parity Select"]
    #[inline(always)]
    pub fn eps(&mut self) -> EpsW<'_, LcrSpec> {
        EpsW::new(self, 4)
    }
    #[doc = "Bit 6 - Break Control"]
    #[inline(always)]
    pub fn bc(&mut self) -> BcW<'_, LcrSpec> {
        BcW::new(self, 6)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn dlab(&mut self) -> DlabW<'_, LcrSpec> {
        DlabW::new(self, 7)
    }
}
#[doc = "Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrSpec;
impl crate::RegisterSpec for LcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcr::R`](R) reader structure"]
impl crate::Readable for LcrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcr::W`](W) writer structure"]
impl crate::Writable for LcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCR to value 0"]
impl crate::Resettable for LcrSpec {}
