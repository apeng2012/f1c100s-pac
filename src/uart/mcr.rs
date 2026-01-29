#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Field `DTR` reader - Data Terminal Ready"]
pub type DtrR = crate::BitReader;
#[doc = "Field `DTR` writer - Data Terminal Ready"]
pub type DtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS` reader - Request to Send"]
pub type RtsR = crate::BitReader;
#[doc = "Field `RTS` writer - Request to Send"]
pub type RtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LB` reader - Loop Back Mode"]
pub type LbR = crate::BitReader;
#[doc = "Field `LB` writer - Loop Back Mode"]
pub type LbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFCE` reader - Auto Flow Control Enable"]
pub type AfceR = crate::BitReader;
#[doc = "Field `AFCE` writer - Auto Flow Control Enable"]
pub type AfceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIRE` reader - SIR Mode Enable"]
pub type SireR = crate::BitReader;
#[doc = "Field `SIRE` writer - SIR Mode Enable"]
pub type SireW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data Terminal Ready"]
    #[inline(always)]
    pub fn dtr(&self) -> DtrR {
        DtrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Request to Send"]
    #[inline(always)]
    pub fn rts(&self) -> RtsR {
        RtsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Loop Back Mode"]
    #[inline(always)]
    pub fn lb(&self) -> LbR {
        LbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Auto Flow Control Enable"]
    #[inline(always)]
    pub fn afce(&self) -> AfceR {
        AfceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SIR Mode Enable"]
    #[inline(always)]
    pub fn sire(&self) -> SireR {
        SireR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Terminal Ready"]
    #[inline(always)]
    pub fn dtr(&mut self) -> DtrW<'_, McrSpec> {
        DtrW::new(self, 0)
    }
    #[doc = "Bit 1 - Request to Send"]
    #[inline(always)]
    pub fn rts(&mut self) -> RtsW<'_, McrSpec> {
        RtsW::new(self, 1)
    }
    #[doc = "Bit 4 - Loop Back Mode"]
    #[inline(always)]
    pub fn lb(&mut self) -> LbW<'_, McrSpec> {
        LbW::new(self, 4)
    }
    #[doc = "Bit 5 - Auto Flow Control Enable"]
    #[inline(always)]
    pub fn afce(&mut self) -> AfceW<'_, McrSpec> {
        AfceW::new(self, 5)
    }
    #[doc = "Bit 6 - SIR Mode Enable"]
    #[inline(always)]
    pub fn sire(&mut self) -> SireW<'_, McrSpec> {
        SireW::new(self, 6)
    }
}
#[doc = "Modem Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for McrSpec {}
