#[doc = "Register `PD_CFG2` reader"]
pub type R = crate::R<PdCfg2Spec>;
#[doc = "Register `PD_CFG2` writer"]
pub type W = crate::W<PdCfg2Spec>;
#[doc = "Field `PD16_SELECT` reader - PD16 Select"]
pub type Pd16SelectR = crate::FieldReader;
#[doc = "Field `PD16_SELECT` writer - PD16 Select"]
pub type Pd16SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD17_SELECT` reader - PD17 Select"]
pub type Pd17SelectR = crate::FieldReader;
#[doc = "Field `PD17_SELECT` writer - PD17 Select"]
pub type Pd17SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD18_SELECT` reader - PD18 Select"]
pub type Pd18SelectR = crate::FieldReader;
#[doc = "Field `PD18_SELECT` writer - PD18 Select"]
pub type Pd18SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD19_SELECT` reader - PD19 Select"]
pub type Pd19SelectR = crate::FieldReader;
#[doc = "Field `PD19_SELECT` writer - PD19 Select"]
pub type Pd19SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD20_SELECT` reader - PD20 Select"]
pub type Pd20SelectR = crate::FieldReader;
#[doc = "Field `PD20_SELECT` writer - PD20 Select"]
pub type Pd20SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD21_SELECT` reader - PD21 Select"]
pub type Pd21SelectR = crate::FieldReader;
#[doc = "Field `PD21_SELECT` writer - PD21 Select"]
pub type Pd21SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - PD16 Select"]
    #[inline(always)]
    pub fn pd16_select(&self) -> Pd16SelectR {
        Pd16SelectR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - PD17 Select"]
    #[inline(always)]
    pub fn pd17_select(&self) -> Pd17SelectR {
        Pd17SelectR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - PD18 Select"]
    #[inline(always)]
    pub fn pd18_select(&self) -> Pd18SelectR {
        Pd18SelectR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - PD19 Select"]
    #[inline(always)]
    pub fn pd19_select(&self) -> Pd19SelectR {
        Pd19SelectR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - PD20 Select"]
    #[inline(always)]
    pub fn pd20_select(&self) -> Pd20SelectR {
        Pd20SelectR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - PD21 Select"]
    #[inline(always)]
    pub fn pd21_select(&self) -> Pd21SelectR {
        Pd21SelectR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PD16 Select"]
    #[inline(always)]
    pub fn pd16_select(&mut self) -> Pd16SelectW<'_, PdCfg2Spec> {
        Pd16SelectW::new(self, 0)
    }
    #[doc = "Bits 4:6 - PD17 Select"]
    #[inline(always)]
    pub fn pd17_select(&mut self) -> Pd17SelectW<'_, PdCfg2Spec> {
        Pd17SelectW::new(self, 4)
    }
    #[doc = "Bits 8:10 - PD18 Select"]
    #[inline(always)]
    pub fn pd18_select(&mut self) -> Pd18SelectW<'_, PdCfg2Spec> {
        Pd18SelectW::new(self, 8)
    }
    #[doc = "Bits 12:14 - PD19 Select"]
    #[inline(always)]
    pub fn pd19_select(&mut self) -> Pd19SelectW<'_, PdCfg2Spec> {
        Pd19SelectW::new(self, 12)
    }
    #[doc = "Bits 16:18 - PD20 Select"]
    #[inline(always)]
    pub fn pd20_select(&mut self) -> Pd20SelectW<'_, PdCfg2Spec> {
        Pd20SelectW::new(self, 16)
    }
    #[doc = "Bits 20:22 - PD21 Select"]
    #[inline(always)]
    pub fn pd21_select(&mut self) -> Pd21SelectW<'_, PdCfg2Spec> {
        Pd21SelectW::new(self, 20)
    }
}
#[doc = "PD Configure Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdCfg2Spec;
impl crate::RegisterSpec for PdCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_cfg2::R`](R) reader structure"]
impl crate::Readable for PdCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`pd_cfg2::W`](W) writer structure"]
impl crate::Writable for PdCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PD_CFG2 to value 0x0077_7777"]
impl crate::Resettable for PdCfg2Spec {
    const RESET_VALUE: u32 = 0x0077_7777;
}
