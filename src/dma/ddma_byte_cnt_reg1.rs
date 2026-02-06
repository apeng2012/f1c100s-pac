#[doc = "Register `DDMA_BYTE_CNT_REG1` reader"]
pub type R = crate::R<DdmaByteCntReg1Spec>;
#[doc = "Register `DDMA_BYTE_CNT_REG1` writer"]
pub type W = crate::W<DdmaByteCntReg1Spec>;
#[doc = "Field `BYTE_CNT` reader - Dedicated DMA Byte Counter"]
pub type ByteCntR = crate::FieldReader<u32>;
#[doc = "Field `BYTE_CNT` writer - Dedicated DMA Byte Counter"]
pub type ByteCntW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Dedicated DMA Byte Counter"]
    #[inline(always)]
    pub fn byte_cnt(&self) -> ByteCntR {
        ByteCntR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - Dedicated DMA Byte Counter"]
    #[inline(always)]
    pub fn byte_cnt(&mut self) -> ByteCntW<'_, DdmaByteCntReg1Spec> {
        ByteCntW::new(self, 0)
    }
}
#[doc = "Dedicated DMA Channel 1 Byte Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_byte_cnt_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_byte_cnt_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdmaByteCntReg1Spec;
impl crate::RegisterSpec for DdmaByteCntReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddma_byte_cnt_reg1::R`](R) reader structure"]
impl crate::Readable for DdmaByteCntReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`ddma_byte_cnt_reg1::W`](W) writer structure"]
impl crate::Writable for DdmaByteCntReg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DDMA_BYTE_CNT_REG1 to value 0"]
impl crate::Resettable for DdmaByteCntReg1Spec {}
