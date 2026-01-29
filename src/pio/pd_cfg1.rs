#[doc = "Register `PD_CFG1` reader"]
pub type R = crate::R<PdCfg1Spec>;
#[doc = "Register `PD_CFG1` writer"]
pub type W = crate::W<PdCfg1Spec>;
#[doc = "Field `PD8_SELECT` reader - PD8 Select"]
pub type Pd8SelectR = crate::FieldReader;
#[doc = "Field `PD8_SELECT` writer - PD8 Select"]
pub type Pd8SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD9_SELECT` reader - PD9 Select"]
pub type Pd9SelectR = crate::FieldReader;
#[doc = "Field `PD9_SELECT` writer - PD9 Select"]
pub type Pd9SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD10_SELECT` reader - PD10 Select"]
pub type Pd10SelectR = crate::FieldReader;
#[doc = "Field `PD10_SELECT` writer - PD10 Select"]
pub type Pd10SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD11_SELECT` reader - PD11 Select"]
pub type Pd11SelectR = crate::FieldReader;
#[doc = "Field `PD11_SELECT` writer - PD11 Select"]
pub type Pd11SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD12_SELECT` reader - PD12 Select"]
pub type Pd12SelectR = crate::FieldReader;
#[doc = "Field `PD12_SELECT` writer - PD12 Select"]
pub type Pd12SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD13_SELECT` reader - PD13 Select"]
pub type Pd13SelectR = crate::FieldReader;
#[doc = "Field `PD13_SELECT` writer - PD13 Select"]
pub type Pd13SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD14_SELECT` reader - PD14 Select"]
pub type Pd14SelectR = crate::FieldReader;
#[doc = "Field `PD14_SELECT` writer - PD14 Select"]
pub type Pd14SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD15_SELECT` reader - PD15 Select"]
pub type Pd15SelectR = crate::FieldReader;
#[doc = "Field `PD15_SELECT` writer - PD15 Select"]
pub type Pd15SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - PD8 Select"]
    #[inline(always)]
    pub fn pd8_select(&self) -> Pd8SelectR {
        Pd8SelectR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - PD9 Select"]
    #[inline(always)]
    pub fn pd9_select(&self) -> Pd9SelectR {
        Pd9SelectR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - PD10 Select"]
    #[inline(always)]
    pub fn pd10_select(&self) -> Pd10SelectR {
        Pd10SelectR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - PD11 Select"]
    #[inline(always)]
    pub fn pd11_select(&self) -> Pd11SelectR {
        Pd11SelectR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - PD12 Select"]
    #[inline(always)]
    pub fn pd12_select(&self) -> Pd12SelectR {
        Pd12SelectR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - PD13 Select"]
    #[inline(always)]
    pub fn pd13_select(&self) -> Pd13SelectR {
        Pd13SelectR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - PD14 Select"]
    #[inline(always)]
    pub fn pd14_select(&self) -> Pd14SelectR {
        Pd14SelectR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - PD15 Select"]
    #[inline(always)]
    pub fn pd15_select(&self) -> Pd15SelectR {
        Pd15SelectR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PD8 Select"]
    #[inline(always)]
    pub fn pd8_select(&mut self) -> Pd8SelectW<'_, PdCfg1Spec> {
        Pd8SelectW::new(self, 0)
    }
    #[doc = "Bits 4:6 - PD9 Select"]
    #[inline(always)]
    pub fn pd9_select(&mut self) -> Pd9SelectW<'_, PdCfg1Spec> {
        Pd9SelectW::new(self, 4)
    }
    #[doc = "Bits 8:10 - PD10 Select"]
    #[inline(always)]
    pub fn pd10_select(&mut self) -> Pd10SelectW<'_, PdCfg1Spec> {
        Pd10SelectW::new(self, 8)
    }
    #[doc = "Bits 12:14 - PD11 Select"]
    #[inline(always)]
    pub fn pd11_select(&mut self) -> Pd11SelectW<'_, PdCfg1Spec> {
        Pd11SelectW::new(self, 12)
    }
    #[doc = "Bits 16:18 - PD12 Select"]
    #[inline(always)]
    pub fn pd12_select(&mut self) -> Pd12SelectW<'_, PdCfg1Spec> {
        Pd12SelectW::new(self, 16)
    }
    #[doc = "Bits 20:22 - PD13 Select"]
    #[inline(always)]
    pub fn pd13_select(&mut self) -> Pd13SelectW<'_, PdCfg1Spec> {
        Pd13SelectW::new(self, 20)
    }
    #[doc = "Bits 24:26 - PD14 Select"]
    #[inline(always)]
    pub fn pd14_select(&mut self) -> Pd14SelectW<'_, PdCfg1Spec> {
        Pd14SelectW::new(self, 24)
    }
    #[doc = "Bits 28:30 - PD15 Select"]
    #[inline(always)]
    pub fn pd15_select(&mut self) -> Pd15SelectW<'_, PdCfg1Spec> {
        Pd15SelectW::new(self, 28)
    }
}
#[doc = "PD Configure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdCfg1Spec;
impl crate::RegisterSpec for PdCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_cfg1::R`](R) reader structure"]
impl crate::Readable for PdCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pd_cfg1::W`](W) writer structure"]
impl crate::Writable for PdCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PD_CFG1 to value 0x7777_7777"]
impl crate::Resettable for PdCfg1Spec {
    const RESET_VALUE: u32 = 0x7777_7777;
}
