#[doc = "Register `DRAM_GATING` reader"]
pub type R = crate::R<DramGatingSpec>;
#[doc = "Register `DRAM_GATING` writer"]
pub type W = crate::W<DramGatingSpec>;
#[doc = "Field `VE_DCLK_GATING` reader - Gating DRAM SCLK(1X) For VE"]
pub type VeDclkGatingR = crate::BitReader;
#[doc = "Field `VE_DCLK_GATING` writer - Gating DRAM SCLK(1X) For VE"]
pub type VeDclkGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_DCLK_GATING` reader - Gating DRAM SCLK(1X) For CSI"]
pub type CsiDclkGatingR = crate::BitReader;
#[doc = "Field `CSI_DCLK_GATING` writer - Gating DRAM SCLK(1X) For CSI"]
pub type CsiDclkGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEINTERLACE_DCLK_GATING` reader - Gating DRAM SCLK(1X) For DEINTERLACE"]
pub type DeinterlaceDclkGatingR = crate::BitReader;
#[doc = "Field `DEINTERLACE_DCLK_GATING` writer - Gating DRAM SCLK(1X) For DEINTERLACE"]
pub type DeinterlaceDclkGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TVD_DCLK_GATING` reader - Gating DRAM SCLK(1X) For TVD"]
pub type TvdDclkGatingR = crate::BitReader;
#[doc = "Field `TVD_DCLK_GATING` writer - Gating DRAM SCLK(1X) For TVD"]
pub type TvdDclkGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FE_DCLK_GATING` reader - Gating DRAM SCLK(1X) For FE"]
pub type FeDclkGatingR = crate::BitReader;
#[doc = "Field `FE_DCLK_GATING` writer - Gating DRAM SCLK(1X) For FE"]
pub type FeDclkGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BE_DCLK_GATING` reader - Gating DRAM SCLK(1X) For BE"]
pub type BeDclkGatingR = crate::BitReader;
#[doc = "Field `BE_DCLK_GATING` writer - Gating DRAM SCLK(1X) For BE"]
pub type BeDclkGatingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Gating DRAM SCLK(1X) For VE"]
    #[inline(always)]
    pub fn ve_dclk_gating(&self) -> VeDclkGatingR {
        VeDclkGatingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gating DRAM SCLK(1X) For CSI"]
    #[inline(always)]
    pub fn csi_dclk_gating(&self) -> CsiDclkGatingR {
        CsiDclkGatingR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gating DRAM SCLK(1X) For DEINTERLACE"]
    #[inline(always)]
    pub fn deinterlace_dclk_gating(&self) -> DeinterlaceDclkGatingR {
        DeinterlaceDclkGatingR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Gating DRAM SCLK(1X) For TVD"]
    #[inline(always)]
    pub fn tvd_dclk_gating(&self) -> TvdDclkGatingR {
        TvdDclkGatingR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 24 - Gating DRAM SCLK(1X) For FE"]
    #[inline(always)]
    pub fn fe_dclk_gating(&self) -> FeDclkGatingR {
        FeDclkGatingR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Gating DRAM SCLK(1X) For BE"]
    #[inline(always)]
    pub fn be_dclk_gating(&self) -> BeDclkGatingR {
        BeDclkGatingR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gating DRAM SCLK(1X) For VE"]
    #[inline(always)]
    pub fn ve_dclk_gating(&mut self) -> VeDclkGatingW<'_, DramGatingSpec> {
        VeDclkGatingW::new(self, 0)
    }
    #[doc = "Bit 1 - Gating DRAM SCLK(1X) For CSI"]
    #[inline(always)]
    pub fn csi_dclk_gating(&mut self) -> CsiDclkGatingW<'_, DramGatingSpec> {
        CsiDclkGatingW::new(self, 1)
    }
    #[doc = "Bit 2 - Gating DRAM SCLK(1X) For DEINTERLACE"]
    #[inline(always)]
    pub fn deinterlace_dclk_gating(&mut self) -> DeinterlaceDclkGatingW<'_, DramGatingSpec> {
        DeinterlaceDclkGatingW::new(self, 2)
    }
    #[doc = "Bit 3 - Gating DRAM SCLK(1X) For TVD"]
    #[inline(always)]
    pub fn tvd_dclk_gating(&mut self) -> TvdDclkGatingW<'_, DramGatingSpec> {
        TvdDclkGatingW::new(self, 3)
    }
    #[doc = "Bit 24 - Gating DRAM SCLK(1X) For FE"]
    #[inline(always)]
    pub fn fe_dclk_gating(&mut self) -> FeDclkGatingW<'_, DramGatingSpec> {
        FeDclkGatingW::new(self, 24)
    }
    #[doc = "Bit 26 - Gating DRAM SCLK(1X) For BE"]
    #[inline(always)]
    pub fn be_dclk_gating(&mut self) -> BeDclkGatingW<'_, DramGatingSpec> {
        BeDclkGatingW::new(self, 26)
    }
}
#[doc = "DRAM GATING Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dram_gating::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dram_gating::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DramGatingSpec;
impl crate::RegisterSpec for DramGatingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dram_gating::R`](R) reader structure"]
impl crate::Readable for DramGatingSpec {}
#[doc = "`write(|w| ..)` method takes [`dram_gating::W`](W) writer structure"]
impl crate::Writable for DramGatingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DRAM_GATING to value 0"]
impl crate::Resettable for DramGatingSpec {}
