#[doc = "Register `DDMA_GEN_DATA` reader"]
pub type R = crate::R<DdmaGenDataSpec>;
#[doc = "Register `DDMA_GEN_DATA` writer"]
pub type W = crate::W<DdmaGenDataSpec>;
#[doc = "Field `GEN_DATA` reader - Dedicated DMA general data (used as source in memory set mode)"]
pub type GenDataR = crate::FieldReader<u32>;
#[doc = "Field `GEN_DATA` writer - Dedicated DMA general data (used as source in memory set mode)"]
pub type GenDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Dedicated DMA general data (used as source in memory set mode)"]
    #[inline(always)]
    pub fn gen_data(&self) -> GenDataR {
        GenDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Dedicated DMA general data (used as source in memory set mode)"]
    #[inline(always)]
    pub fn gen_data(&mut self) -> GenDataW<'_, DdmaGenDataSpec> {
        GenDataW::new(self, 0)
    }
}
#[doc = "Dedicated DMA General Data Register (only valid for DDMA3, used with memory set mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_gen_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_gen_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdmaGenDataSpec;
impl crate::RegisterSpec for DdmaGenDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddma_gen_data::R`](R) reader structure"]
impl crate::Readable for DdmaGenDataSpec {}
#[doc = "`write(|w| ..)` method takes [`ddma_gen_data::W`](W) writer structure"]
impl crate::Writable for DdmaGenDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DDMA_GEN_DATA to value 0"]
impl crate::Resettable for DdmaGenDataSpec {}
