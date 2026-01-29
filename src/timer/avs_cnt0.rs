#[doc = "Register `AVS_CNT0` reader"]
pub type R = crate::R<AvsCnt0Spec>;
#[doc = "Register `AVS_CNT0` writer"]
pub type W = crate::W<AvsCnt0Spec>;
#[doc = "Field `AVS_CNT0` reader - AVS Counter 0 Value"]
pub type AvsCnt0R = crate::FieldReader<u32>;
#[doc = "Field `AVS_CNT0` writer - AVS Counter 0 Value"]
pub type AvsCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AVS Counter 0 Value"]
    #[inline(always)]
    pub fn avs_cnt0(&self) -> AvsCnt0R {
        AvsCnt0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AVS Counter 0 Value"]
    #[inline(always)]
    pub fn avs_cnt0(&mut self) -> AvsCnt0W<'_, AvsCnt0Spec> {
        AvsCnt0W::new(self, 0)
    }
}
#[doc = "AVS Counter 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`avs_cnt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`avs_cnt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AvsCnt0Spec;
impl crate::RegisterSpec for AvsCnt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`avs_cnt0::R`](R) reader structure"]
impl crate::Readable for AvsCnt0Spec {}
#[doc = "`write(|w| ..)` method takes [`avs_cnt0::W`](W) writer structure"]
impl crate::Writable for AvsCnt0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AVS_CNT0 to value 0"]
impl crate::Resettable for AvsCnt0Spec {}
