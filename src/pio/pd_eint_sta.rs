#[doc = "Register `PD_EINT_STA` reader"]
pub type R = crate::R<PdEintStaSpec>;
#[doc = "Register `PD_EINT_STA` writer"]
pub type W = crate::W<PdEintStaSpec>;
#[doc = "Field `EINT_STA` reader - External INT Pending (write 1 to clear)"]
pub type EintStaR = crate::FieldReader<u32>;
#[doc = "Field `EINT_STA` writer - External INT Pending (write 1 to clear)"]
pub type EintStaW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - External INT Pending (write 1 to clear)"]
    #[inline(always)]
    pub fn eint_sta(&self) -> EintStaR {
        EintStaR::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - External INT Pending (write 1 to clear)"]
    #[inline(always)]
    pub fn eint_sta(&mut self) -> EintStaW<'_, PdEintStaSpec> {
        EintStaW::new(self, 0)
    }
}
#[doc = "PD External Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_eint_sta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_eint_sta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdEintStaSpec;
impl crate::RegisterSpec for PdEintStaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_eint_sta::R`](R) reader structure"]
impl crate::Readable for PdEintStaSpec {}
#[doc = "`write(|w| ..)` method takes [`pd_eint_sta::W`](W) writer structure"]
impl crate::Writable for PdEintStaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PD_EINT_STA to value 0"]
impl crate::Resettable for PdEintStaSpec {}
