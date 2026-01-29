#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `ERBFI` reader - Enable Received Data Available Interrupt"]
pub type ErbfiR = crate::BitReader;
#[doc = "Field `ERBFI` writer - Enable Received Data Available Interrupt"]
pub type ErbfiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETBEI` reader - Enable Transmit Holding Register Empty Interrupt"]
pub type EtbeiR = crate::BitReader;
#[doc = "Field `ETBEI` writer - Enable Transmit Holding Register Empty Interrupt"]
pub type EtbeiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELSI` reader - Enable Receiver Line Status Interrupt"]
pub type ElsiR = crate::BitReader;
#[doc = "Field `ELSI` writer - Enable Receiver Line Status Interrupt"]
pub type ElsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDSSI` reader - Enable Modem Status Interrupt"]
pub type EdssiR = crate::BitReader;
#[doc = "Field `EDSSI` writer - Enable Modem Status Interrupt"]
pub type EdssiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTIME` reader - Programmable THRE Interrupt Mode Enable"]
pub type PtimeR = crate::BitReader;
#[doc = "Field `PTIME` writer - Programmable THRE Interrupt Mode Enable"]
pub type PtimeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Received Data Available Interrupt"]
    #[inline(always)]
    pub fn erbfi(&self) -> ErbfiR {
        ErbfiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Transmit Holding Register Empty Interrupt"]
    #[inline(always)]
    pub fn etbei(&self) -> EtbeiR {
        EtbeiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Receiver Line Status Interrupt"]
    #[inline(always)]
    pub fn elsi(&self) -> ElsiR {
        ElsiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Modem Status Interrupt"]
    #[inline(always)]
    pub fn edssi(&self) -> EdssiR {
        EdssiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Programmable THRE Interrupt Mode Enable"]
    #[inline(always)]
    pub fn ptime(&self) -> PtimeR {
        PtimeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Received Data Available Interrupt"]
    #[inline(always)]
    pub fn erbfi(&mut self) -> ErbfiW<'_, IerSpec> {
        ErbfiW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Transmit Holding Register Empty Interrupt"]
    #[inline(always)]
    pub fn etbei(&mut self) -> EtbeiW<'_, IerSpec> {
        EtbeiW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Receiver Line Status Interrupt"]
    #[inline(always)]
    pub fn elsi(&mut self) -> ElsiW<'_, IerSpec> {
        ElsiW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Modem Status Interrupt"]
    #[inline(always)]
    pub fn edssi(&mut self) -> EdssiW<'_, IerSpec> {
        EdssiW::new(self, 3)
    }
    #[doc = "Bit 7 - Programmable THRE Interrupt Mode Enable"]
    #[inline(always)]
    pub fn ptime(&mut self) -> PtimeW<'_, IerSpec> {
        PtimeW::new(self, 7)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
