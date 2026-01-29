#[doc = "Register `PE_CFG0` reader"]
pub type R = crate::R<PeCfg0Spec>;
#[doc = "Register `PE_CFG0` writer"]
pub type W = crate::W<PeCfg0Spec>;
#[doc = "Field `PE0_SELECT` reader - PE0 Select"]
pub type Pe0SelectR = crate::FieldReader;
#[doc = "Field `PE0_SELECT` writer - PE0 Select"]
pub type Pe0SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PE1_SELECT` reader - PE1 Select"]
pub type Pe1SelectR = crate::FieldReader;
#[doc = "Field `PE1_SELECT` writer - PE1 Select"]
pub type Pe1SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PE2_SELECT` reader - PE2 Select"]
pub type Pe2SelectR = crate::FieldReader;
#[doc = "Field `PE2_SELECT` writer - PE2 Select"]
pub type Pe2SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PE3_SELECT` reader - PE3 Select"]
pub type Pe3SelectR = crate::FieldReader;
#[doc = "Field `PE3_SELECT` writer - PE3 Select"]
pub type Pe3SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PE4_SELECT` reader - PE4 Select"]
pub type Pe4SelectR = crate::FieldReader;
#[doc = "Field `PE4_SELECT` writer - PE4 Select"]
pub type Pe4SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PE5_SELECT` reader - PE5 Select"]
pub type Pe5SelectR = crate::FieldReader;
#[doc = "Field `PE5_SELECT` writer - PE5 Select"]
pub type Pe5SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PE6_SELECT` reader - PE6 Select"]
pub type Pe6SelectR = crate::FieldReader;
#[doc = "Field `PE6_SELECT` writer - PE6 Select"]
pub type Pe6SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PE7_SELECT` reader - PE7 Select"]
pub type Pe7SelectR = crate::FieldReader;
#[doc = "Field `PE7_SELECT` writer - PE7 Select"]
pub type Pe7SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - PE0 Select"]
    #[inline(always)]
    pub fn pe0_select(&self) -> Pe0SelectR {
        Pe0SelectR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - PE1 Select"]
    #[inline(always)]
    pub fn pe1_select(&self) -> Pe1SelectR {
        Pe1SelectR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - PE2 Select"]
    #[inline(always)]
    pub fn pe2_select(&self) -> Pe2SelectR {
        Pe2SelectR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - PE3 Select"]
    #[inline(always)]
    pub fn pe3_select(&self) -> Pe3SelectR {
        Pe3SelectR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - PE4 Select"]
    #[inline(always)]
    pub fn pe4_select(&self) -> Pe4SelectR {
        Pe4SelectR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - PE5 Select"]
    #[inline(always)]
    pub fn pe5_select(&self) -> Pe5SelectR {
        Pe5SelectR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - PE6 Select"]
    #[inline(always)]
    pub fn pe6_select(&self) -> Pe6SelectR {
        Pe6SelectR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - PE7 Select"]
    #[inline(always)]
    pub fn pe7_select(&self) -> Pe7SelectR {
        Pe7SelectR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PE0 Select"]
    #[inline(always)]
    pub fn pe0_select(&mut self) -> Pe0SelectW<'_, PeCfg0Spec> {
        Pe0SelectW::new(self, 0)
    }
    #[doc = "Bits 4:6 - PE1 Select"]
    #[inline(always)]
    pub fn pe1_select(&mut self) -> Pe1SelectW<'_, PeCfg0Spec> {
        Pe1SelectW::new(self, 4)
    }
    #[doc = "Bits 8:10 - PE2 Select"]
    #[inline(always)]
    pub fn pe2_select(&mut self) -> Pe2SelectW<'_, PeCfg0Spec> {
        Pe2SelectW::new(self, 8)
    }
    #[doc = "Bits 12:14 - PE3 Select"]
    #[inline(always)]
    pub fn pe3_select(&mut self) -> Pe3SelectW<'_, PeCfg0Spec> {
        Pe3SelectW::new(self, 12)
    }
    #[doc = "Bits 16:18 - PE4 Select"]
    #[inline(always)]
    pub fn pe4_select(&mut self) -> Pe4SelectW<'_, PeCfg0Spec> {
        Pe4SelectW::new(self, 16)
    }
    #[doc = "Bits 20:22 - PE5 Select"]
    #[inline(always)]
    pub fn pe5_select(&mut self) -> Pe5SelectW<'_, PeCfg0Spec> {
        Pe5SelectW::new(self, 20)
    }
    #[doc = "Bits 24:26 - PE6 Select"]
    #[inline(always)]
    pub fn pe6_select(&mut self) -> Pe6SelectW<'_, PeCfg0Spec> {
        Pe6SelectW::new(self, 24)
    }
    #[doc = "Bits 28:30 - PE7 Select"]
    #[inline(always)]
    pub fn pe7_select(&mut self) -> Pe7SelectW<'_, PeCfg0Spec> {
        Pe7SelectW::new(self, 28)
    }
}
#[doc = "PE Configure Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeCfg0Spec;
impl crate::RegisterSpec for PeCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_cfg0::R`](R) reader structure"]
impl crate::Readable for PeCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pe_cfg0::W`](W) writer structure"]
impl crate::Writable for PeCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PE_CFG0 to value 0x7777_7777"]
impl crate::Resettable for PeCfg0Spec {
    const RESET_VALUE: u32 = 0x7777_7777;
}
