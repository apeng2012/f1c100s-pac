#[doc = "Register `PA_DATA` reader"]
pub type R = crate::R<PaDataSpec>;
#[doc = "Register `PA_DATA` writer"]
pub type W = crate::W<PaDataSpec>;
#[doc = "Field `PA_DAT` reader - PA Data"]
pub type PaDatR = crate::FieldReader;
#[doc = "Field `PA_DAT` writer - PA Data"]
pub type PaDatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - PA Data"]
    #[inline(always)]
    pub fn pa_dat(&self) -> PaDatR {
        PaDatR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PA Data"]
    #[inline(always)]
    pub fn pa_dat(&mut self) -> PaDatW<'_, PaDataSpec> {
        PaDatW::new(self, 0)
    }
}
#[doc = "PA Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaDataSpec;
impl crate::RegisterSpec for PaDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_data::R`](R) reader structure"]
impl crate::Readable for PaDataSpec {}
#[doc = "`write(|w| ..)` method takes [`pa_data::W`](W) writer structure"]
impl crate::Writable for PaDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PA_DATA to value 0"]
impl crate::Resettable for PaDataSpec {}
