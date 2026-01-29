#[doc = "Register `PE_DATA` reader"]
pub type R = crate::R<PeDataSpec>;
#[doc = "Register `PE_DATA` writer"]
pub type W = crate::W<PeDataSpec>;
#[doc = "Field `PE_DAT` reader - PE Data"]
pub type PeDatR = crate::FieldReader<u16>;
#[doc = "Field `PE_DAT` writer - PE Data"]
pub type PeDatW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - PE Data"]
    #[inline(always)]
    pub fn pe_dat(&self) -> PeDatR {
        PeDatR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - PE Data"]
    #[inline(always)]
    pub fn pe_dat(&mut self) -> PeDatW<'_, PeDataSpec> {
        PeDatW::new(self, 0)
    }
}
#[doc = "PE Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeDataSpec;
impl crate::RegisterSpec for PeDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_data::R`](R) reader structure"]
impl crate::Readable for PeDataSpec {}
#[doc = "`write(|w| ..)` method takes [`pe_data::W`](W) writer structure"]
impl crate::Writable for PeDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PE_DATA to value 0"]
impl crate::Resettable for PeDataSpec {}
