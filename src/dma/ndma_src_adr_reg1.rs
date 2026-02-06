#[doc = "Register `NDMA_SRC_ADR_REG1` reader"]
pub type R = crate::R<NdmaSrcAdrReg1Spec>;
#[doc = "Register `NDMA_SRC_ADR_REG1` writer"]
pub type W = crate::W<NdmaSrcAdrReg1Spec>;
#[doc = "Field `SRC_ADDR` reader - Normal DMA Source Address"]
pub type SrcAddrR = crate::FieldReader<u32>;
#[doc = "Field `SRC_ADDR` writer - Normal DMA Source Address"]
pub type SrcAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Normal DMA Source Address"]
    #[inline(always)]
    pub fn src_addr(&self) -> SrcAddrR {
        SrcAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Normal DMA Source Address"]
    #[inline(always)]
    pub fn src_addr(&mut self) -> SrcAddrW<'_, NdmaSrcAdrReg1Spec> {
        SrcAddrW::new(self, 0)
    }
}
#[doc = "Normal DMA Channel 1 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_src_adr_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_src_adr_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NdmaSrcAdrReg1Spec;
impl crate::RegisterSpec for NdmaSrcAdrReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ndma_src_adr_reg1::R`](R) reader structure"]
impl crate::Readable for NdmaSrcAdrReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`ndma_src_adr_reg1::W`](W) writer structure"]
impl crate::Writable for NdmaSrcAdrReg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NDMA_SRC_ADR_REG1 to value 0"]
impl crate::Resettable for NdmaSrcAdrReg1Spec {}
