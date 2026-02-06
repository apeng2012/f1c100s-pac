#[doc = "Register `DDMA_SRC_ADR_REG0` reader"]
pub type R = crate::R<DdmaSrcAdrReg0Spec>;
#[doc = "Register `DDMA_SRC_ADR_REG0` writer"]
pub type W = crate::W<DdmaSrcAdrReg0Spec>;
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
    pub fn src_addr(&mut self) -> SrcAddrW<'_, DdmaSrcAdrReg0Spec> {
        SrcAddrW::new(self, 0)
    }
}
#[doc = "Dedicated DMA Channel 0 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_src_adr_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_src_adr_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdmaSrcAdrReg0Spec;
impl crate::RegisterSpec for DdmaSrcAdrReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddma_src_adr_reg0::R`](R) reader structure"]
impl crate::Readable for DdmaSrcAdrReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ddma_src_adr_reg0::W`](W) writer structure"]
impl crate::Writable for DdmaSrcAdrReg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DDMA_SRC_ADR_REG0 to value 0"]
impl crate::Resettable for DdmaSrcAdrReg0Spec {}
