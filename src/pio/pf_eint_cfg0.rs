#[doc = "Register `PF_EINT_CFG0` reader"]
pub type R = crate::R<PfEintCfg0Spec>;
#[doc = "Register `PF_EINT_CFG0` writer"]
pub type W = crate::W<PfEintCfg0Spec>;
#[doc = "Field `EINT0_CFG` reader - External INT0 Mode"]
pub type Eint0CfgR = crate::FieldReader;
#[doc = "Field `EINT0_CFG` writer - External INT0 Mode"]
pub type Eint0CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EINT1_CFG` reader - External INT1 Mode"]
pub type Eint1CfgR = crate::FieldReader;
#[doc = "Field `EINT1_CFG` writer - External INT1 Mode"]
pub type Eint1CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EINT2_CFG` reader - External INT2 Mode"]
pub type Eint2CfgR = crate::FieldReader;
#[doc = "Field `EINT2_CFG` writer - External INT2 Mode"]
pub type Eint2CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EINT3_CFG` reader - External INT3 Mode"]
pub type Eint3CfgR = crate::FieldReader;
#[doc = "Field `EINT3_CFG` writer - External INT3 Mode"]
pub type Eint3CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EINT4_CFG` reader - External INT4 Mode"]
pub type Eint4CfgR = crate::FieldReader;
#[doc = "Field `EINT4_CFG` writer - External INT4 Mode"]
pub type Eint4CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EINT5_CFG` reader - External INT5 Mode"]
pub type Eint5CfgR = crate::FieldReader;
#[doc = "Field `EINT5_CFG` writer - External INT5 Mode"]
pub type Eint5CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - External INT0 Mode"]
    #[inline(always)]
    pub fn eint0_cfg(&self) -> Eint0CfgR {
        Eint0CfgR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External INT1 Mode"]
    #[inline(always)]
    pub fn eint1_cfg(&self) -> Eint1CfgR {
        Eint1CfgR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External INT2 Mode"]
    #[inline(always)]
    pub fn eint2_cfg(&self) -> Eint2CfgR {
        Eint2CfgR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - External INT3 Mode"]
    #[inline(always)]
    pub fn eint3_cfg(&self) -> Eint3CfgR {
        Eint3CfgR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - External INT4 Mode"]
    #[inline(always)]
    pub fn eint4_cfg(&self) -> Eint4CfgR {
        Eint4CfgR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - External INT5 Mode"]
    #[inline(always)]
    pub fn eint5_cfg(&self) -> Eint5CfgR {
        Eint5CfgR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - External INT0 Mode"]
    #[inline(always)]
    pub fn eint0_cfg(&mut self) -> Eint0CfgW<'_, PfEintCfg0Spec> {
        Eint0CfgW::new(self, 0)
    }
    #[doc = "Bits 4:7 - External INT1 Mode"]
    #[inline(always)]
    pub fn eint1_cfg(&mut self) -> Eint1CfgW<'_, PfEintCfg0Spec> {
        Eint1CfgW::new(self, 4)
    }
    #[doc = "Bits 8:11 - External INT2 Mode"]
    #[inline(always)]
    pub fn eint2_cfg(&mut self) -> Eint2CfgW<'_, PfEintCfg0Spec> {
        Eint2CfgW::new(self, 8)
    }
    #[doc = "Bits 12:15 - External INT3 Mode"]
    #[inline(always)]
    pub fn eint3_cfg(&mut self) -> Eint3CfgW<'_, PfEintCfg0Spec> {
        Eint3CfgW::new(self, 12)
    }
    #[doc = "Bits 16:19 - External INT4 Mode"]
    #[inline(always)]
    pub fn eint4_cfg(&mut self) -> Eint4CfgW<'_, PfEintCfg0Spec> {
        Eint4CfgW::new(self, 16)
    }
    #[doc = "Bits 20:23 - External INT5 Mode"]
    #[inline(always)]
    pub fn eint5_cfg(&mut self) -> Eint5CfgW<'_, PfEintCfg0Spec> {
        Eint5CfgW::new(self, 20)
    }
}
#[doc = "PF External Interrupt Configure Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_eint_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_eint_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfEintCfg0Spec;
impl crate::RegisterSpec for PfEintCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_eint_cfg0::R`](R) reader structure"]
impl crate::Readable for PfEintCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pf_eint_cfg0::W`](W) writer structure"]
impl crate::Writable for PfEintCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PF_EINT_CFG0 to value 0"]
impl crate::Resettable for PfEintCfg0Spec {}
