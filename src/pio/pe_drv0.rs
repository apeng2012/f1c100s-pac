#[doc = "Register `PE_DRV0` reader"]
pub type R = crate::R<PeDrv0Spec>;
#[doc = "Register `PE_DRV0` writer"]
pub type W = crate::W<PeDrv0Spec>;
#[doc = "Field `PE_DRV` reader - PE0-12 Multi-Driving Select"]
pub type PeDrvR = crate::FieldReader<u32>;
#[doc = "Field `PE_DRV` writer - PE0-12 Multi-Driving Select"]
pub type PeDrvW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - PE0-12 Multi-Driving Select"]
    #[inline(always)]
    pub fn pe_drv(&self) -> PeDrvR {
        PeDrvR::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - PE0-12 Multi-Driving Select"]
    #[inline(always)]
    pub fn pe_drv(&mut self) -> PeDrvW<'_, PeDrv0Spec> {
        PeDrvW::new(self, 0)
    }
}
#[doc = "PE Multi-Driving Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_drv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_drv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeDrv0Spec;
impl crate::RegisterSpec for PeDrv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_drv0::R`](R) reader structure"]
impl crate::Readable for PeDrv0Spec {}
#[doc = "`write(|w| ..)` method takes [`pe_drv0::W`](W) writer structure"]
impl crate::Writable for PeDrv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PE_DRV0 to value 0x0155_5555"]
impl crate::Resettable for PeDrv0Spec {
    const RESET_VALUE: u32 = 0x0155_5555;
}
