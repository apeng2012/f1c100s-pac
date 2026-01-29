#[doc = "Register `PB_DRV1` reader"]
pub type R = crate::R<PbDrv1Spec>;
#[doc = "Register `PB_DRV1` writer"]
pub type W = crate::W<PbDrv1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PB Multi-Driving Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_drv1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_drv1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbDrv1Spec;
impl crate::RegisterSpec for PbDrv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb_drv1::R`](R) reader structure"]
impl crate::Readable for PbDrv1Spec {}
#[doc = "`write(|w| ..)` method takes [`pb_drv1::W`](W) writer structure"]
impl crate::Writable for PbDrv1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PB_DRV1 to value 0"]
impl crate::Resettable for PbDrv1Spec {}
