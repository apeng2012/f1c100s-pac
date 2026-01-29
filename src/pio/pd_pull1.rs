#[doc = "Register `PD_PULL1` reader"]
pub type R = crate::R<PdPull1Spec>;
#[doc = "Register `PD_PULL1` writer"]
pub type W = crate::W<PdPull1Spec>;
#[doc = "Field `PD_PULL` reader - PD16-21 Pull Select"]
pub type PdPullR = crate::FieldReader<u16>;
#[doc = "Field `PD_PULL` writer - PD16-21 Pull Select"]
pub type PdPullW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - PD16-21 Pull Select"]
    #[inline(always)]
    pub fn pd_pull(&self) -> PdPullR {
        PdPullR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - PD16-21 Pull Select"]
    #[inline(always)]
    pub fn pd_pull(&mut self) -> PdPullW<'_, PdPull1Spec> {
        PdPullW::new(self, 0)
    }
}
#[doc = "PD Pull Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_pull1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_pull1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdPull1Spec;
impl crate::RegisterSpec for PdPull1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_pull1::R`](R) reader structure"]
impl crate::Readable for PdPull1Spec {}
#[doc = "`write(|w| ..)` method takes [`pd_pull1::W`](W) writer structure"]
impl crate::Writable for PdPull1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PD_PULL1 to value 0"]
impl crate::Resettable for PdPull1Spec {}
