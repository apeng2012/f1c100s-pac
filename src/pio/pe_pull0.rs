#[doc = "Register `PE_PULL0` reader"]
pub type R = crate::R<PePull0Spec>;
#[doc = "Register `PE_PULL0` writer"]
pub type W = crate::W<PePull0Spec>;
#[doc = "Field `PE_PULL` reader - PE0-12 Pull Select"]
pub type PePullR = crate::FieldReader<u32>;
#[doc = "Field `PE_PULL` writer - PE0-12 Pull Select"]
pub type PePullW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - PE0-12 Pull Select"]
    #[inline(always)]
    pub fn pe_pull(&self) -> PePullR {
        PePullR::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - PE0-12 Pull Select"]
    #[inline(always)]
    pub fn pe_pull(&mut self) -> PePullW<'_, PePull0Spec> {
        PePullW::new(self, 0)
    }
}
#[doc = "PE Pull Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_pull0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_pull0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PePull0Spec;
impl crate::RegisterSpec for PePull0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_pull0::R`](R) reader structure"]
impl crate::Readable for PePull0Spec {}
#[doc = "`write(|w| ..)` method takes [`pe_pull0::W`](W) writer structure"]
impl crate::Writable for PePull0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PE_PULL0 to value 0"]
impl crate::Resettable for PePull0Spec {}
