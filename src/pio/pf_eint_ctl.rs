#[doc = "Register `PF_EINT_CTL` reader"]
pub type R = crate::R<PfEintCtlSpec>;
#[doc = "Register `PF_EINT_CTL` writer"]
pub type W = crate::W<PfEintCtlSpec>;
#[doc = "Field `EINT_CTL` reader - External INT Enable (bit n for PF_EINTn)"]
pub type EintCtlR = crate::FieldReader;
#[doc = "Field `EINT_CTL` writer - External INT Enable (bit n for PF_EINTn)"]
pub type EintCtlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - External INT Enable (bit n for PF_EINTn)"]
    #[inline(always)]
    pub fn eint_ctl(&self) -> EintCtlR {
        EintCtlR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - External INT Enable (bit n for PF_EINTn)"]
    #[inline(always)]
    pub fn eint_ctl(&mut self) -> EintCtlW<'_, PfEintCtlSpec> {
        EintCtlW::new(self, 0)
    }
}
#[doc = "PF External Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_eint_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_eint_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfEintCtlSpec;
impl crate::RegisterSpec for PfEintCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_eint_ctl::R`](R) reader structure"]
impl crate::Readable for PfEintCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`pf_eint_ctl::W`](W) writer structure"]
impl crate::Writable for PfEintCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PF_EINT_CTL to value 0"]
impl crate::Resettable for PfEintCtlSpec {}
