#[doc = "Register `PD_CFG0` reader"]
pub type R = crate::R<PdCfg0Spec>;
#[doc = "Register `PD_CFG0` writer"]
pub type W = crate::W<PdCfg0Spec>;
#[doc = "Field `PD0_SELECT` reader - PD0 Select"]
pub type Pd0SelectR = crate::FieldReader;
#[doc = "Field `PD0_SELECT` writer - PD0 Select"]
pub type Pd0SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD1_SELECT` reader - PD1 Select"]
pub type Pd1SelectR = crate::FieldReader;
#[doc = "Field `PD1_SELECT` writer - PD1 Select"]
pub type Pd1SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD2_SELECT` reader - PD2 Select"]
pub type Pd2SelectR = crate::FieldReader;
#[doc = "Field `PD2_SELECT` writer - PD2 Select"]
pub type Pd2SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD3_SELECT` reader - PD3 Select"]
pub type Pd3SelectR = crate::FieldReader;
#[doc = "Field `PD3_SELECT` writer - PD3 Select"]
pub type Pd3SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD4_SELECT` reader - PD4 Select"]
pub type Pd4SelectR = crate::FieldReader;
#[doc = "Field `PD4_SELECT` writer - PD4 Select"]
pub type Pd4SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD5_SELECT` reader - PD5 Select"]
pub type Pd5SelectR = crate::FieldReader;
#[doc = "Field `PD5_SELECT` writer - PD5 Select"]
pub type Pd5SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD6_SELECT` reader - PD6 Select"]
pub type Pd6SelectR = crate::FieldReader;
#[doc = "Field `PD6_SELECT` writer - PD6 Select"]
pub type Pd6SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD7_SELECT` reader - PD7 Select"]
pub type Pd7SelectR = crate::FieldReader;
#[doc = "Field `PD7_SELECT` writer - PD7 Select"]
pub type Pd7SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - PD0 Select"]
    #[inline(always)]
    pub fn pd0_select(&self) -> Pd0SelectR {
        Pd0SelectR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - PD1 Select"]
    #[inline(always)]
    pub fn pd1_select(&self) -> Pd1SelectR {
        Pd1SelectR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - PD2 Select"]
    #[inline(always)]
    pub fn pd2_select(&self) -> Pd2SelectR {
        Pd2SelectR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - PD3 Select"]
    #[inline(always)]
    pub fn pd3_select(&self) -> Pd3SelectR {
        Pd3SelectR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - PD4 Select"]
    #[inline(always)]
    pub fn pd4_select(&self) -> Pd4SelectR {
        Pd4SelectR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - PD5 Select"]
    #[inline(always)]
    pub fn pd5_select(&self) -> Pd5SelectR {
        Pd5SelectR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - PD6 Select"]
    #[inline(always)]
    pub fn pd6_select(&self) -> Pd6SelectR {
        Pd6SelectR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - PD7 Select"]
    #[inline(always)]
    pub fn pd7_select(&self) -> Pd7SelectR {
        Pd7SelectR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PD0 Select"]
    #[inline(always)]
    pub fn pd0_select(&mut self) -> Pd0SelectW<'_, PdCfg0Spec> {
        Pd0SelectW::new(self, 0)
    }
    #[doc = "Bits 4:6 - PD1 Select"]
    #[inline(always)]
    pub fn pd1_select(&mut self) -> Pd1SelectW<'_, PdCfg0Spec> {
        Pd1SelectW::new(self, 4)
    }
    #[doc = "Bits 8:10 - PD2 Select"]
    #[inline(always)]
    pub fn pd2_select(&mut self) -> Pd2SelectW<'_, PdCfg0Spec> {
        Pd2SelectW::new(self, 8)
    }
    #[doc = "Bits 12:14 - PD3 Select"]
    #[inline(always)]
    pub fn pd3_select(&mut self) -> Pd3SelectW<'_, PdCfg0Spec> {
        Pd3SelectW::new(self, 12)
    }
    #[doc = "Bits 16:18 - PD4 Select"]
    #[inline(always)]
    pub fn pd4_select(&mut self) -> Pd4SelectW<'_, PdCfg0Spec> {
        Pd4SelectW::new(self, 16)
    }
    #[doc = "Bits 20:22 - PD5 Select"]
    #[inline(always)]
    pub fn pd5_select(&mut self) -> Pd5SelectW<'_, PdCfg0Spec> {
        Pd5SelectW::new(self, 20)
    }
    #[doc = "Bits 24:26 - PD6 Select"]
    #[inline(always)]
    pub fn pd6_select(&mut self) -> Pd6SelectW<'_, PdCfg0Spec> {
        Pd6SelectW::new(self, 24)
    }
    #[doc = "Bits 28:30 - PD7 Select"]
    #[inline(always)]
    pub fn pd7_select(&mut self) -> Pd7SelectW<'_, PdCfg0Spec> {
        Pd7SelectW::new(self, 28)
    }
}
#[doc = "PD Configure Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdCfg0Spec;
impl crate::RegisterSpec for PdCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_cfg0::R`](R) reader structure"]
impl crate::Readable for PdCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pd_cfg0::W`](W) writer structure"]
impl crate::Writable for PdCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PD_CFG0 to value 0x7777_7777"]
impl crate::Resettable for PdCfg0Spec {
    const RESET_VALUE: u32 = 0x7777_7777;
}
