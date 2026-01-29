#[doc = "Register `PB_CFG1` reader"]
pub type R = crate::R<PbCfg1Spec>;
#[doc = "Register `PB_CFG1` writer"]
pub type W = crate::W<PbCfg1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PB Configure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbCfg1Spec;
impl crate::RegisterSpec for PbCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb_cfg1::R`](R) reader structure"]
impl crate::Readable for PbCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pb_cfg1::W`](W) writer structure"]
impl crate::Writable for PbCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PB_CFG1 to value 0"]
impl crate::Resettable for PbCfg1Spec {}
