#[doc = "Register `IIR` reader"]
pub type R = crate::R<IirSpec>;
#[doc = "Field `IID` reader - Interrupt ID"]
pub type IidR = crate::FieldReader;
#[doc = "Field `FIFOSE` reader - FIFOs Enabled"]
pub type FifoseR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Interrupt ID"]
    #[inline(always)]
    pub fn iid(&self) -> IidR {
        IidR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - FIFOs Enabled"]
    #[inline(always)]
    pub fn fifose(&self) -> FifoseR {
        FifoseR::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "Interrupt Identity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IirSpec;
impl crate::RegisterSpec for IirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iir::R`](R) reader structure"]
impl crate::Readable for IirSpec {}
#[doc = "`reset()` method sets IIR to value 0"]
impl crate::Resettable for IirSpec {}
