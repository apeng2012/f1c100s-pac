#[doc = "Register `BUS_CLK_GATING1` reader"]
pub type R = crate::R<BusClkGating1Spec>;
#[doc = "Register `BUS_CLK_GATING1` writer"]
pub type W = crate::W<BusClkGating1Spec>;
#[doc = "Field `VE_GATING` reader - Gating Clock For VE"]
pub type VeGatingR = crate::BitReader;
#[doc = "Field `VE_GATING` writer - Gating Clock For VE"]
pub type VeGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_GATING` reader - Gating Clock For LCD"]
pub type LcdGatingR = crate::BitReader;
#[doc = "Field `LCD_GATING` writer - Gating Clock For LCD"]
pub type LcdGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEINTERLACE_GATING` reader - Gating Clock For DE Interlacer"]
pub type DeinterlaceGatingR = crate::BitReader;
#[doc = "Field `DEINTERLACE_GATING` writer - Gating Clock For DE Interlacer"]
pub type DeinterlaceGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_GATING` reader - Gating Clock For CSI"]
pub type CsiGatingR = crate::BitReader;
#[doc = "Field `CSI_GATING` writer - Gating Clock For CSI"]
pub type CsiGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TVD_GATING` reader - Gating Clock For TVD"]
pub type TvdGatingR = crate::BitReader;
#[doc = "Field `TVD_GATING` writer - Gating Clock For TVD"]
pub type TvdGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TVE_GATING` reader - Gating Clock For TVE"]
pub type TveGatingR = crate::BitReader;
#[doc = "Field `TVE_GATING` writer - Gating Clock For TVE"]
pub type TveGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBE_GATING` reader - Gating Clock For DEBE"]
pub type DebeGatingR = crate::BitReader;
#[doc = "Field `DEBE_GATING` writer - Gating Clock For DEBE"]
pub type DebeGatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEFE_GATING` reader - Gating Clock For DEFE"]
pub type DefeGatingR = crate::BitReader;
#[doc = "Field `DEFE_GATING` writer - Gating Clock For DEFE"]
pub type DefeGatingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Gating Clock For VE"]
    #[inline(always)]
    pub fn ve_gating(&self) -> VeGatingR {
        VeGatingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Gating Clock For LCD"]
    #[inline(always)]
    pub fn lcd_gating(&self) -> LcdGatingR {
        LcdGatingR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Gating Clock For DE Interlacer"]
    #[inline(always)]
    pub fn deinterlace_gating(&self) -> DeinterlaceGatingR {
        DeinterlaceGatingR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Gating Clock For CSI"]
    #[inline(always)]
    pub fn csi_gating(&self) -> CsiGatingR {
        CsiGatingR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Gating Clock For TVD"]
    #[inline(always)]
    pub fn tvd_gating(&self) -> TvdGatingR {
        TvdGatingR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Gating Clock For TVE"]
    #[inline(always)]
    pub fn tve_gating(&self) -> TveGatingR {
        TveGatingR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Gating Clock For DEBE"]
    #[inline(always)]
    pub fn debe_gating(&self) -> DebeGatingR {
        DebeGatingR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Gating Clock For DEFE"]
    #[inline(always)]
    pub fn defe_gating(&self) -> DefeGatingR {
        DefeGatingR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gating Clock For VE"]
    #[inline(always)]
    pub fn ve_gating(&mut self) -> VeGatingW<'_, BusClkGating1Spec> {
        VeGatingW::new(self, 0)
    }
    #[doc = "Bit 4 - Gating Clock For LCD"]
    #[inline(always)]
    pub fn lcd_gating(&mut self) -> LcdGatingW<'_, BusClkGating1Spec> {
        LcdGatingW::new(self, 4)
    }
    #[doc = "Bit 5 - Gating Clock For DE Interlacer"]
    #[inline(always)]
    pub fn deinterlace_gating(&mut self) -> DeinterlaceGatingW<'_, BusClkGating1Spec> {
        DeinterlaceGatingW::new(self, 5)
    }
    #[doc = "Bit 8 - Gating Clock For CSI"]
    #[inline(always)]
    pub fn csi_gating(&mut self) -> CsiGatingW<'_, BusClkGating1Spec> {
        CsiGatingW::new(self, 8)
    }
    #[doc = "Bit 9 - Gating Clock For TVD"]
    #[inline(always)]
    pub fn tvd_gating(&mut self) -> TvdGatingW<'_, BusClkGating1Spec> {
        TvdGatingW::new(self, 9)
    }
    #[doc = "Bit 10 - Gating Clock For TVE"]
    #[inline(always)]
    pub fn tve_gating(&mut self) -> TveGatingW<'_, BusClkGating1Spec> {
        TveGatingW::new(self, 10)
    }
    #[doc = "Bit 12 - Gating Clock For DEBE"]
    #[inline(always)]
    pub fn debe_gating(&mut self) -> DebeGatingW<'_, BusClkGating1Spec> {
        DebeGatingW::new(self, 12)
    }
    #[doc = "Bit 14 - Gating Clock For DEFE"]
    #[inline(always)]
    pub fn defe_gating(&mut self) -> DefeGatingW<'_, BusClkGating1Spec> {
        DefeGatingW::new(self, 14)
    }
}
#[doc = "Bus Clock Gating Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_clk_gating1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_clk_gating1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusClkGating1Spec;
impl crate::RegisterSpec for BusClkGating1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_clk_gating1::R`](R) reader structure"]
impl crate::Readable for BusClkGating1Spec {}
#[doc = "`write(|w| ..)` method takes [`bus_clk_gating1::W`](W) writer structure"]
impl crate::Writable for BusClkGating1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUS_CLK_GATING1 to value 0"]
impl crate::Resettable for BusClkGating1Spec {}
