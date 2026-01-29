#[doc = "Register `PC_CFG3` reader"]
pub type R = crate::R<PcCfg3Spec>;
#[doc = "Register `PC_CFG3` writer"]
pub type W = crate::W<PcCfg3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PC Configure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcCfg3Spec;
impl crate::RegisterSpec for PcCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc_cfg3::R`](R) reader structure"]
impl crate::Readable for PcCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`pc_cfg3::W`](W) writer structure"]
impl crate::Writable for PcCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PC_CFG3 to value 0"]
impl crate::Resettable for PcCfg3Spec {}
