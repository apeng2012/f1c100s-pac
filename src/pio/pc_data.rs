#[doc = "Register `PC_DATA` reader"]
pub type R = crate::R<PcDataSpec>;
#[doc = "Register `PC_DATA` writer"]
pub type W = crate::W<PcDataSpec>;
#[doc = "Field `PC_DAT` reader - PC Data"]
pub type PcDatR = crate::FieldReader;
#[doc = "Field `PC_DAT` writer - PC Data"]
pub type PcDatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - PC Data"]
    #[inline(always)]
    pub fn pc_dat(&self) -> PcDatR {
        PcDatR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PC Data"]
    #[inline(always)]
    pub fn pc_dat(&mut self) -> PcDatW<'_, PcDataSpec> {
        PcDatW::new(self, 0)
    }
}
#[doc = "PC Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcDataSpec;
impl crate::RegisterSpec for PcDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc_data::R`](R) reader structure"]
impl crate::Readable for PcDataSpec {}
#[doc = "`write(|w| ..)` method takes [`pc_data::W`](W) writer structure"]
impl crate::Writable for PcDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PC_DATA to value 0"]
impl crate::Resettable for PcDataSpec {}
