#[doc = "Register `PF_PULL1` reader"]
pub type R = crate::R<PfPull1Spec>;
#[doc = "Register `PF_PULL1` writer"]
pub type W = crate::W<PfPull1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PF Pull Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_pull1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_pull1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfPull1Spec;
impl crate::RegisterSpec for PfPull1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_pull1::R`](R) reader structure"]
impl crate::Readable for PfPull1Spec {}
#[doc = "`write(|w| ..)` method takes [`pf_pull1::W`](W) writer structure"]
impl crate::Writable for PfPull1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PF_PULL1 to value 0"]
impl crate::Resettable for PfPull1Spec {}
