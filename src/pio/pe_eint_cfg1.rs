#[doc = "Register `PE_EINT_CFG1` reader"]
pub type R = crate::R<PeEintCfg1Spec>;
#[doc = "Register `PE_EINT_CFG1` writer"]
pub type W = crate::W<PeEintCfg1Spec>;
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
}
impl W {
    #[doc = "Bits 0:3 - External INT8 Mode"]
    #[inline(always)]
    pub fn eint8_cfg(&mut self) -> Eint8CfgW<'_, PeEintCfg1Spec> {
        Eint8CfgW::new(self, 0)
    }
    #[doc = "Bits 4:7 - External INT9 Mode"]
    #[inline(always)]
    pub fn eint9_cfg(&mut self) -> Eint9CfgW<'_, PeEintCfg1Spec> {
        Eint9CfgW::new(self, 4)
    }
    #[doc = "Bits 8:11 - External INT10 Mode"]
    #[inline(always)]
    pub fn eint10_cfg(&mut self) -> Eint10CfgW<'_, PeEintCfg1Spec> {
        Eint10CfgW::new(self, 8)
    }
    #[doc = "Bits 12:15 - External INT11 Mode"]
    #[inline(always)]
    pub fn eint11_cfg(&mut self) -> Eint11CfgW<'_, PeEintCfg1Spec> {
        Eint11CfgW::new(self, 12)
    }
    #[doc = "Bits 16:19 - External INT12 Mode"]
    #[inline(always)]
    pub fn eint12_cfg(&mut self) -> Eint12CfgW<'_, PeEintCfg1Spec> {
        Eint12CfgW::new(self, 16)
    }
}
#[doc = "PE External Interrupt Configure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_eint_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_eint_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeEintCfg1Spec;
impl crate::RegisterSpec for PeEintCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_eint_cfg1::R`](R) reader structure"]
impl crate::Readable for PeEintCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pe_eint_cfg1::W`](W) writer structure"]
impl crate::Writable for PeEintCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PE_EINT_CFG1 to value 0"]
impl crate::Resettable for PeEintCfg1Spec {}
