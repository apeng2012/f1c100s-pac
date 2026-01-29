#[doc = "Register `PC_DRV1` reader"]
pub type R = crate::R<PcDrv1Spec>;
#[doc = "Register `PC_DRV1` writer"]
pub type W = crate::W<PcDrv1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PC Multi-Driving Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_drv1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_drv1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcDrv1Spec;
impl crate::RegisterSpec for PcDrv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc_drv1::R`](R) reader structure"]
impl crate::Readable for PcDrv1Spec {}
#[doc = "`write(|w| ..)` method takes [`pc_drv1::W`](W) writer structure"]
impl crate::Writable for PcDrv1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PC_DRV1 to value 0"]
impl crate::Resettable for PcDrv1Spec {}
