#[doc = "Register `USR` reader"]
pub type R = crate::R<UsrSpec>;
#[doc = "Field `BUSY` reader - UART Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `TFNF` reader - TX FIFO Not Full"]
pub type TfnfR = crate::BitReader;
#[doc = "Field `TFE` reader - TX FIFO Empty"]
pub type TfeR = crate::BitReader;
#[doc = "Field `RFNE` reader - RX FIFO Not Empty"]
pub type RfneR = crate::BitReader;
#[doc = "Field `RFF` reader - RX FIFO Full"]
pub type RffR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - UART Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Not Full"]
    #[inline(always)]
    pub fn tfnf(&self) -> TfnfR {
        TfnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX FIFO Empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO Not Empty"]
    #[inline(always)]
    pub fn rfne(&self) -> RfneR {
        RfneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Full"]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "UART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsrSpec;
impl crate::RegisterSpec for UsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usr::R`](R) reader structure"]
impl crate::Readable for UsrSpec {}
#[doc = "`reset()` method sets USR to value 0"]
impl crate::Resettable for UsrSpec {}
