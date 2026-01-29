#[doc = "Register `PD_EINT_CFG1` reader"]
pub type R = crate::R<PdEintCfg1Spec>;
#[doc = "Register `PD_EINT_CFG1` writer"]
pub type W = crate::W<PdEintCfg1Spec>;
#[doc = "Field `EINT8_CFG` reader - External INT8 Mode"]
pub type Eint8CfgR = crate::FieldReader;
#[doc = "Field `EINT8_CFG` writer - External INT8 Mode"]
pub type Eint8CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EINT9_CFG` reader - External INT9 Mode"]
pub type Eint9CfgR = crate::FieldReader;
#[doc = "Field `EINT9_CFG` writer - External INT9 Mode"]
pub type Eint9CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EINT10_CFG` reader - External INT10 Mode"]
pub type Eint10CfgR = crate::FieldReader;
#[doc = "Field `EINT10_CFG` writer - External INT10 Mode"]
pub type Eint10CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EINT11_CFG` reader - External INT11 Mode"]
pub type Eint11CfgR = crate::FieldReader;
#[doc = "Field `EINT11_CFG` writer - External INT11 Mode"]
pub type Eint11CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EINT12_CFG` reader - External INT12 Mode"]
pub type Eint12CfgR = crate::FieldReader;
#[doc = "Field `EINT12_CFG` writer - External INT12 Mode"]
pub type Eint12CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EINT13_CFG` reader - External INT13 Mode"]
pub type Eint13CfgR = crate::FieldReader;
#[doc = "Field `EINT13_CFG` writer - External INT13 Mode"]
pub type Eint13CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EINT14_CFG` reader - External INT14 Mode"]
pub type Eint14CfgR = crate::FieldReader;
#[doc = "Field `EINT14_CFG` writer - External INT14 Mode"]
pub type Eint14CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EINT15_CFG` reader - External INT15 Mode"]
pub type Eint15CfgR = crate::FieldReader;
#[doc = "Field `EINT15_CFG` writer - External INT15 Mode"]
pub type Eint15CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - External INT8 Mode"]
    #[inline(always)]
    pub fn eint8_cfg(&self) -> Eint8CfgR {
        Eint8CfgR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External INT9 Mode"]
    #[inline(always)]
    pub fn eint9_cfg(&self) -> Eint9CfgR {
        Eint9CfgR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External INT10 Mode"]
    #[inline(always)]
    pub fn eint10_cfg(&self) -> Eint10CfgR {
        Eint10CfgR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - External INT11 Mode"]
    #[inline(always)]
    pub fn eint11_cfg(&self) -> Eint11CfgR {
        Eint11CfgR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - External INT12 Mode"]
    #[inline(always)]
    pub fn eint12_cfg(&self) -> Eint12CfgR {
        Eint12CfgR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - External INT13 Mode"]
    #[inline(always)]
    pub fn eint13_cfg(&self) -> Eint13CfgR {
        Eint13CfgR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - External INT14 Mode"]
    #[inline(always)]
    pub fn eint14_cfg(&self) -> Eint14CfgR {
        Eint14CfgR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - External INT15 Mode"]
    #[inline(always)]
    pub fn eint15_cfg(&self) -> Eint15CfgR {
        Eint15CfgR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - External INT8 Mode"]
    #[inline(always)]
    pub fn eint8_cfg(&mut self) -> Eint8CfgW<'_, PdEintCfg1Spec> {
        Eint8CfgW::new(self, 0)
    }
    #[doc = "Bits 4:7 - External INT9 Mode"]
    #[inline(always)]
    pub fn eint9_cfg(&mut self) -> Eint9CfgW<'_, PdEintCfg1Spec> {
        Eint9CfgW::new(self, 4)
    }
    #[doc = "Bits 8:11 - External INT10 Mode"]
    #[inline(always)]
    pub fn eint10_cfg(&mut self) -> Eint10CfgW<'_, PdEintCfg1Spec> {
        Eint10CfgW::new(self, 8)
    }
    #[doc = "Bits 12:15 - External INT11 Mode"]
    #[inline(always)]
    pub fn eint11_cfg(&mut self) -> Eint11CfgW<'_, PdEintCfg1Spec> {
        Eint11CfgW::new(self, 12)
    }
    #[doc = "Bits 16:19 - External INT12 Mode"]
    #[inline(always)]
    pub fn eint12_cfg(&mut self) -> Eint12CfgW<'_, PdEintCfg1Spec> {
        Eint12CfgW::new(self, 16)
    }
    #[doc = "Bits 20:23 - External INT13 Mode"]
    #[inline(always)]
    pub fn eint13_cfg(&mut self) -> Eint13CfgW<'_, PdEintCfg1Spec> {
        Eint13CfgW::new(self, 20)
    }
    #[doc = "Bits 24:27 - External INT14 Mode"]
    #[inline(always)]
    pub fn eint14_cfg(&mut self) -> Eint14CfgW<'_, PdEintCfg1Spec> {
        Eint14CfgW::new(self, 24)
    }
    #[doc = "Bits 28:31 - External INT15 Mode"]
    #[inline(always)]
    pub fn eint15_cfg(&mut self) -> Eint15CfgW<'_, PdEintCfg1Spec> {
        Eint15CfgW::new(self, 28)
    }
}
#[doc = "PD External Interrupt Configure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_eint_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_eint_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdEintCfg1Spec;
impl crate::RegisterSpec for PdEintCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_eint_cfg1::R`](R) reader structure"]
impl crate::Readable for PdEintCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pd_eint_cfg1::W`](W) writer structure"]
impl crate::Writable for PdEintCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PD_EINT_CFG1 to value 0"]
impl crate::Resettable for PdEintCfg1Spec {}
