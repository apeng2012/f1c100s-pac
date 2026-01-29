#[doc = "Register `PD_DRV1` reader"]
pub type R = crate::R<PdDrv1Spec>;
#[doc = "Register `PD_DRV1` writer"]
pub type W = crate::W<PdDrv1Spec>;
#[doc = "Field `PD_DRV` reader - PD16-21 Multi-Driving Select"]
pub type PdDrvR = crate::FieldReader<u16>;
#[doc = "Field `PD_DRV` writer - PD16-21 Multi-Driving Select"]
pub type PdDrvW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - PD16-21 Multi-Driving Select"]
    #[inline(always)]
    pub fn pd_drv(&self) -> PdDrvR {
        PdDrvR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - PD16-21 Multi-Driving Select"]
    #[inline(always)]
    pub fn pd_drv(&mut self) -> PdDrvW<'_, PdDrv1Spec> {
        PdDrvW::new(self, 0)
    }
}
#[doc = "PD Multi-Driving Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_drv1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_drv1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdDrv1Spec;
impl crate::RegisterSpec for PdDrv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_drv1::R`](R) reader structure"]
impl crate::Readable for PdDrv1Spec {}
#[doc = "`write(|w| ..)` method takes [`pd_drv1::W`](W) writer structure"]
impl crate::Writable for PdDrv1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PD_DRV1 to value 0x0555"]
impl crate::Resettable for PdDrv1Spec {
    const RESET_VALUE: u32 = 0x0555;
}
