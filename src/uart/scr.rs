#[doc = "Register `SCR` reader"]
pub type R = crate::R<ScrSpec>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Field `SCR` reader - Scratch data"]
pub type ScrR = crate::FieldReader;
#[doc = "Field `SCR` writer - Scratch data"]
pub type ScrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Scratch data"]
    #[inline(always)]
    pub fn scr(&self) -> ScrR {
        ScrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Scratch data"]
    #[inline(always)]
    pub fn scr(&mut self) -> ScrW<'_, ScrSpec> {
        ScrW::new(self, 0)
    }
}
#[doc = "Scratch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for ScrSpec {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for ScrSpec {}
