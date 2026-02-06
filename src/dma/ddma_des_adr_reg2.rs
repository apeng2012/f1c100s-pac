#[doc = "Register `DDMA_DES_ADR_REG2` reader"]
pub type R = crate::R<DdmaDesAdrReg2Spec>;
#[doc = "Register `DDMA_DES_ADR_REG2` writer"]
pub type W = crate::W<DdmaDesAdrReg2Spec>;
#[doc = "Field `DES_ADDR` reader - Dedicated DMA Destination Start Address"]
pub type DesAddrR = crate::FieldReader<u32>;
#[doc = "Field `DES_ADDR` writer - Dedicated DMA Destination Start Address"]
pub type DesAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Dedicated DMA Destination Start Address"]
    #[inline(always)]
    pub fn des_addr(&self) -> DesAddrR {
        DesAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Dedicated DMA Destination Start Address"]
    #[inline(always)]
    pub fn des_addr(&mut self) -> DesAddrW<'_, DdmaDesAdrReg2Spec> {
        DesAddrW::new(self, 0)
    }
}
#[doc = "Dedicated DMA Channel 2 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_des_adr_reg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_des_adr_reg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdmaDesAdrReg2Spec;
impl crate::RegisterSpec for DdmaDesAdrReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddma_des_adr_reg2::R`](R) reader structure"]
impl crate::Readable for DdmaDesAdrReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`ddma_des_adr_reg2::W`](W) writer structure"]
impl crate::Writable for DdmaDesAdrReg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DDMA_DES_ADR_REG2 to value 0"]
impl crate::Resettable for DdmaDesAdrReg2Spec {}
