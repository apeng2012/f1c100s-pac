#[doc = "Register `NDMA_BYTE_CNT_REG3` reader"]
pub type R = crate::R<NdmaByteCntReg3Spec>;
#[doc = "Register `NDMA_BYTE_CNT_REG3` writer"]
pub type W = crate::W<NdmaByteCntReg3Spec>;
#[doc = "Field `BYTE_CNT` reader - Normal DMA Byte Counter"]
pub type ByteCntR = crate::FieldReader<u32>;
#[doc = "Field `BYTE_CNT` writer - Normal DMA Byte Counter"]
pub type ByteCntW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Normal DMA Byte Counter"]
    #[inline(always)]
    pub fn byte_cnt(&self) -> ByteCntR {
        ByteCntR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Normal DMA Byte Counter"]
    #[inline(always)]
    pub fn byte_cnt(&mut self) -> ByteCntW<'_, NdmaByteCntReg3Spec> {
        ByteCntW::new(self, 0)
    }
}
#[doc = "Normal DMA Channel 3 Byte Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_byte_cnt_reg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_byte_cnt_reg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NdmaByteCntReg3Spec;
impl crate::RegisterSpec for NdmaByteCntReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ndma_byte_cnt_reg3::R`](R) reader structure"]
impl crate::Readable for NdmaByteCntReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`ndma_byte_cnt_reg3::W`](W) writer structure"]
impl crate::Writable for NdmaByteCntReg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NDMA_BYTE_CNT_REG3 to value 0"]
impl crate::Resettable for NdmaByteCntReg3Spec {}
