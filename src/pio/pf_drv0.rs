#[doc = "Register `PF_DRV0` reader"]
pub type R = crate::R<PfDrv0Spec>;
#[doc = "Register `PF_DRV0` writer"]
pub type W = crate::W<PfDrv0Spec>;
#[doc = "Field `PF0_DRV` reader - PF0 Multi-Driving Select"]
pub type Pf0DrvR = crate::FieldReader;
#[doc = "Field `PF0_DRV` writer - PF0 Multi-Driving Select"]
pub type Pf0DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PF1_DRV` reader - PF1 Multi-Driving Select"]
pub type Pf1DrvR = crate::FieldReader;
#[doc = "Field `PF1_DRV` writer - PF1 Multi-Driving Select"]
pub type Pf1DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PF2_DRV` reader - PF2 Multi-Driving Select"]
pub type Pf2DrvR = crate::FieldReader;
#[doc = "Field `PF2_DRV` writer - PF2 Multi-Driving Select"]
pub type Pf2DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PF3_DRV` reader - PF3 Multi-Driving Select"]
pub type Pf3DrvR = crate::FieldReader;
#[doc = "Field `PF3_DRV` writer - PF3 Multi-Driving Select"]
pub type Pf3DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PF4_DRV` reader - PF4 Multi-Driving Select"]
pub type Pf4DrvR = crate::FieldReader;
#[doc = "Field `PF4_DRV` writer - PF4 Multi-Driving Select"]
pub type Pf4DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PF5_DRV` reader - PF5 Multi-Driving Select"]
pub type Pf5DrvR = crate::FieldReader;
#[doc = "Field `PF5_DRV` writer - PF5 Multi-Driving Select"]
pub type Pf5DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PF0 Multi-Driving Select"]
    #[inline(always)]
    pub fn pf0_drv(&self) -> Pf0DrvR {
        Pf0DrvR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PF1 Multi-Driving Select"]
    #[inline(always)]
    pub fn pf1_drv(&self) -> Pf1DrvR {
        Pf1DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PF2 Multi-Driving Select"]
    #[inline(always)]
    pub fn pf2_drv(&self) -> Pf2DrvR {
        Pf2DrvR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PF3 Multi-Driving Select"]
    #[inline(always)]
    pub fn pf3_drv(&self) -> Pf3DrvR {
        Pf3DrvR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PF4 Multi-Driving Select"]
    #[inline(always)]
    pub fn pf4_drv(&self) -> Pf4DrvR {
        Pf4DrvR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PF5 Multi-Driving Select"]
    #[inline(always)]
    pub fn pf5_drv(&self) -> Pf5DrvR {
        Pf5DrvR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PF0 Multi-Driving Select"]
    #[inline(always)]
    pub fn pf0_drv(&mut self) -> Pf0DrvW<'_, PfDrv0Spec> {
        Pf0DrvW::new(self, 0)
    }
    #[doc = "Bits 2:3 - PF1 Multi-Driving Select"]
    #[inline(always)]
    pub fn pf1_drv(&mut self) -> Pf1DrvW<'_, PfDrv0Spec> {
        Pf1DrvW::new(self, 2)
    }
    #[doc = "Bits 4:5 - PF2 Multi-Driving Select"]
    #[inline(always)]
    pub fn pf2_drv(&mut self) -> Pf2DrvW<'_, PfDrv0Spec> {
        Pf2DrvW::new(self, 4)
    }
    #[doc = "Bits 6:7 - PF3 Multi-Driving Select"]
    #[inline(always)]
    pub fn pf3_drv(&mut self) -> Pf3DrvW<'_, PfDrv0Spec> {
        Pf3DrvW::new(self, 6)
    }
    #[doc = "Bits 8:9 - PF4 Multi-Driving Select"]
    #[inline(always)]
    pub fn pf4_drv(&mut self) -> Pf4DrvW<'_, PfDrv0Spec> {
        Pf4DrvW::new(self, 8)
    }
    #[doc = "Bits 10:11 - PF5 Multi-Driving Select"]
    #[inline(always)]
    pub fn pf5_drv(&mut self) -> Pf5DrvW<'_, PfDrv0Spec> {
        Pf5DrvW::new(self, 10)
    }
}
#[doc = "PF Multi-Driving Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_drv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_drv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfDrv0Spec;
impl crate::RegisterSpec for PfDrv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_drv0::R`](R) reader structure"]
impl crate::Readable for PfDrv0Spec {}
#[doc = "`write(|w| ..)` method takes [`pf_drv0::W`](W) writer structure"]
impl crate::Writable for PfDrv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PF_DRV0 to value 0x0555"]
impl crate::Resettable for PfDrv0Spec {
    const RESET_VALUE: u32 = 0x0555;
}
