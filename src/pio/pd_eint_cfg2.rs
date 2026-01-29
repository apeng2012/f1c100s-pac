#[doc = "Register `PD_EINT_CFG2` reader"]
pub type R = crate::R<PdEintCfg2Spec>;
#[doc = "Register `PD_EINT_CFG2` writer"]
pub type W = crate::W<PdEintCfg2Spec>;
#[doc = "Field `EINT16_CFG` reader - External INT16 Mode"]
pub type Eint16CfgR = crate::FieldReader;
#[doc = "Field `EINT16_CFG` writer - External INT16 Mode"]
pub type Eint16CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EINT17_CFG` reader - External INT17 Mode"]
pub type Eint17CfgR = crate::FieldReader;
#[doc = "Field `EINT17_CFG` writer - External INT17 Mode"]
pub type Eint17CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EINT18_CFG` reader - External INT18 Mode"]
pub type Eint18CfgR = crate::FieldReader;
#[doc = "Field `EINT18_CFG` writer - External INT18 Mode"]
pub type Eint18CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EINT19_CFG` reader - External INT19 Mode"]
pub type Eint19CfgR = crate::FieldReader;
#[doc = "Field `EINT19_CFG` writer - External INT19 Mode"]
pub type Eint19CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EINT20_CFG` reader - External INT20 Mode"]
pub type Eint20CfgR = crate::FieldReader;
#[doc = "Field `EINT20_CFG` writer - External INT20 Mode"]
pub type Eint20CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EINT21_CFG` reader - External INT21 Mode"]
pub type Eint21CfgR = crate::FieldReader;
#[doc = "Field `EINT21_CFG` writer - External INT21 Mode"]
pub type Eint21CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - External INT16 Mode"]
    #[inline(always)]
    pub fn eint16_cfg(&self) -> Eint16CfgR {
        Eint16CfgR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External INT17 Mode"]
    #[inline(always)]
    pub fn eint17_cfg(&self) -> Eint17CfgR {
        Eint17CfgR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External INT18 Mode"]
    #[inline(always)]
    pub fn eint18_cfg(&self) -> Eint18CfgR {
        Eint18CfgR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - External INT19 Mode"]
    #[inline(always)]
    pub fn eint19_cfg(&self) -> Eint19CfgR {
        Eint19CfgR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - External INT20 Mode"]
    #[inline(always)]
    pub fn eint20_cfg(&self) -> Eint20CfgR {
        Eint20CfgR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - External INT21 Mode"]
    #[inline(always)]
    pub fn eint21_cfg(&self) -> Eint21CfgR {
        Eint21CfgR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - External INT16 Mode"]
    #[inline(always)]
    pub fn eint16_cfg(&mut self) -> Eint16CfgW<'_, PdEintCfg2Spec> {
        Eint16CfgW::new(self, 0)
    }
    #[doc = "Bits 4:7 - External INT17 Mode"]
    #[inline(always)]
    pub fn eint17_cfg(&mut self) -> Eint17CfgW<'_, PdEintCfg2Spec> {
        Eint17CfgW::new(self, 4)
    }
    #[doc = "Bits 8:11 - External INT18 Mode"]
    #[inline(always)]
    pub fn eint18_cfg(&mut self) -> Eint18CfgW<'_, PdEintCfg2Spec> {
        Eint18CfgW::new(self, 8)
    }
    #[doc = "Bits 12:15 - External INT19 Mode"]
    #[inline(always)]
    pub fn eint19_cfg(&mut self) -> Eint19CfgW<'_, PdEintCfg2Spec> {
        Eint19CfgW::new(self, 12)
    }
    #[doc = "Bits 16:19 - External INT20 Mode"]
    #[inline(always)]
    pub fn eint20_cfg(&mut self) -> Eint20CfgW<'_, PdEintCfg2Spec> {
        Eint20CfgW::new(self, 16)
    }
    #[doc = "Bits 20:23 - External INT21 Mode"]
    #[inline(always)]
    pub fn eint21_cfg(&mut self) -> Eint21CfgW<'_, PdEintCfg2Spec> {
        Eint21CfgW::new(self, 20)
    }
}
#[doc = "PD External Interrupt Configure Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_eint_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_eint_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdEintCfg2Spec;
impl crate::RegisterSpec for PdEintCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_eint_cfg2::R`](R) reader structure"]
impl crate::Readable for PdEintCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`pd_eint_cfg2::W`](W) writer structure"]
impl crate::Writable for PdEintCfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PD_EINT_CFG2 to value 0"]
impl crate::Resettable for PdEintCfg2Spec {}
