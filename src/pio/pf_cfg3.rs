#[doc = "Register `PF_CFG3` reader"]
pub type R = crate::R<PfCfg3Spec>;
#[doc = "Register `PF_CFG3` writer"]
pub type W = crate::W<PfCfg3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PF Configure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfCfg3Spec;
impl crate::RegisterSpec for PfCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_cfg3::R`](R) reader structure"]
impl crate::Readable for PfCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`pf_cfg3::W`](W) writer structure"]
impl crate::Writable for PfCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PF_CFG3 to value 0"]
impl crate::Resettable for PfCfg3Spec {}
