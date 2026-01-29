#[doc = "Register `AVS_CNT_DIV` reader"]
pub type R = crate::R<AvsCntDivSpec>;
#[doc = "Register `AVS_CNT_DIV` writer"]
pub type W = crate::W<AvsCntDivSpec>;
#[doc = "Field `AVS_CNT0_D` reader - Divisor N for AVS Counter 0"]
pub type AvsCnt0DR = crate::FieldReader<u16>;
#[doc = "Field `AVS_CNT0_D` writer - Divisor N for AVS Counter 0"]
pub type AvsCnt0DW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AVS_CNT1_D` reader - Divisor N for AVS Counter 1"]
pub type AvsCnt1DR = crate::FieldReader<u16>;
#[doc = "Field `AVS_CNT1_D` writer - Divisor N for AVS Counter 1"]
pub type AvsCnt1DW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Divisor N for AVS Counter 0"]
    #[inline(always)]
    pub fn avs_cnt0_d(&self) -> AvsCnt0DR {
        AvsCnt0DR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Divisor N for AVS Counter 1"]
    #[inline(always)]
    pub fn avs_cnt1_d(&self) -> AvsCnt1DR {
        AvsCnt1DR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Divisor N for AVS Counter 0"]
    #[inline(always)]
    pub fn avs_cnt0_d(&mut self) -> AvsCnt0DW<'_, AvsCntDivSpec> {
        AvsCnt0DW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Divisor N for AVS Counter 1"]
    #[inline(always)]
    pub fn avs_cnt1_d(&mut self) -> AvsCnt1DW<'_, AvsCntDivSpec> {
        AvsCnt1DW::new(self, 16)
    }
}
#[doc = "AVS Counter Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`avs_cnt_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`avs_cnt_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AvsCntDivSpec;
impl crate::RegisterSpec for AvsCntDivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`avs_cnt_div::R`](R) reader structure"]
impl crate::Readable for AvsCntDivSpec {}
#[doc = "`write(|w| ..)` method takes [`avs_cnt_div::W`](W) writer structure"]
impl crate::Writable for AvsCntDivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AVS_CNT_DIV to value 0x05db_05db"]
impl crate::Resettable for AvsCntDivSpec {
    const RESET_VALUE: u32 = 0x05db_05db;
}
