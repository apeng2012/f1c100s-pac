#[doc = "Register `PF_EINT_STA` reader"]
pub type R = crate::R<PfEintStaSpec>;
#[doc = "Register `PF_EINT_STA` writer"]
pub type W = crate::W<PfEintStaSpec>;
#[doc = "Field `EINT_STA` reader - External INT Pending (write 1 to clear)"]
pub type EintStaR = crate::FieldReader;
#[doc = "Field `EINT_STA` writer - External INT Pending (write 1 to clear)"]
pub type EintStaW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - External INT Pending (write 1 to clear)"]
    #[inline(always)]
    pub fn eint_sta(&self) -> EintStaR {
        EintStaR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - External INT Pending (write 1 to clear)"]
    #[inline(always)]
    pub fn eint_sta(&mut self) -> EintStaW<'_, PfEintStaSpec> {
        EintStaW::new(self, 0)
    }
}
#[doc = "PF External Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_eint_sta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_eint_sta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfEintStaSpec;
impl crate::RegisterSpec for PfEintStaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_eint_sta::R`](R) reader structure"]
impl crate::Readable for PfEintStaSpec {}
#[doc = "`write(|w| ..)` method takes [`pf_eint_sta::W`](W) writer structure"]
impl crate::Writable for PfEintStaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PF_EINT_STA to value 0"]
impl crate::Resettable for PfEintStaSpec {}
