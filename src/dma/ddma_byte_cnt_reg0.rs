#[doc = "Register `DDMA_BYTE_CNT_REG0` reader"]
pub type R = crate::R<DdmaByteCntReg0Spec>;
#[doc = "Register `DDMA_BYTE_CNT_REG0` writer"]
pub type W = crate::W<DdmaByteCntReg0Spec>;
#[doc = "Field `BYTE_CNT` reader - Dedicated DMA Byte Counter. 0=no transfer. Max 0x1000000."]
pub type ByteCntR = crate::FieldReader<u32>;
#[doc = "Field `BYTE_CNT` writer - Dedicated DMA Byte Counter. 0=no transfer. Max 0x1000000."]
pub type ByteCntW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Dedicated DMA Byte Counter. 0=no transfer. Max 0x1000000."]
    #[inline(always)]
    pub fn byte_cnt(&self) -> ByteCntR {
        ByteCntR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - Dedicated DMA Byte Counter. 0=no transfer. Max 0x1000000."]
    #[inline(always)]
    pub fn byte_cnt(&mut self) -> ByteCntW<'_, DdmaByteCntReg0Spec> {
        ByteCntW::new(self, 0)
    }
}
#[doc = "Dedicated DMA Channel 0 Byte Counter Register (max 0x1000000)\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_byte_cnt_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_byte_cnt_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdmaByteCntReg0Spec;
impl crate::RegisterSpec for DdmaByteCntReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddma_byte_cnt_reg0::R`](R) reader structure"]
impl crate::Readable for DdmaByteCntReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ddma_byte_cnt_reg0::W`](W) writer structure"]
impl crate::Writable for DdmaByteCntReg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DDMA_BYTE_CNT_REG0 to value 0"]
impl crate::Resettable for DdmaByteCntReg0Spec {}
