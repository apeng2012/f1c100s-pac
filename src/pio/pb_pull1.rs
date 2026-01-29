#[doc = "Register `PB_PULL1` reader"]
pub type R = crate::R<PbPull1Spec>;
#[doc = "Register `PB_PULL1` writer"]
pub type W = crate::W<PbPull1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PB Pull Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_pull1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_pull1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbPull1Spec;
impl crate::RegisterSpec for PbPull1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb_pull1::R`](R) reader structure"]
impl crate::Readable for PbPull1Spec {}
#[doc = "`write(|w| ..)` method takes [`pb_pull1::W`](W) writer structure"]
impl crate::Writable for PbPull1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PB_PULL1 to value 0"]
impl crate::Resettable for PbPull1Spec {}
