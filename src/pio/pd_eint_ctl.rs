#[doc = "Register `PD_EINT_CTL` reader"]
pub type R = crate::R<PdEintCtlSpec>;
#[doc = "Register `PD_EINT_CTL` writer"]
pub type W = crate::W<PdEintCtlSpec>;
#[doc = "Field `EINT_CTL` reader - External INT Enable (bit n for PD_EINTn)"]
pub type EintCtlR = crate::FieldReader<u32>;
#[doc = "Field `EINT_CTL` writer - External INT Enable (bit n for PD_EINTn)"]
pub type EintCtlW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - External INT Enable (bit n for PD_EINTn)"]
    #[inline(always)]
    pub fn eint_ctl(&self) -> EintCtlR {
        EintCtlR::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - External INT Enable (bit n for PD_EINTn)"]
    #[inline(always)]
    pub fn eint_ctl(&mut self) -> EintCtlW<'_, PdEintCtlSpec> {
        EintCtlW::new(self, 0)
    }
}
#[doc = "PD External Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_eint_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_eint_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdEintCtlSpec;
impl crate::RegisterSpec for PdEintCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_eint_ctl::R`](R) reader structure"]
impl crate::Readable for PdEintCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`pd_eint_ctl::W`](W) writer structure"]
impl crate::Writable for PdEintCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PD_EINT_CTL to value 0"]
impl crate::Resettable for PdEintCtlSpec {}
