#[doc = "Register `PC_DRV0` reader"]
pub type R = crate::R<PcDrv0Spec>;
#[doc = "Register `PC_DRV0` writer"]
pub type W = crate::W<PcDrv0Spec>;
#[doc = "Field `PC0_DRV` reader - PC0 Multi-Driving Select"]
pub type Pc0DrvR = crate::FieldReader;
#[doc = "Field `PC0_DRV` writer - PC0 Multi-Driving Select"]
pub type Pc0DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC1_DRV` reader - PC1 Multi-Driving Select"]
pub type Pc1DrvR = crate::FieldReader;
#[doc = "Field `PC1_DRV` writer - PC1 Multi-Driving Select"]
pub type Pc1DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC2_DRV` reader - PC2 Multi-Driving Select"]
pub type Pc2DrvR = crate::FieldReader;
#[doc = "Field `PC2_DRV` writer - PC2 Multi-Driving Select"]
pub type Pc2DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC3_DRV` reader - PC3 Multi-Driving Select"]
pub type Pc3DrvR = crate::FieldReader;
#[doc = "Field `PC3_DRV` writer - PC3 Multi-Driving Select"]
pub type Pc3DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PC0 Multi-Driving Select"]
    #[inline(always)]
    pub fn pc0_drv(&self) -> Pc0DrvR {
        Pc0DrvR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PC1 Multi-Driving Select"]
    #[inline(always)]
    pub fn pc1_drv(&self) -> Pc1DrvR {
        Pc1DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PC2 Multi-Driving Select"]
    #[inline(always)]
    pub fn pc2_drv(&self) -> Pc2DrvR {
        Pc2DrvR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PC3 Multi-Driving Select"]
    #[inline(always)]
    pub fn pc3_drv(&self) -> Pc3DrvR {
        Pc3DrvR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PC0 Multi-Driving Select"]
    #[inline(always)]
    pub fn pc0_drv(&mut self) -> Pc0DrvW<'_, PcDrv0Spec> {
        Pc0DrvW::new(self, 0)
    }
    #[doc = "Bits 2:3 - PC1 Multi-Driving Select"]
    #[inline(always)]
    pub fn pc1_drv(&mut self) -> Pc1DrvW<'_, PcDrv0Spec> {
        Pc1DrvW::new(self, 2)
    }
    #[doc = "Bits 4:5 - PC2 Multi-Driving Select"]
    #[inline(always)]
    pub fn pc2_drv(&mut self) -> Pc2DrvW<'_, PcDrv0Spec> {
        Pc2DrvW::new(self, 4)
    }
    #[doc = "Bits 6:7 - PC3 Multi-Driving Select"]
    #[inline(always)]
    pub fn pc3_drv(&mut self) -> Pc3DrvW<'_, PcDrv0Spec> {
        Pc3DrvW::new(self, 6)
    }
}
#[doc = "PC Multi-Driving Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_drv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_drv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcDrv0Spec;
impl crate::RegisterSpec for PcDrv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc_drv0::R`](R) reader structure"]
impl crate::Readable for PcDrv0Spec {}
#[doc = "`write(|w| ..)` method takes [`pc_drv0::W`](W) writer structure"]
impl crate::Writable for PcDrv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PC_DRV0 to value 0x55"]
impl crate::Resettable for PcDrv0Spec {
    const RESET_VALUE: u32 = 0x55;
}
