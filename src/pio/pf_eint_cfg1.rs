#[doc = "Register `PF_EINT_CFG1` reader"]
pub type R = crate::R<PfEintCfg1Spec>;
#[doc = "Register `PF_EINT_CFG1` writer"]
pub type W = crate::W<PfEintCfg1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PF External Interrupt Configure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_eint_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_eint_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfEintCfg1Spec;
impl crate::RegisterSpec for PfEintCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_eint_cfg1::R`](R) reader structure"]
impl crate::Readable for PfEintCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pf_eint_cfg1::W`](W) writer structure"]
impl crate::Writable for PfEintCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PF_EINT_CFG1 to value 0"]
impl crate::Resettable for PfEintCfg1Spec {}
