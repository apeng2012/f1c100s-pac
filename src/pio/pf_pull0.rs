#[doc = "Register `PF_PULL0` reader"]
pub type R = crate::R<PfPull0Spec>;
#[doc = "Register `PF_PULL0` writer"]
pub type W = crate::W<PfPull0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PF Pull Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_pull0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_pull0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfPull0Spec;
impl crate::RegisterSpec for PfPull0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_pull0::R`](R) reader structure"]
impl crate::Readable for PfPull0Spec {}
#[doc = "`write(|w| ..)` method takes [`pf_pull0::W`](W) writer structure"]
impl crate::Writable for PfPull0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PF_PULL0 to value 0"]
impl crate::Resettable for PfPull0Spec {}
