#[doc = "Register `NDMA_DES_ADR_REG2` reader"]
pub type R = crate::R<NdmaDesAdrReg2Spec>;
#[doc = "Register `NDMA_DES_ADR_REG2` writer"]
pub type W = crate::W<NdmaDesAdrReg2Spec>;
#[doc = "Field `DES_ADDR` reader - Normal DMA Destination Address"]
pub type DesAddrR = crate::FieldReader<u32>;
#[doc = "Field `DES_ADDR` writer - Normal DMA Destination Address"]
pub type DesAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Normal DMA Destination Address"]
    #[inline(always)]
    pub fn des_addr(&self) -> DesAddrR {
        DesAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Normal DMA Destination Address"]
    #[inline(always)]
    pub fn des_addr(&mut self) -> DesAddrW<'_, NdmaDesAdrReg2Spec> {
        DesAddrW::new(self, 0)
    }
}
#[doc = "Normal DMA Channel 2 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_des_adr_reg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_des_adr_reg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NdmaDesAdrReg2Spec;
impl crate::RegisterSpec for NdmaDesAdrReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ndma_des_adr_reg2::R`](R) reader structure"]
impl crate::Readable for NdmaDesAdrReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`ndma_des_adr_reg2::W`](W) writer structure"]
impl crate::Writable for NdmaDesAdrReg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NDMA_DES_ADR_REG2 to value 0"]
impl crate::Resettable for NdmaDesAdrReg2Spec {}
