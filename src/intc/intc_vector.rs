#[doc = "Register `INTC_VECTOR` reader"]
pub type R = crate::R<IntcVectorSpec>;
#[doc = "Field `INTC_VECTOR_ADDR` reader - Vector address for the interrupt currently active on the CPU IRQ input"]
pub type IntcVectorAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 2:31 - Vector address for the interrupt currently active on the CPU IRQ input"]
    #[inline(always)]
    pub fn intc_vector_addr(&self) -> IntcVectorAddrR {
        IntcVectorAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
#[doc = "Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_vector::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcVectorSpec;
impl crate::RegisterSpec for IntcVectorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc_vector::R`](R) reader structure"]
impl crate::Readable for IntcVectorSpec {}
#[doc = "`reset()` method sets INTC_VECTOR to value 0"]
impl crate::Resettable for IntcVectorSpec {}
