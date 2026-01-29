#[doc = "Register `PE_EINT_CFG3` reader"]
pub type R = crate::R<PeEintCfg3Spec>;
#[doc = "Register `PE_EINT_CFG3` writer"]
pub type W = crate::W<PeEintCfg3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PE External Interrupt Configure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_eint_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_eint_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeEintCfg3Spec;
impl crate::RegisterSpec for PeEintCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_eint_cfg3::R`](R) reader structure"]
impl crate::Readable for PeEintCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`pe_eint_cfg3::W`](W) writer structure"]
impl crate::Writable for PeEintCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PE_EINT_CFG3 to value 0"]
impl crate::Resettable for PeEintCfg3Spec {}
