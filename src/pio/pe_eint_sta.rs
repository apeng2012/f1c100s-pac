#[doc = "Register `PE_EINT_STA` reader"]
pub type R = crate::R<PeEintStaSpec>;
#[doc = "Register `PE_EINT_STA` writer"]
pub type W = crate::W<PeEintStaSpec>;
#[doc = "Field `EINT_STA` reader - External INT Pending (write 1 to clear)"]
pub type EintStaR = crate::FieldReader<u16>;
#[doc = "Field `EINT_STA` writer - External INT Pending (write 1 to clear)"]
pub type EintStaW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - External INT Pending (write 1 to clear)"]
    #[inline(always)]
    pub fn eint_sta(&self) -> EintStaR {
        EintStaR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - External INT Pending (write 1 to clear)"]
    #[inline(always)]
    pub fn eint_sta(&mut self) -> EintStaW<'_, PeEintStaSpec> {
        EintStaW::new(self, 0)
    }
}
#[doc = "PE External Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_eint_sta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_eint_sta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeEintStaSpec;
impl crate::RegisterSpec for PeEintStaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_eint_sta::R`](R) reader structure"]
impl crate::Readable for PeEintStaSpec {}
#[doc = "`write(|w| ..)` method takes [`pe_eint_sta::W`](W) writer structure"]
impl crate::Writable for PeEintStaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PE_EINT_STA to value 0"]
impl crate::Resettable for PeEintStaSpec {}
