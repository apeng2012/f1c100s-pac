#[doc = "Register `INTC_BASE_ADDR` reader"]
pub type R = crate::R<IntcBaseAddrSpec>;
#[doc = "Register `INTC_BASE_ADDR` writer"]
pub type W = crate::W<IntcBaseAddrSpec>;
#[doc = "Field `INTC_BASE_ADDR` reader - Upper 30 bits of the base address of the vector table"]
pub type IntcBaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `INTC_BASE_ADDR` writer - Upper 30 bits of the base address of the vector table"]
pub type IntcBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Upper 30 bits of the base address of the vector table"]
    #[inline(always)]
    pub fn intc_base_addr(&self) -> IntcBaseAddrR {
        IntcBaseAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Upper 30 bits of the base address of the vector table"]
    #[inline(always)]
    pub fn intc_base_addr(&mut self) -> IntcBaseAddrW<'_, IntcBaseAddrSpec> {
        IntcBaseAddrW::new(self, 2)
    }
}
#[doc = "Interrupt Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_base_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_base_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcBaseAddrSpec;
impl crate::RegisterSpec for IntcBaseAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc_base_addr::R`](R) reader structure"]
impl crate::Readable for IntcBaseAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`intc_base_addr::W`](W) writer structure"]
impl crate::Writable for IntcBaseAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC_BASE_ADDR to value 0"]
impl crate::Resettable for IntcBaseAddrSpec {}
