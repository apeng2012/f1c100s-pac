#[doc = "Register `PC_PULL0` reader"]
pub type R = crate::R<PcPull0Spec>;
#[doc = "Register `PC_PULL0` writer"]
pub type W = crate::W<PcPull0Spec>;
#[doc = "Field `PC0_PULL` reader - PC0 Pull Select"]
pub type Pc0PullR = crate::FieldReader;
#[doc = "Field `PC0_PULL` writer - PC0 Pull Select"]
pub type Pc0PullW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC1_PULL` reader - PC1 Pull Select"]
pub type Pc1PullR = crate::FieldReader;
#[doc = "Field `PC1_PULL` writer - PC1 Pull Select"]
pub type Pc1PullW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC2_PULL` reader - PC2 Pull Select"]
pub type Pc2PullR = crate::FieldReader;
#[doc = "Field `PC2_PULL` writer - PC2 Pull Select"]
pub type Pc2PullW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC3_PULL` reader - PC3 Pull Select"]
pub type Pc3PullR = crate::FieldReader;
#[doc = "Field `PC3_PULL` writer - PC3 Pull Select"]
pub type Pc3PullW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PC0 Pull Select"]
    #[inline(always)]
    pub fn pc0_pull(&self) -> Pc0PullR {
        Pc0PullR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PC1 Pull Select"]
    #[inline(always)]
    pub fn pc1_pull(&self) -> Pc1PullR {
        Pc1PullR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PC2 Pull Select"]
    #[inline(always)]
    pub fn pc2_pull(&self) -> Pc2PullR {
        Pc2PullR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PC3 Pull Select"]
    #[inline(always)]
    pub fn pc3_pull(&self) -> Pc3PullR {
        Pc3PullR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PC0 Pull Select"]
    #[inline(always)]
    pub fn pc0_pull(&mut self) -> Pc0PullW<'_, PcPull0Spec> {
        Pc0PullW::new(self, 0)
    }
    #[doc = "Bits 2:3 - PC1 Pull Select"]
    #[inline(always)]
    pub fn pc1_pull(&mut self) -> Pc1PullW<'_, PcPull0Spec> {
        Pc1PullW::new(self, 2)
    }
    #[doc = "Bits 4:5 - PC2 Pull Select"]
    #[inline(always)]
    pub fn pc2_pull(&mut self) -> Pc2PullW<'_, PcPull0Spec> {
        Pc2PullW::new(self, 4)
    }
    #[doc = "Bits 6:7 - PC3 Pull Select"]
    #[inline(always)]
    pub fn pc3_pull(&mut self) -> Pc3PullW<'_, PcPull0Spec> {
        Pc3PullW::new(self, 6)
    }
}
#[doc = "PC Pull Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_pull0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_pull0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcPull0Spec;
impl crate::RegisterSpec for PcPull0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc_pull0::R`](R) reader structure"]
impl crate::Readable for PcPull0Spec {}
#[doc = "`write(|w| ..)` method takes [`pc_pull0::W`](W) writer structure"]
impl crate::Writable for PcPull0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PC_PULL0 to value 0x40"]
impl crate::Resettable for PcPull0Spec {
    const RESET_VALUE: u32 = 0x40;
}
