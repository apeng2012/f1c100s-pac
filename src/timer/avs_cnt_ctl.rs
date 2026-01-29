#[doc = "Register `AVS_CNT_CTL` reader"]
pub type R = crate::R<AvsCntCtlSpec>;
#[doc = "Register `AVS_CNT_CTL` writer"]
pub type W = crate::W<AvsCntCtlSpec>;
#[doc = "Field `AVS_CNT0_EN` reader - AVS Counter 0 Enable"]
pub type AvsCnt0EnR = crate::BitReader;
#[doc = "Field `AVS_CNT0_EN` writer - AVS Counter 0 Enable"]
pub type AvsCnt0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVS_CNT1_EN` reader - AVS Counter 1 Enable"]
pub type AvsCnt1EnR = crate::BitReader;
#[doc = "Field `AVS_CNT1_EN` writer - AVS Counter 1 Enable"]
pub type AvsCnt1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVS_CNT0_PS` reader - AVS Counter 0 Pause"]
pub type AvsCnt0PsR = crate::BitReader;
#[doc = "Field `AVS_CNT0_PS` writer - AVS Counter 0 Pause"]
pub type AvsCnt0PsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVS_CNT1_PS` reader - AVS Counter 1 Pause"]
pub type AvsCnt1PsR = crate::BitReader;
#[doc = "Field `AVS_CNT1_PS` writer - AVS Counter 1 Pause"]
pub type AvsCnt1PsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AVS Counter 0 Enable"]
    #[inline(always)]
    pub fn avs_cnt0_en(&self) -> AvsCnt0EnR {
        AvsCnt0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AVS Counter 1 Enable"]
    #[inline(always)]
    pub fn avs_cnt1_en(&self) -> AvsCnt1EnR {
        AvsCnt1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - AVS Counter 0 Pause"]
    #[inline(always)]
    pub fn avs_cnt0_ps(&self) -> AvsCnt0PsR {
        AvsCnt0PsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AVS Counter 1 Pause"]
    #[inline(always)]
    pub fn avs_cnt1_ps(&self) -> AvsCnt1PsR {
        AvsCnt1PsR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AVS Counter 0 Enable"]
    #[inline(always)]
    pub fn avs_cnt0_en(&mut self) -> AvsCnt0EnW<'_, AvsCntCtlSpec> {
        AvsCnt0EnW::new(self, 0)
    }
    #[doc = "Bit 1 - AVS Counter 1 Enable"]
    #[inline(always)]
    pub fn avs_cnt1_en(&mut self) -> AvsCnt1EnW<'_, AvsCntCtlSpec> {
        AvsCnt1EnW::new(self, 1)
    }
    #[doc = "Bit 8 - AVS Counter 0 Pause"]
    #[inline(always)]
    pub fn avs_cnt0_ps(&mut self) -> AvsCnt0PsW<'_, AvsCntCtlSpec> {
        AvsCnt0PsW::new(self, 8)
    }
    #[doc = "Bit 9 - AVS Counter 1 Pause"]
    #[inline(always)]
    pub fn avs_cnt1_ps(&mut self) -> AvsCnt1PsW<'_, AvsCntCtlSpec> {
        AvsCnt1PsW::new(self, 9)
    }
}
#[doc = "AVS Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`avs_cnt_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`avs_cnt_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AvsCntCtlSpec;
impl crate::RegisterSpec for AvsCntCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`avs_cnt_ctl::R`](R) reader structure"]
impl crate::Readable for AvsCntCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`avs_cnt_ctl::W`](W) writer structure"]
impl crate::Writable for AvsCntCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AVS_CNT_CTL to value 0"]
impl crate::Resettable for AvsCntCtlSpec {}
