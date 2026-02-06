#[doc = "Register `NMI_INT_CTRL` reader"]
pub type R = crate::R<NmiIntCtrlSpec>;
#[doc = "Register `NMI_INT_CTRL` writer"]
pub type W = crate::W<NmiIntCtrlSpec>;
#[doc = "Field `NMI_SRC_TYPE` reader - External NMI Interrupt Source Type: 00=Low level, 01=Negative edge, 10=High level, 11=Positive edge"]
pub type NmiSrcTypeR = crate::FieldReader;
#[doc = "Field `NMI_SRC_TYPE` writer - External NMI Interrupt Source Type: 00=Low level, 01=Negative edge, 10=High level, 11=Positive edge"]
pub type NmiSrcTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - External NMI Interrupt Source Type: 00=Low level, 01=Negative edge, 10=High level, 11=Positive edge"]
    #[inline(always)]
    pub fn nmi_src_type(&self) -> NmiSrcTypeR {
        NmiSrcTypeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External NMI Interrupt Source Type: 00=Low level, 01=Negative edge, 10=High level, 11=Positive edge"]
    #[inline(always)]
    pub fn nmi_src_type(&mut self) -> NmiSrcTypeW<'_, NmiIntCtrlSpec> {
        NmiSrcTypeW::new(self, 0)
    }
}
#[doc = "NMI Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmi_int_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmi_int_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmiIntCtrlSpec;
impl crate::RegisterSpec for NmiIntCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmi_int_ctrl::R`](R) reader structure"]
impl crate::Readable for NmiIntCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`nmi_int_ctrl::W`](W) writer structure"]
impl crate::Writable for NmiIntCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NMI_INT_CTRL to value 0"]
impl crate::Resettable for NmiIntCtrlSpec {}
