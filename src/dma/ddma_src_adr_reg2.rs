#[doc = "Register `DDMA_SRC_ADR_REG2` reader"]
pub type R = crate::R<DdmaSrcAdrReg2Spec>;
#[doc = "Register `DDMA_SRC_ADR_REG2` writer"]
pub type W = crate::W<DdmaSrcAdrReg2Spec>;
#[doc = "Field `SRC_ADDR` reader - Dedicated DMA Source Start Address"]
pub type SrcAddrR = crate::FieldReader<u32>;
#[doc = "Field `SRC_ADDR` writer - Dedicated DMA Source Start Address"]
pub type SrcAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Dedicated DMA Source Start Address"]
    #[inline(always)]
    pub fn src_addr(&self) -> SrcAddrR {
        SrcAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Dedicated DMA Source Start Address"]
    #[inline(always)]
    pub fn src_addr(&mut self) -> SrcAddrW<'_, DdmaSrcAdrReg2Spec> {
        SrcAddrW::new(self, 0)
    }
}
#[doc = "Dedicated DMA Channel 2 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_src_adr_reg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_src_adr_reg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdmaSrcAdrReg2Spec;
impl crate::RegisterSpec for DdmaSrcAdrReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddma_src_adr_reg2::R`](R) reader structure"]
impl crate::Readable for DdmaSrcAdrReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`ddma_src_adr_reg2::W`](W) writer structure"]
impl crate::Writable for DdmaSrcAdrReg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DDMA_SRC_ADR_REG2 to value 0"]
impl crate::Resettable for DdmaSrcAdrReg2Spec {}
