#[doc = "Register `PB_CFG0` reader"]
pub type R = crate::R<PbCfg0Spec>;
#[doc = "Register `PB_CFG0` writer"]
pub type W = crate::W<PbCfg0Spec>;
#[doc = "Field `PB0_SELECT` reader - PB0 Select"]
pub type Pb0SelectR = crate::FieldReader;
#[doc = "Field `PB0_SELECT` writer - PB0 Select"]
pub type Pb0SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PB1_SELECT` reader - PB1 Select"]
pub type Pb1SelectR = crate::FieldReader;
#[doc = "Field `PB1_SELECT` writer - PB1 Select"]
pub type Pb1SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PB2_SELECT` reader - PB2 Select"]
pub type Pb2SelectR = crate::FieldReader;
#[doc = "Field `PB2_SELECT` writer - PB2 Select"]
pub type Pb2SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PB3_SELECT` reader - PB3 Select"]
pub type Pb3SelectR = crate::FieldReader;
#[doc = "Field `PB3_SELECT` writer - PB3 Select"]
pub type Pb3SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - PB0 Select"]
    #[inline(always)]
    pub fn pb0_select(&self) -> Pb0SelectR {
        Pb0SelectR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - PB1 Select"]
    #[inline(always)]
    pub fn pb1_select(&self) -> Pb1SelectR {
        Pb1SelectR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - PB2 Select"]
    #[inline(always)]
    pub fn pb2_select(&self) -> Pb2SelectR {
        Pb2SelectR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - PB3 Select"]
    #[inline(always)]
    pub fn pb3_select(&self) -> Pb3SelectR {
        Pb3SelectR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PB0 Select"]
    #[inline(always)]
    pub fn pb0_select(&mut self) -> Pb0SelectW<'_, PbCfg0Spec> {
        Pb0SelectW::new(self, 0)
    }
    #[doc = "Bits 4:6 - PB1 Select"]
    #[inline(always)]
    pub fn pb1_select(&mut self) -> Pb1SelectW<'_, PbCfg0Spec> {
        Pb1SelectW::new(self, 4)
    }
    #[doc = "Bits 8:10 - PB2 Select"]
    #[inline(always)]
    pub fn pb2_select(&mut self) -> Pb2SelectW<'_, PbCfg0Spec> {
        Pb2SelectW::new(self, 8)
    }
    #[doc = "Bits 12:14 - PB3 Select"]
    #[inline(always)]
    pub fn pb3_select(&mut self) -> Pb3SelectW<'_, PbCfg0Spec> {
        Pb3SelectW::new(self, 12)
    }
}
#[doc = "PB Configure Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbCfg0Spec;
impl crate::RegisterSpec for PbCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb_cfg0::R`](R) reader structure"]
impl crate::Readable for PbCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pb_cfg0::W`](W) writer structure"]
impl crate::Writable for PbCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PB_CFG0 to value 0x7777_2222"]
impl crate::Resettable for PbCfg0Spec {
    const RESET_VALUE: u32 = 0x7777_2222;
}
