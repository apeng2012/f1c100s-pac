#[doc = "Register `PB_PULL0` reader"]
pub type R = crate::R<PbPull0Spec>;
#[doc = "Register `PB_PULL0` writer"]
pub type W = crate::W<PbPull0Spec>;
#[doc = "Field `PB0_PULL` reader - PB0 Pull Select"]
pub type Pb0PullR = crate::FieldReader;
#[doc = "Field `PB0_PULL` writer - PB0 Pull Select"]
pub type Pb0PullW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB1_PULL` reader - PB1 Pull Select"]
pub type Pb1PullR = crate::FieldReader;
#[doc = "Field `PB1_PULL` writer - PB1 Pull Select"]
pub type Pb1PullW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB2_PULL` reader - PB2 Pull Select"]
pub type Pb2PullR = crate::FieldReader;
#[doc = "Field `PB2_PULL` writer - PB2 Pull Select"]
pub type Pb2PullW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB3_PULL` reader - PB3 Pull Select"]
pub type Pb3PullR = crate::FieldReader;
#[doc = "Field `PB3_PULL` writer - PB3 Pull Select"]
pub type Pb3PullW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PB0 Pull Select"]
    #[inline(always)]
    pub fn pb0_pull(&self) -> Pb0PullR {
        Pb0PullR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PB1 Pull Select"]
    #[inline(always)]
    pub fn pb1_pull(&self) -> Pb1PullR {
        Pb1PullR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PB2 Pull Select"]
    #[inline(always)]
    pub fn pb2_pull(&self) -> Pb2PullR {
        Pb2PullR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PB3 Pull Select"]
    #[inline(always)]
    pub fn pb3_pull(&self) -> Pb3PullR {
        Pb3PullR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PB0 Pull Select"]
    #[inline(always)]
    pub fn pb0_pull(&mut self) -> Pb0PullW<'_, PbPull0Spec> {
        Pb0PullW::new(self, 0)
    }
    #[doc = "Bits 2:3 - PB1 Pull Select"]
    #[inline(always)]
    pub fn pb1_pull(&mut self) -> Pb1PullW<'_, PbPull0Spec> {
        Pb1PullW::new(self, 2)
    }
    #[doc = "Bits 4:5 - PB2 Pull Select"]
    #[inline(always)]
    pub fn pb2_pull(&mut self) -> Pb2PullW<'_, PbPull0Spec> {
        Pb2PullW::new(self, 4)
    }
    #[doc = "Bits 6:7 - PB3 Pull Select"]
    #[inline(always)]
    pub fn pb3_pull(&mut self) -> Pb3PullW<'_, PbPull0Spec> {
        Pb3PullW::new(self, 6)
    }
}
#[doc = "PB Pull Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_pull0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_pull0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbPull0Spec;
impl crate::RegisterSpec for PbPull0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb_pull0::R`](R) reader structure"]
impl crate::Readable for PbPull0Spec {}
#[doc = "`write(|w| ..)` method takes [`pb_pull0::W`](W) writer structure"]
impl crate::Writable for PbPull0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PB_PULL0 to value 0"]
impl crate::Resettable for PbPull0Spec {}
