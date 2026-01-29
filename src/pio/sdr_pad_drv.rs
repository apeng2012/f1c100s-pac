#[doc = "Register `SDR_PAD_DRV` reader"]
pub type R = crate::R<SdrPadDrvSpec>;
#[doc = "Register `SDR_PAD_DRV` writer"]
pub type W = crate::W<SdrPadDrvSpec>;
#[doc = "Field `DQ_DRV` reader - DQ Multi-Driving Select"]
pub type DqDrvR = crate::FieldReader;
#[doc = "Field `DQ_DRV` writer - DQ Multi-Driving Select"]
pub type DqDrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK_DRV` reader - CK, CK# Multi-Driving Select"]
pub type ClkDrvR = crate::FieldReader;
#[doc = "Field `CLK_DRV` writer - CK, CK# Multi-Driving Select"]
pub type ClkDrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADDR_DRV` reader - DA, BA Multi-Driving Select"]
pub type AddrDrvR = crate::FieldReader;
#[doc = "Field `ADDR_DRV` writer - DA, BA Multi-Driving Select"]
pub type AddrDrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DQM_DRV` reader - DQM Multi-Driving Select"]
pub type DqmDrvR = crate::FieldReader;
#[doc = "Field `DQM_DRV` writer - DQM Multi-Driving Select"]
pub type DqmDrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DQS_DRV` reader - DQS Multi-Driving Select"]
pub type DqsDrvR = crate::FieldReader;
#[doc = "Field `DQS_DRV` writer - DQS Multi-Driving Select"]
pub type DqsDrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RAS_CAS_DRV` reader - RAS#, CAS#, SWE#, SCS#, CKE Multi-Driving Select"]
pub type RasCasDrvR = crate::FieldReader;
#[doc = "Field `RAS_CAS_DRV` writer - RAS#, CAS#, SWE#, SCS#, CKE Multi-Driving Select"]
pub type RasCasDrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODT_DRV` reader - ODT Multi-Driving Select"]
pub type OdtDrvR = crate::FieldReader;
#[doc = "Field `ODT_DRV` writer - ODT Multi-Driving Select"]
pub type OdtDrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - DQ Multi-Driving Select"]
    #[inline(always)]
    pub fn dq_drv(&self) -> DqDrvR {
        DqDrvR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CK, CK# Multi-Driving Select"]
    #[inline(always)]
    pub fn clk_drv(&self) -> ClkDrvR {
        ClkDrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - DA, BA Multi-Driving Select"]
    #[inline(always)]
    pub fn addr_drv(&self) -> AddrDrvR {
        AddrDrvR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - DQM Multi-Driving Select"]
    #[inline(always)]
    pub fn dqm_drv(&self) -> DqmDrvR {
        DqmDrvR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - DQS Multi-Driving Select"]
    #[inline(always)]
    pub fn dqs_drv(&self) -> DqsDrvR {
        DqsDrvR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - RAS#, CAS#, SWE#, SCS#, CKE Multi-Driving Select"]
    #[inline(always)]
    pub fn ras_cas_drv(&self) -> RasCasDrvR {
        RasCasDrvR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - ODT Multi-Driving Select"]
    #[inline(always)]
    pub fn odt_drv(&self) -> OdtDrvR {
        OdtDrvR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DQ Multi-Driving Select"]
    #[inline(always)]
    pub fn dq_drv(&mut self) -> DqDrvW<'_, SdrPadDrvSpec> {
        DqDrvW::new(self, 0)
    }
    #[doc = "Bits 2:3 - CK, CK# Multi-Driving Select"]
    #[inline(always)]
    pub fn clk_drv(&mut self) -> ClkDrvW<'_, SdrPadDrvSpec> {
        ClkDrvW::new(self, 2)
    }
    #[doc = "Bits 4:5 - DA, BA Multi-Driving Select"]
    #[inline(always)]
    pub fn addr_drv(&mut self) -> AddrDrvW<'_, SdrPadDrvSpec> {
        AddrDrvW::new(self, 4)
    }
    #[doc = "Bits 6:7 - DQM Multi-Driving Select"]
    #[inline(always)]
    pub fn dqm_drv(&mut self) -> DqmDrvW<'_, SdrPadDrvSpec> {
        DqmDrvW::new(self, 6)
    }
    #[doc = "Bits 8:9 - DQS Multi-Driving Select"]
    #[inline(always)]
    pub fn dqs_drv(&mut self) -> DqsDrvW<'_, SdrPadDrvSpec> {
        DqsDrvW::new(self, 8)
    }
    #[doc = "Bits 10:11 - RAS#, CAS#, SWE#, SCS#, CKE Multi-Driving Select"]
    #[inline(always)]
    pub fn ras_cas_drv(&mut self) -> RasCasDrvW<'_, SdrPadDrvSpec> {
        RasCasDrvW::new(self, 10)
    }
    #[doc = "Bits 12:13 - ODT Multi-Driving Select"]
    #[inline(always)]
    pub fn odt_drv(&mut self) -> OdtDrvW<'_, SdrPadDrvSpec> {
        OdtDrvW::new(self, 12)
    }
}
#[doc = "SDRAM Pad Multi-Driving Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdr_pad_drv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdr_pad_drv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdrPadDrvSpec;
impl crate::RegisterSpec for SdrPadDrvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdr_pad_drv::R`](R) reader structure"]
impl crate::Readable for SdrPadDrvSpec {}
#[doc = "`write(|w| ..)` method takes [`sdr_pad_drv::W`](W) writer structure"]
impl crate::Writable for SdrPadDrvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDR_PAD_DRV to value 0x1555"]
impl crate::Resettable for SdrPadDrvSpec {
    const RESET_VALUE: u32 = 0x1555;
}
