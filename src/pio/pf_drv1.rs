#[doc = "Register `PF_DRV1` reader"]
pub type R = crate::R<PfDrv1Spec>;
#[doc = "Register `PF_DRV1` writer"]
pub type W = crate::W<PfDrv1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PF Multi-Driving Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_drv1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_drv1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfDrv1Spec;
impl crate::RegisterSpec for PfDrv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_drv1::R`](R) reader structure"]
impl crate::Readable for PfDrv1Spec {}
#[doc = "`write(|w| ..)` method takes [`pf_drv1::W`](W) writer structure"]
impl crate::Writable for PfDrv1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PF_DRV1 to value 0"]
impl crate::Resettable for PfDrv1Spec {}
