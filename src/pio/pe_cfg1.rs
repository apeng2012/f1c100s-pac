#[doc = "Register `PE_CFG1` reader"]
pub type R = crate::R<PeCfg1Spec>;
#[doc = "Register `PE_CFG1` writer"]
pub type W = crate::W<PeCfg1Spec>;
#[doc = "Field `PE8_SELECT` reader - PE8 Select"]
pub type Pe8SelectR = crate::FieldReader;
#[doc = "Field `PE8_SELECT` writer - PE8 Select"]
pub type Pe8SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PE9_SELECT` reader - PE9 Select"]
pub type Pe9SelectR = crate::FieldReader;
#[doc = "Field `PE9_SELECT` writer - PE9 Select"]
pub type Pe9SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PE10_SELECT` reader - PE10 Select"]
pub type Pe10SelectR = crate::FieldReader;
#[doc = "Field `PE10_SELECT` writer - PE10 Select"]
pub type Pe10SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PE11_SELECT` reader - PE11 Select"]
pub type Pe11SelectR = crate::FieldReader;
#[doc = "Field `PE11_SELECT` writer - PE11 Select"]
pub type Pe11SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PE12_SELECT` reader - PE12 Select"]
pub type Pe12SelectR = crate::FieldReader;
#[doc = "Field `PE12_SELECT` writer - PE12 Select"]
pub type Pe12SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - PE8 Select"]
    #[inline(always)]
    pub fn pe8_select(&self) -> Pe8SelectR {
        Pe8SelectR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - PE9 Select"]
    #[inline(always)]
    pub fn pe9_select(&self) -> Pe9SelectR {
        Pe9SelectR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - PE10 Select"]
    #[inline(always)]
    pub fn pe10_select(&self) -> Pe10SelectR {
        Pe10SelectR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - PE11 Select"]
    #[inline(always)]
    pub fn pe11_select(&self) -> Pe11SelectR {
        Pe11SelectR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - PE12 Select"]
    #[inline(always)]
    pub fn pe12_select(&self) -> Pe12SelectR {
        Pe12SelectR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PE8 Select"]
    #[inline(always)]
    pub fn pe8_select(&mut self) -> Pe8SelectW<'_, PeCfg1Spec> {
        Pe8SelectW::new(self, 0)
    }
    #[doc = "Bits 4:6 - PE9 Select"]
    #[inline(always)]
    pub fn pe9_select(&mut self) -> Pe9SelectW<'_, PeCfg1Spec> {
        Pe9SelectW::new(self, 4)
    }
    #[doc = "Bits 8:10 - PE10 Select"]
    #[inline(always)]
    pub fn pe10_select(&mut self) -> Pe10SelectW<'_, PeCfg1Spec> {
        Pe10SelectW::new(self, 8)
    }
    #[doc = "Bits 12:14 - PE11 Select"]
    #[inline(always)]
    pub fn pe11_select(&mut self) -> Pe11SelectW<'_, PeCfg1Spec> {
        Pe11SelectW::new(self, 12)
    }
    #[doc = "Bits 16:18 - PE12 Select"]
    #[inline(always)]
    pub fn pe12_select(&mut self) -> Pe12SelectW<'_, PeCfg1Spec> {
        Pe12SelectW::new(self, 16)
    }
}
#[doc = "PE Configure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeCfg1Spec;
impl crate::RegisterSpec for PeCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_cfg1::R`](R) reader structure"]
impl crate::Readable for PeCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pe_cfg1::W`](W) writer structure"]
impl crate::Writable for PeCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PE_CFG1 to value 0x0007_7777"]
impl crate::Resettable for PeCfg1Spec {
    const RESET_VALUE: u32 = 0x0007_7777;
}
