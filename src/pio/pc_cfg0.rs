#[doc = "Register `PC_CFG0` reader"]
pub type R = crate::R<PcCfg0Spec>;
#[doc = "Register `PC_CFG0` writer"]
pub type W = crate::W<PcCfg0Spec>;
#[doc = "Field `PC0_SELECT` reader - PC0 Select"]
pub type Pc0SelectR = crate::FieldReader;
#[doc = "Field `PC0_SELECT` writer - PC0 Select"]
pub type Pc0SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PC1_SELECT` reader - PC1 Select"]
pub type Pc1SelectR = crate::FieldReader;
#[doc = "Field `PC1_SELECT` writer - PC1 Select"]
pub type Pc1SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PC2_SELECT` reader - PC2 Select"]
pub type Pc2SelectR = crate::FieldReader;
#[doc = "Field `PC2_SELECT` writer - PC2 Select"]
pub type Pc2SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PC3_SELECT` reader - PC3 Select"]
pub type Pc3SelectR = crate::FieldReader;
#[doc = "Field `PC3_SELECT` writer - PC3 Select"]
pub type Pc3SelectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - PC0 Select"]
    #[inline(always)]
    pub fn pc0_select(&self) -> Pc0SelectR {
        Pc0SelectR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - PC1 Select"]
    #[inline(always)]
    pub fn pc1_select(&self) -> Pc1SelectR {
        Pc1SelectR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - PC2 Select"]
    #[inline(always)]
    pub fn pc2_select(&self) -> Pc2SelectR {
        Pc2SelectR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - PC3 Select"]
    #[inline(always)]
    pub fn pc3_select(&self) -> Pc3SelectR {
        Pc3SelectR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PC0 Select"]
    #[inline(always)]
    pub fn pc0_select(&mut self) -> Pc0SelectW<'_, PcCfg0Spec> {
        Pc0SelectW::new(self, 0)
    }
    #[doc = "Bits 4:6 - PC1 Select"]
    #[inline(always)]
    pub fn pc1_select(&mut self) -> Pc1SelectW<'_, PcCfg0Spec> {
        Pc1SelectW::new(self, 4)
    }
    #[doc = "Bits 8:10 - PC2 Select"]
    #[inline(always)]
    pub fn pc2_select(&mut self) -> Pc2SelectW<'_, PcCfg0Spec> {
        Pc2SelectW::new(self, 8)
    }
    #[doc = "Bits 12:14 - PC3 Select"]
    #[inline(always)]
    pub fn pc3_select(&mut self) -> Pc3SelectW<'_, PcCfg0Spec> {
        Pc3SelectW::new(self, 12)
    }
}
#[doc = "PC Configure Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcCfg0Spec;
impl crate::RegisterSpec for PcCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc_cfg0::R`](R) reader structure"]
impl crate::Readable for PcCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pc_cfg0::W`](W) writer structure"]
impl crate::Writable for PcCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PC_CFG0 to value 0x7777_7777"]
impl crate::Resettable for PcCfg0Spec {
    const RESET_VALUE: u32 = 0x7777_7777;
}
