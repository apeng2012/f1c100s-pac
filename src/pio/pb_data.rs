#[doc = "Register `PB_DATA` reader"]
pub type R = crate::R<PbDataSpec>;
#[doc = "Register `PB_DATA` writer"]
pub type W = crate::W<PbDataSpec>;
#[doc = "Field `PB_DAT` reader - PB Data"]
pub type PbDatR = crate::FieldReader;
#[doc = "Field `PB_DAT` writer - PB Data"]
pub type PbDatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - PB Data"]
    #[inline(always)]
    pub fn pb_dat(&self) -> PbDatR {
        PbDatR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PB Data"]
    #[inline(always)]
    pub fn pb_dat(&mut self) -> PbDatW<'_, PbDataSpec> {
        PbDatW::new(self, 0)
    }
}
#[doc = "PB Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbDataSpec;
impl crate::RegisterSpec for PbDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb_data::R`](R) reader structure"]
impl crate::Readable for PbDataSpec {}
#[doc = "`write(|w| ..)` method takes [`pb_data::W`](W) writer structure"]
impl crate::Writable for PbDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PB_DATA to value 0"]
impl crate::Resettable for PbDataSpec {}
