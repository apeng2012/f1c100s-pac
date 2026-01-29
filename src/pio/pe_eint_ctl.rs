#[doc = "Register `PE_EINT_CTL` reader"]
pub type R = crate::R<PeEintCtlSpec>;
#[doc = "Register `PE_EINT_CTL` writer"]
pub type W = crate::W<PeEintCtlSpec>;
#[doc = "Field `EINT_CTL` reader - External INT Enable (bit n for PE_EINTn)"]
pub type EintCtlR = crate::FieldReader<u16>;
#[doc = "Field `EINT_CTL` writer - External INT Enable (bit n for PE_EINTn)"]
pub type EintCtlW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - External INT Enable (bit n for PE_EINTn)"]
    #[inline(always)]
    pub fn eint_ctl(&self) -> EintCtlR {
        EintCtlR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - External INT Enable (bit n for PE_EINTn)"]
    #[inline(always)]
    pub fn eint_ctl(&mut self) -> EintCtlW<'_, PeEintCtlSpec> {
        EintCtlW::new(self, 0)
    }
}
#[doc = "PE External Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_eint_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_eint_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeEintCtlSpec;
impl crate::RegisterSpec for PeEintCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_eint_ctl::R`](R) reader structure"]
impl crate::Readable for PeEintCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`pe_eint_ctl::W`](W) writer structure"]
impl crate::Writable for PeEintCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PE_EINT_CTL to value 0"]
impl crate::Resettable for PeEintCtlSpec {}
