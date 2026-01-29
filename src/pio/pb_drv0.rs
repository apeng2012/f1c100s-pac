#[doc = "Register `PB_DRV0` reader"]
pub type R = crate::R<PbDrv0Spec>;
#[doc = "Register `PB_DRV0` writer"]
pub type W = crate::W<PbDrv0Spec>;
#[doc = "Field `PB0_DRV` reader - PB0 Multi-Driving Select"]
pub type Pb0DrvR = crate::FieldReader;
#[doc = "Field `PB0_DRV` writer - PB0 Multi-Driving Select"]
pub type Pb0DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB1_DRV` reader - PB1 Multi-Driving Select"]
pub type Pb1DrvR = crate::FieldReader;
#[doc = "Field `PB1_DRV` writer - PB1 Multi-Driving Select"]
pub type Pb1DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB2_DRV` reader - PB2 Multi-Driving Select"]
pub type Pb2DrvR = crate::FieldReader;
#[doc = "Field `PB2_DRV` writer - PB2 Multi-Driving Select"]
pub type Pb2DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB3_DRV` reader - PB3 Multi-Driving Select"]
pub type Pb3DrvR = crate::FieldReader;
#[doc = "Field `PB3_DRV` writer - PB3 Multi-Driving Select"]
pub type Pb3DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PB0 Multi-Driving Select"]
    #[inline(always)]
    pub fn pb0_drv(&self) -> Pb0DrvR {
        Pb0DrvR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PB1 Multi-Driving Select"]
    #[inline(always)]
    pub fn pb1_drv(&self) -> Pb1DrvR {
        Pb1DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PB2 Multi-Driving Select"]
    #[inline(always)]
    pub fn pb2_drv(&self) -> Pb2DrvR {
        Pb2DrvR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PB3 Multi-Driving Select"]
    #[inline(always)]
    pub fn pb3_drv(&self) -> Pb3DrvR {
        Pb3DrvR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PB0 Multi-Driving Select"]
    #[inline(always)]
    pub fn pb0_drv(&mut self) -> Pb0DrvW<'_, PbDrv0Spec> {
        Pb0DrvW::new(self, 0)
    }
    #[doc = "Bits 2:3 - PB1 Multi-Driving Select"]
    #[inline(always)]
    pub fn pb1_drv(&mut self) -> Pb1DrvW<'_, PbDrv0Spec> {
        Pb1DrvW::new(self, 2)
    }
    #[doc = "Bits 4:5 - PB2 Multi-Driving Select"]
    #[inline(always)]
    pub fn pb2_drv(&mut self) -> Pb2DrvW<'_, PbDrv0Spec> {
        Pb2DrvW::new(self, 4)
    }
    #[doc = "Bits 6:7 - PB3 Multi-Driving Select"]
    #[inline(always)]
    pub fn pb3_drv(&mut self) -> Pb3DrvW<'_, PbDrv0Spec> {
        Pb3DrvW::new(self, 6)
    }
}
#[doc = "PB Multi-Driving Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_drv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_drv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbDrv0Spec;
impl crate::RegisterSpec for PbDrv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb_drv0::R`](R) reader structure"]
impl crate::Readable for PbDrv0Spec {}
#[doc = "`write(|w| ..)` method takes [`pb_drv0::W`](W) writer structure"]
impl crate::Writable for PbDrv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PB_DRV0 to value 0x55"]
impl crate::Resettable for PbDrv0Spec {
    const RESET_VALUE: u32 = 0x55;
}
