#[doc = "Register `SDR_PAD_PULL` reader"]
pub type R = crate::R<SdrPadPullSpec>;
#[doc = "Register `SDR_PAD_PULL` writer"]
pub type W = crate::W<SdrPadPullSpec>;
#[doc = "Field `DQ_PULL` reader - DQ Pull-up Select"]
pub type DqPullR = crate::FieldReader;
#[doc = "Field `DQ_PULL` writer - DQ Pull-up Select"]
pub type DqPullW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK_PULL` reader - CK, CK# Pull-up Select"]
pub type ClkPullR = crate::FieldReader;
#[doc = "Field `CLK_PULL` writer - CK, CK# Pull-up Select"]
pub type ClkPullW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DQM_PULL` reader - DQM Pull-up Select"]
pub type DqmPullR = crate::FieldReader;
#[doc = "Field `DQM_PULL` writer - DQM Pull-up Select"]
pub type DqmPullW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DQS_PULL` reader - DQS Pull-up Select"]
pub type DqsPullR = crate::FieldReader;
#[doc = "Field `DQS_PULL` writer - DQS Pull-up Select"]
pub type DqsPullW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDR_PAD_TYPE` reader - SDRAM Pad Type (0: LVCMOS, 1: SSTL-2)"]
pub type SdrPadTypeR = crate::BitReader;
#[doc = "Field `SDR_PAD_TYPE` writer - SDRAM Pad Type (0: LVCMOS, 1: SSTL-2)"]
pub type SdrPadTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREF_CFG` reader - Reference Configuration Factor"]
pub type VrefCfgR = crate::FieldReader;
#[doc = "Field `VREF_CFG` writer - Reference Configuration Factor"]
pub type VrefCfgW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `VREF_INTER_EN` reader - Internal Reference Enable"]
pub type VrefInterEnR = crate::BitReader;
#[doc = "Field `VREF_INTER_EN` writer - Internal Reference Enable"]
pub type VrefInterEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - DQ Pull-up Select"]
    #[inline(always)]
    pub fn dq_pull(&self) -> DqPullR {
        DqPullR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CK, CK# Pull-up Select"]
    #[inline(always)]
    pub fn clk_pull(&self) -> ClkPullR {
        ClkPullR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 6:7 - DQM Pull-up Select"]
    #[inline(always)]
    pub fn dqm_pull(&self) -> DqmPullR {
        DqmPullR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - DQS Pull-up Select"]
    #[inline(always)]
    pub fn dqs_pull(&self) -> DqsPullR {
        DqsPullR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - SDRAM Pad Type (0: LVCMOS, 1: SSTL-2)"]
    #[inline(always)]
    pub fn sdr_pad_type(&self) -> SdrPadTypeR {
        SdrPadTypeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - Reference Configuration Factor"]
    #[inline(always)]
    pub fn vref_cfg(&self) -> VrefCfgR {
        VrefCfgR::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Internal Reference Enable"]
    #[inline(always)]
    pub fn vref_inter_en(&self) -> VrefInterEnR {
        VrefInterEnR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DQ Pull-up Select"]
    #[inline(always)]
    pub fn dq_pull(&mut self) -> DqPullW<'_, SdrPadPullSpec> {
        DqPullW::new(self, 0)
    }
    #[doc = "Bits 2:3 - CK, CK# Pull-up Select"]
    #[inline(always)]
    pub fn clk_pull(&mut self) -> ClkPullW<'_, SdrPadPullSpec> {
        ClkPullW::new(self, 2)
    }
    #[doc = "Bits 6:7 - DQM Pull-up Select"]
    #[inline(always)]
    pub fn dqm_pull(&mut self) -> DqmPullW<'_, SdrPadPullSpec> {
        DqmPullW::new(self, 6)
    }
    #[doc = "Bits 8:9 - DQS Pull-up Select"]
    #[inline(always)]
    pub fn dqs_pull(&mut self) -> DqsPullW<'_, SdrPadPullSpec> {
        DqsPullW::new(self, 8)
    }
    #[doc = "Bit 16 - SDRAM Pad Type (0: LVCMOS, 1: SSTL-2)"]
    #[inline(always)]
    pub fn sdr_pad_type(&mut self) -> SdrPadTypeW<'_, SdrPadPullSpec> {
        SdrPadTypeW::new(self, 16)
    }
    #[doc = "Bits 17:22 - Reference Configuration Factor"]
    #[inline(always)]
    pub fn vref_cfg(&mut self) -> VrefCfgW<'_, SdrPadPullSpec> {
        VrefCfgW::new(self, 17)
    }
    #[doc = "Bit 23 - Internal Reference Enable"]
    #[inline(always)]
    pub fn vref_inter_en(&mut self) -> VrefInterEnW<'_, SdrPadPullSpec> {
        VrefInterEnW::new(self, 23)
    }
}
#[doc = "SDRAM Pad Pull Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdr_pad_pull::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdr_pad_pull::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdrPadPullSpec;
impl crate::RegisterSpec for SdrPadPullSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdr_pad_pull::R`](R) reader structure"]
impl crate::Readable for SdrPadPullSpec {}
#[doc = "`write(|w| ..)` method takes [`sdr_pad_pull::W`](W) writer structure"]
impl crate::Writable for SdrPadPullSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDR_PAD_PULL to value 0x0001_0105"]
impl crate::Resettable for SdrPadPullSpec {
    const RESET_VALUE: u32 = 0x0001_0105;
}
