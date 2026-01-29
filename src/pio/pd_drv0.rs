#[doc = "Register `PD_DRV0` reader"]
pub type R = crate::R<PdDrv0Spec>;
#[doc = "Register `PD_DRV0` writer"]
pub type W = crate::W<PdDrv0Spec>;
#[doc = "Field `PD_DRV` reader - PD0-15 Multi-Driving Select"]
pub type PdDrvR = crate::FieldReader<u32>;
#[doc = "Field `PD_DRV` writer - PD0-15 Multi-Driving Select"]
pub type PdDrvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PD0-15 Multi-Driving Select"]
    #[inline(always)]
    pub fn pd_drv(&self) -> PdDrvR {
        PdDrvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PD0-15 Multi-Driving Select"]
    #[inline(always)]
    pub fn pd_drv(&mut self) -> PdDrvW<'_, PdDrv0Spec> {
        PdDrvW::new(self, 0)
    }
}
#[doc = "PD Multi-Driving Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_drv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_drv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdDrv0Spec;
impl crate::RegisterSpec for PdDrv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_drv0::R`](R) reader structure"]
impl crate::Readable for PdDrv0Spec {}
#[doc = "`write(|w| ..)` method takes [`pd_drv0::W`](W) writer structure"]
impl crate::Writable for PdDrv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PD_DRV0 to value 0x5555_5555"]
impl crate::Resettable for PdDrv0Spec {
    const RESET_VALUE: u32 = 0x5555_5555;
}
