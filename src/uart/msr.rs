#[doc = "Register `MSR` reader"]
pub type R = crate::R<MsrSpec>;
#[doc = "Field `DCTS` reader - Delta Clear to Send"]
pub type DctsR = crate::BitReader;
#[doc = "Field `DDSR` reader - Delta Data Set Ready"]
pub type DdsrR = crate::BitReader;
#[doc = "Field `TERI` reader - Trailing Edge Ring Indicator"]
pub type TeriR = crate::BitReader;
#[doc = "Field `DDCD` reader - Delta Data Carrier Detect"]
pub type DdcdR = crate::BitReader;
#[doc = "Field `CTS` reader - Clear to Send"]
pub type CtsR = crate::BitReader;
#[doc = "Field `DSR` reader - Data Set Ready"]
pub type DsrR = crate::BitReader;
#[doc = "Field `RI` reader - Ring Indicator"]
pub type RiR = crate::BitReader;
#[doc = "Field `DCD` reader - Data Carrier Detect"]
pub type DcdR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Delta Clear to Send"]
    #[inline(always)]
    pub fn dcts(&self) -> DctsR {
        DctsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Delta Data Set Ready"]
    #[inline(always)]
    pub fn ddsr(&self) -> DdsrR {
        DdsrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trailing Edge Ring Indicator"]
    #[inline(always)]
    pub fn teri(&self) -> TeriR {
        TeriR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Delta Data Carrier Detect"]
    #[inline(always)]
    pub fn ddcd(&self) -> DdcdR {
        DdcdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear to Send"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Set Ready"]
    #[inline(always)]
    pub fn dsr(&self) -> DsrR {
        DsrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Ring Indicator"]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Carrier Detect"]
    #[inline(always)]
    pub fn dcd(&self) -> DcdR {
        DcdR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Modem Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsrSpec;
impl crate::RegisterSpec for MsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MsrSpec {}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MsrSpec {}
