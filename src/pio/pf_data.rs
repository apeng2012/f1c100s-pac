#[doc = "Register `PF_DATA` reader"]
pub type R = crate::R<PfDataSpec>;
#[doc = "Register `PF_DATA` writer"]
pub type W = crate::W<PfDataSpec>;
#[doc = "Field `PF_DAT` reader - PF Data"]
pub type PfDatR = crate::FieldReader;
#[doc = "Field `PF_DAT` writer - PF Data"]
pub type PfDatW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - PF Data"]
    #[inline(always)]
    pub fn pf_dat(&self) -> PfDatR {
        PfDatR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PF Data"]
    #[inline(always)]
    pub fn pf_dat(&mut self) -> PfDatW<'_, PfDataSpec> {
        PfDatW::new(self, 0)
    }
}
#[doc = "PF Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfDataSpec;
impl crate::RegisterSpec for PfDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_data::R`](R) reader structure"]
impl crate::Readable for PfDataSpec {}
#[doc = "`write(|w| ..)` method takes [`pf_data::W`](W) writer structure"]
impl crate::Writable for PfDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PF_DATA to value 0"]
impl crate::Resettable for PfDataSpec {}
