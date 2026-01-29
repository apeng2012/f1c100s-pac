#[doc = "Register `PA_CFG0` reader"]
pub type R = crate::R<PaCfg0Spec>;
#[doc = "Register `PA_CFG0` writer"]
pub type W = crate::W<PaCfg0Spec>;
#[doc = "Field `PA0_SELECT` reader - PA0 Select"]
pub type Pa0SelectR = crate::FieldReader;
#[doc = "Field `PA0_SELECT` writer - PA0 Select"]
pub type Pa0SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PA1_SELECT` reader - PA1 Select"]
pub type Pa1SelectR = crate::FieldReader;
#[doc = "Field `PA1_SELECT` writer - PA1 Select"]
pub type Pa1SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PA2_SELECT` reader - PA2 Select"]
pub type Pa2SelectR = crate::FieldReader;
#[doc = "Field `PA2_SELECT` writer - PA2 Select"]
pub type Pa2SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PA3_SELECT` reader - PA3 Select"]
pub type Pa3SelectR = crate::FieldReader;
#[doc = "Field `PA3_SELECT` writer - PA3 Select"]
pub type Pa3SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - PA0 Select"]
    #[inline(always)]
    pub fn pa0_select(&self) -> Pa0SelectR {
        Pa0SelectR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - PA1 Select"]
    #[inline(always)]
    pub fn pa1_select(&self) -> Pa1SelectR {
        Pa1SelectR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - PA2 Select"]
    #[inline(always)]
    pub fn pa2_select(&self) -> Pa2SelectR {
        Pa2SelectR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - PA3 Select"]
    #[inline(always)]
    pub fn pa3_select(&self) -> Pa3SelectR {
        Pa3SelectR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PA0 Select"]
    #[inline(always)]
    pub fn pa0_select(&mut self) -> Pa0SelectW<'_, PaCfg0Spec> {
        Pa0SelectW::new(self, 0)
    }
    #[doc = "Bits 4:6 - PA1 Select"]
    #[inline(always)]
    pub fn pa1_select(&mut self) -> Pa1SelectW<'_, PaCfg0Spec> {
        Pa1SelectW::new(self, 4)
    }
    #[doc = "Bits 8:10 - PA2 Select"]
    #[inline(always)]
    pub fn pa2_select(&mut self) -> Pa2SelectW<'_, PaCfg0Spec> {
        Pa2SelectW::new(self, 8)
    }
    #[doc = "Bits 12:14 - PA3 Select"]
    #[inline(always)]
    pub fn pa3_select(&mut self) -> Pa3SelectW<'_, PaCfg0Spec> {
        Pa3SelectW::new(self, 12)
    }
}
#[doc = "PA Configure Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaCfg0Spec;
impl crate::RegisterSpec for PaCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_cfg0::R`](R) reader structure"]
impl crate::Readable for PaCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pa_cfg0::W`](W) writer structure"]
impl crate::Writable for PaCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PA_CFG0 to value 0x7777_7777"]
impl crate::Resettable for PaCfg0Spec {
    const RESET_VALUE: u32 = 0x7777_7777;
}
