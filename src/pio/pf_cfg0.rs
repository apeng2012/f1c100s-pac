#[doc = "Register `PF_CFG0` reader"]
pub type R = crate::R<PfCfg0Spec>;
#[doc = "Register `PF_CFG0` writer"]
pub type W = crate::W<PfCfg0Spec>;
#[doc = "Field `PF0_SELECT` reader - PF0 Select"]
pub type Pf0SelectR = crate::FieldReader;
#[doc = "Field `PF0_SELECT` writer - PF0 Select"]
pub type Pf0SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PF1_SELECT` reader - PF1 Select"]
pub type Pf1SelectR = crate::FieldReader;
#[doc = "Field `PF1_SELECT` writer - PF1 Select"]
pub type Pf1SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PF2_SELECT` reader - PF2 Select"]
pub type Pf2SelectR = crate::FieldReader;
#[doc = "Field `PF2_SELECT` writer - PF2 Select"]
pub type Pf2SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PF3_SELECT` reader - PF3 Select"]
pub type Pf3SelectR = crate::FieldReader;
#[doc = "Field `PF3_SELECT` writer - PF3 Select"]
pub type Pf3SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PF4_SELECT` reader - PF4 Select"]
pub type Pf4SelectR = crate::FieldReader;
#[doc = "Field `PF4_SELECT` writer - PF4 Select"]
pub type Pf4SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PF5_SELECT` reader - PF5 Select"]
pub type Pf5SelectR = crate::FieldReader;
#[doc = "Field `PF5_SELECT` writer - PF5 Select"]
pub type Pf5SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - PF0 Select"]
    #[inline(always)]
    pub fn pf0_select(&self) -> Pf0SelectR {
        Pf0SelectR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - PF1 Select"]
    #[inline(always)]
    pub fn pf1_select(&self) -> Pf1SelectR {
        Pf1SelectR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - PF2 Select"]
    #[inline(always)]
    pub fn pf2_select(&self) -> Pf2SelectR {
        Pf2SelectR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - PF3 Select"]
    #[inline(always)]
    pub fn pf3_select(&self) -> Pf3SelectR {
        Pf3SelectR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - PF4 Select"]
    #[inline(always)]
    pub fn pf4_select(&self) -> Pf4SelectR {
        Pf4SelectR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - PF5 Select"]
    #[inline(always)]
    pub fn pf5_select(&self) -> Pf5SelectR {
        Pf5SelectR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PF0 Select"]
    #[inline(always)]
    pub fn pf0_select(&mut self) -> Pf0SelectW<'_, PfCfg0Spec> {
        Pf0SelectW::new(self, 0)
    }
    #[doc = "Bits 4:6 - PF1 Select"]
    #[inline(always)]
    pub fn pf1_select(&mut self) -> Pf1SelectW<'_, PfCfg0Spec> {
        Pf1SelectW::new(self, 4)
    }
    #[doc = "Bits 8:10 - PF2 Select"]
    #[inline(always)]
    pub fn pf2_select(&mut self) -> Pf2SelectW<'_, PfCfg0Spec> {
        Pf2SelectW::new(self, 8)
    }
    #[doc = "Bits 12:14 - PF3 Select"]
    #[inline(always)]
    pub fn pf3_select(&mut self) -> Pf3SelectW<'_, PfCfg0Spec> {
        Pf3SelectW::new(self, 12)
    }
    #[doc = "Bits 16:18 - PF4 Select"]
    #[inline(always)]
    pub fn pf4_select(&mut self) -> Pf4SelectW<'_, PfCfg0Spec> {
        Pf4SelectW::new(self, 16)
    }
    #[doc = "Bits 20:22 - PF5 Select"]
    #[inline(always)]
    pub fn pf5_select(&mut self) -> Pf5SelectW<'_, PfCfg0Spec> {
        Pf5SelectW::new(self, 20)
    }
}
#[doc = "PF Configure Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfCfg0Spec;
impl crate::RegisterSpec for PfCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_cfg0::R`](R) reader structure"]
impl crate::Readable for PfCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pf_cfg0::W`](W) writer structure"]
impl crate::Writable for PfCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PF_CFG0 to value 0x0037_3333"]
impl crate::Resettable for PfCfg0Spec {
    const RESET_VALUE: u32 = 0x0037_3333;
}
