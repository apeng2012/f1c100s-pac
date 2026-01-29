#[doc = "Register `BUS_SOFT_RST1` reader"]
pub type R = crate::R<BusSoftRst1Spec>;
#[doc = "Register `BUS_SOFT_RST1` writer"]
pub type W = crate::W<BusSoftRst1Spec>;
#[doc = "Field `VE_RST` reader - VE Reset: 0=Assert, 1=De-assert"]
pub type VeRstR = crate::BitReader;
#[doc = "Field `VE_RST` writer - VE Reset: 0=Assert, 1=De-assert"]
pub type VeRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_RST` reader - LCD Reset: 0=Assert, 1=De-assert"]
pub type LcdRstR = crate::BitReader;
#[doc = "Field `LCD_RST` writer - LCD Reset: 0=Assert, 1=De-assert"]
pub type LcdRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEINTERLACE_RST` reader - DEINTERLACE Reset: 0=Assert, 1=De-assert"]
pub type DeinterlaceRstR = crate::BitReader;
#[doc = "Field `DEINTERLACE_RST` writer - DEINTERLACE Reset: 0=Assert, 1=De-assert"]
pub type DeinterlaceRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSI_RST` reader - CSI Reset: 0=Assert, 1=De-assert"]
pub type CsiRstR = crate::BitReader;
#[doc = "Field `CSI_RST` writer - CSI Reset: 0=Assert, 1=De-assert"]
pub type CsiRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TVD_RST` reader - TVD Reset: 0=Assert, 1=De-assert"]
pub type TvdRstR = crate::BitReader;
#[doc = "Field `TVD_RST` writer - TVD Reset: 0=Assert, 1=De-assert"]
pub type TvdRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TVE_RST` reader - TVE Reset: 0=Assert, 1=De-assert"]
pub type TveRstR = crate::BitReader;
#[doc = "Field `TVE_RST` writer - TVE Reset: 0=Assert, 1=De-assert"]
pub type TveRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBE_RST` reader - DEBE Reset: 0=Assert, 1=De-assert"]
pub type DebeRstR = crate::BitReader;
#[doc = "Field `DEBE_RST` writer - DEBE Reset: 0=Assert, 1=De-assert"]
pub type DebeRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEFE_RST` reader - DEFE Reset: 0=Assert, 1=De-assert"]
pub type DefeRstR = crate::BitReader;
#[doc = "Field `DEFE_RST` writer - DEFE Reset: 0=Assert, 1=De-assert"]
pub type DefeRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VE Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn ve_rst(&self) -> VeRstR {
        VeRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - LCD Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn lcd_rst(&self) -> LcdRstR {
        LcdRstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DEINTERLACE Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn deinterlace_rst(&self) -> DeinterlaceRstR {
        DeinterlaceRstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CSI Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn csi_rst(&self) -> CsiRstR {
        CsiRstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TVD Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn tvd_rst(&self) -> TvdRstR {
        TvdRstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TVE Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn tve_rst(&self) -> TveRstR {
        TveRstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - DEBE Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn debe_rst(&self) -> DebeRstR {
        DebeRstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - DEFE Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn defe_rst(&self) -> DefeRstR {
        DefeRstR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VE Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn ve_rst(&mut self) -> VeRstW<'_, BusSoftRst1Spec> {
        VeRstW::new(self, 0)
    }
    #[doc = "Bit 4 - LCD Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn lcd_rst(&mut self) -> LcdRstW<'_, BusSoftRst1Spec> {
        LcdRstW::new(self, 4)
    }
    #[doc = "Bit 5 - DEINTERLACE Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn deinterlace_rst(&mut self) -> DeinterlaceRstW<'_, BusSoftRst1Spec> {
        DeinterlaceRstW::new(self, 5)
    }
    #[doc = "Bit 8 - CSI Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn csi_rst(&mut self) -> CsiRstW<'_, BusSoftRst1Spec> {
        CsiRstW::new(self, 8)
    }
    #[doc = "Bit 9 - TVD Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn tvd_rst(&mut self) -> TvdRstW<'_, BusSoftRst1Spec> {
        TvdRstW::new(self, 9)
    }
    #[doc = "Bit 10 - TVE Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn tve_rst(&mut self) -> TveRstW<'_, BusSoftRst1Spec> {
        TveRstW::new(self, 10)
    }
    #[doc = "Bit 12 - DEBE Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn debe_rst(&mut self) -> DebeRstW<'_, BusSoftRst1Spec> {
        DebeRstW::new(self, 12)
    }
    #[doc = "Bit 14 - DEFE Reset: 0=Assert, 1=De-assert"]
    #[inline(always)]
    pub fn defe_rst(&mut self) -> DefeRstW<'_, BusSoftRst1Spec> {
        DefeRstW::new(self, 14)
    }
}
#[doc = "Bus Software Reset Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_soft_rst1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_soft_rst1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusSoftRst1Spec;
impl crate::RegisterSpec for BusSoftRst1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_soft_rst1::R`](R) reader structure"]
impl crate::Readable for BusSoftRst1Spec {}
#[doc = "`write(|w| ..)` method takes [`bus_soft_rst1::W`](W) writer structure"]
impl crate::Writable for BusSoftRst1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUS_SOFT_RST1 to value 0"]
impl crate::Resettable for BusSoftRst1Spec {}
