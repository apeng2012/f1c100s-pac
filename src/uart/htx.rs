#[doc = "Register `HTX` reader"]
pub type R = crate::R<HtxSpec>;
#[doc = "Register `HTX` writer"]
pub type W = crate::W<HtxSpec>;
#[doc = "Field `HALT_TX` reader - Halt TX"]
pub type HaltTxR = crate::BitReader;
#[doc = "Field `HALT_TX` writer - Halt TX"]
pub type HaltTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHCFG_AT_BUSY` reader - Change Config While Busy"]
pub type ChcfgAtBusyR = crate::BitReader;
#[doc = "Field `CHCFG_AT_BUSY` writer - Change Config While Busy"]
pub type ChcfgAtBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANGE_UPDATE` reader - Configuration Update Trigger"]
pub type ChangeUpdateR = crate::BitReader;
#[doc = "Field `CHANGE_UPDATE` writer - Configuration Update Trigger"]
pub type ChangeUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIR_TX_INVERT` reader - SIR TX Pulse Polarity Invert"]
pub type SirTxInvertR = crate::BitReader;
#[doc = "Field `SIR_TX_INVERT` writer - SIR TX Pulse Polarity Invert"]
pub type SirTxInvertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIR_RX_INVERT` reader - SIR RX Pulse Polarity Invert"]
pub type SirRxInvertR = crate::BitReader;
#[doc = "Field `SIR_RX_INVERT` writer - SIR RX Pulse Polarity Invert"]
pub type SirRxInvertW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Halt TX"]
    #[inline(always)]
    pub fn halt_tx(&self) -> HaltTxR {
        HaltTxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Change Config While Busy"]
    #[inline(always)]
    pub fn chcfg_at_busy(&self) -> ChcfgAtBusyR {
        ChcfgAtBusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configuration Update Trigger"]
    #[inline(always)]
    pub fn change_update(&self) -> ChangeUpdateR {
        ChangeUpdateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SIR TX Pulse Polarity Invert"]
    #[inline(always)]
    pub fn sir_tx_invert(&self) -> SirTxInvertR {
        SirTxInvertR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SIR RX Pulse Polarity Invert"]
    #[inline(always)]
    pub fn sir_rx_invert(&self) -> SirRxInvertR {
        SirRxInvertR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt TX"]
    #[inline(always)]
    pub fn halt_tx(&mut self) -> HaltTxW<'_, HtxSpec> {
        HaltTxW::new(self, 0)
    }
    #[doc = "Bit 1 - Change Config While Busy"]
    #[inline(always)]
    pub fn chcfg_at_busy(&mut self) -> ChcfgAtBusyW<'_, HtxSpec> {
        ChcfgAtBusyW::new(self, 1)
    }
    #[doc = "Bit 2 - Configuration Update Trigger"]
    #[inline(always)]
    pub fn change_update(&mut self) -> ChangeUpdateW<'_, HtxSpec> {
        ChangeUpdateW::new(self, 2)
    }
    #[doc = "Bit 4 - SIR TX Pulse Polarity Invert"]
    #[inline(always)]
    pub fn sir_tx_invert(&mut self) -> SirTxInvertW<'_, HtxSpec> {
        SirTxInvertW::new(self, 4)
    }
    #[doc = "Bit 5 - SIR RX Pulse Polarity Invert"]
    #[inline(always)]
    pub fn sir_rx_invert(&mut self) -> SirRxInvertW<'_, HtxSpec> {
        SirRxInvertW::new(self, 5)
    }
}
#[doc = "Halt TX Register\n\nYou can [`read`](crate::Reg::read) this register and get [`htx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HtxSpec;
impl crate::RegisterSpec for HtxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`htx::R`](R) reader structure"]
impl crate::Readable for HtxSpec {}
#[doc = "`write(|w| ..)` method takes [`htx::W`](W) writer structure"]
impl crate::Writable for HtxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HTX to value 0"]
impl crate::Resettable for HtxSpec {}
