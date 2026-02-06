#[doc = "Register `DDMA_PAR_REG3` reader"]
pub type R = crate::R<DdmaParReg3Spec>;
#[doc = "Register `DDMA_PAR_REG3` writer"]
pub type W = crate::W<DdmaParReg3Spec>;
#[doc = "Field `SRC_COMITY_CNT` reader - Source Comity Counter (value = N+1)"]
pub type SrcComityCntR = crate::FieldReader;
#[doc = "Field `SRC_COMITY_CNT` writer - Source Comity Counter (value = N+1)"]
pub type SrcComityCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRC_BLOCK_CNT` reader - Source Block Counter (value = N+1)"]
pub type SrcBlockCntR = crate::FieldReader;
#[doc = "Field `SRC_BLOCK_CNT` writer - Source Block Counter (value = N+1)"]
pub type SrcBlockCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DST_COMITY_CNT` reader - Destination Comity Counter (value = N+1)"]
pub type DstComityCntR = crate::FieldReader;
#[doc = "Field `DST_COMITY_CNT` writer - Destination Comity Counter (value = N+1)"]
pub type DstComityCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DST_BLOCK_CNT` reader - Destination Block Counter (value = N+1)"]
pub type DstBlockCntR = crate::FieldReader;
#[doc = "Field `DST_BLOCK_CNT` writer - Destination Block Counter (value = N+1)"]
pub type DstBlockCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Source Comity Counter (value = N+1)"]
    #[inline(always)]
    pub fn src_comity_cnt(&self) -> SrcComityCntR {
        SrcComityCntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Source Block Counter (value = N+1)"]
    #[inline(always)]
    pub fn src_block_cnt(&self) -> SrcBlockCntR {
        SrcBlockCntR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Destination Comity Counter (value = N+1)"]
    #[inline(always)]
    pub fn dst_comity_cnt(&self) -> DstComityCntR {
        DstComityCntR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Destination Block Counter (value = N+1)"]
    #[inline(always)]
    pub fn dst_block_cnt(&self) -> DstBlockCntR {
        DstBlockCntR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Source Comity Counter (value = N+1)"]
    #[inline(always)]
    pub fn src_comity_cnt(&mut self) -> SrcComityCntW<'_, DdmaParReg3Spec> {
        SrcComityCntW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Source Block Counter (value = N+1)"]
    #[inline(always)]
    pub fn src_block_cnt(&mut self) -> SrcBlockCntW<'_, DdmaParReg3Spec> {
        SrcBlockCntW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Destination Comity Counter (value = N+1)"]
    #[inline(always)]
    pub fn dst_comity_cnt(&mut self) -> DstComityCntW<'_, DdmaParReg3Spec> {
        DstComityCntW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Destination Block Counter (value = N+1)"]
    #[inline(always)]
    pub fn dst_block_cnt(&mut self) -> DstBlockCntW<'_, DdmaParReg3Spec> {
        DstBlockCntW::new(self, 24)
    }
}
#[doc = "Dedicated DMA Channel 3 Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_par_reg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_par_reg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdmaParReg3Spec;
impl crate::RegisterSpec for DdmaParReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddma_par_reg3::R`](R) reader structure"]
impl crate::Readable for DdmaParReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`ddma_par_reg3::W`](W) writer structure"]
impl crate::Writable for DdmaParReg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DDMA_PAR_REG3 to value 0"]
impl crate::Resettable for DdmaParReg3Spec {}
