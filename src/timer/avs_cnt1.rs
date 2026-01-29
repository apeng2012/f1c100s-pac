#[doc = "Register `AVS_CNT1` reader"]
pub type R = crate::R<AvsCnt1Spec>;
#[doc = "Register `AVS_CNT1` writer"]
pub type W = crate::W<AvsCnt1Spec>;
#[doc = "Field `AVS_CNT1` reader - AVS Counter 1 Value"]
pub type AvsCnt1R = crate::FieldReader<u32>;
#[doc = "Field `AVS_CNT1` writer - AVS Counter 1 Value"]
pub type AvsCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AVS Counter 1 Value"]
    #[inline(always)]
    pub fn avs_cnt1(&self) -> AvsCnt1R {
        AvsCnt1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AVS Counter 1 Value"]
    #[inline(always)]
    pub fn avs_cnt1(&mut self) -> AvsCnt1W<'_, AvsCnt1Spec> {
        AvsCnt1W::new(self, 0)
    }
}
#[doc = "AVS Counter 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`avs_cnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`avs_cnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AvsCnt1Spec;
impl crate::RegisterSpec for AvsCnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`avs_cnt1::R`](R) reader structure"]
impl crate::Readable for AvsCnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`avs_cnt1::W`](W) writer structure"]
impl crate::Writable for AvsCnt1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AVS_CNT1 to value 0"]
impl crate::Resettable for AvsCnt1Spec {}
