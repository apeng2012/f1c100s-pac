#[doc = "Register `PA_DRV0` reader"]
pub type R = crate::R<PaDrv0Spec>;
#[doc = "Register `PA_DRV0` writer"]
pub type W = crate::W<PaDrv0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PA Multi-Driving Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_drv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_drv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaDrv0Spec;
impl crate::RegisterSpec for PaDrv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_drv0::R`](R) reader structure"]
impl crate::Readable for PaDrv0Spec {}
#[doc = "`write(|w| ..)` method takes [`pa_drv0::W`](W) writer structure"]
impl crate::Writable for PaDrv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PA_DRV0 to value 0"]
impl crate::Resettable for PaDrv0Spec {}
