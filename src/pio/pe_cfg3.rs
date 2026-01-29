#[doc = "Register `PE_CFG3` reader"]
pub type R = crate::R<PeCfg3Spec>;
#[doc = "Register `PE_CFG3` writer"]
pub type W = crate::W<PeCfg3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PE Configure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeCfg3Spec;
impl crate::RegisterSpec for PeCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_cfg3::R`](R) reader structure"]
impl crate::Readable for PeCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`pe_cfg3::W`](W) writer structure"]
impl crate::Writable for PeCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PE_CFG3 to value 0"]
impl crate::Resettable for PeCfg3Spec {}
