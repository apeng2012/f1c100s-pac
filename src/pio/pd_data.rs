#[doc = "Register `PD_DATA` reader"]
pub type R = crate::R<PdDataSpec>;
#[doc = "Register `PD_DATA` writer"]
pub type W = crate::W<PdDataSpec>;
#[doc = "Field `PD_DAT` reader - PD Data"]
pub type PdDatR = crate::FieldReader<u32>;
#[doc = "Field `PD_DAT` writer - PD Data"]
pub type PdDatW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - PD Data"]
    #[inline(always)]
    pub fn pd_dat(&self) -> PdDatR {
        PdDatR::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - PD Data"]
    #[inline(always)]
    pub fn pd_dat(&mut self) -> PdDatW<'_, PdDataSpec> {
        PdDatW::new(self, 0)
    }
}
#[doc = "PD Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdDataSpec;
impl crate::RegisterSpec for PdDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_data::R`](R) reader structure"]
impl crate::Readable for PdDataSpec {}
#[doc = "`write(|w| ..)` method takes [`pd_data::W`](W) writer structure"]
impl crate::Writable for PdDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PD_DATA to value 0"]
impl crate::Resettable for PdDataSpec {}
