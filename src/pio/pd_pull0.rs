#[doc = "Register `PD_PULL0` reader"]
pub type R = crate::R<PdPull0Spec>;
#[doc = "Register `PD_PULL0` writer"]
pub type W = crate::W<PdPull0Spec>;
#[doc = "Field `PD_PULL` reader - PD0-15 Pull Select"]
pub type PdPullR = crate::FieldReader<u32>;
#[doc = "Field `PD_PULL` writer - PD0-15 Pull Select"]
pub type PdPullW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PD0-15 Pull Select"]
    #[inline(always)]
    pub fn pd_pull(&self) -> PdPullR {
        PdPullR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PD0-15 Pull Select"]
    #[inline(always)]
    pub fn pd_pull(&mut self) -> PdPullW<'_, PdPull0Spec> {
        PdPullW::new(self, 0)
    }
}
#[doc = "PD Pull Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_pull0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_pull0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdPull0Spec;
impl crate::RegisterSpec for PdPull0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_pull0::R`](R) reader structure"]
impl crate::Readable for PdPull0Spec {}
#[doc = "`write(|w| ..)` method takes [`pd_pull0::W`](W) writer structure"]
impl crate::Writable for PdPull0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PD_PULL0 to value 0"]
impl crate::Resettable for PdPull0Spec {}
