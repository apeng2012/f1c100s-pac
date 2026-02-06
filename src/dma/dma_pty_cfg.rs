#[doc = "Register `DMA_PTY_CFG` reader"]
pub type R = crate::R<DmaPtyCfgSpec>;
#[doc = "Register `DMA_PTY_CFG` writer"]
pub type W = crate::W<DmaPtyCfgSpec>;
#[doc = "Field `NDMA_CPU_PRIO` reader - NDMA/CPU Priority. 00=CPU>NDMA, 01=NDMA>CPU"]
pub type NdmaCpuPrioR = crate::FieldReader;
#[doc = "Field `NDMA_CPU_PRIO` writer - NDMA/CPU Priority. 00=CPU>NDMA, 01=NDMA>CPU"]
pub type NdmaCpuPrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AC320_PRIO_CNT` reader - AC320 Priority Counter. AC320 can continuously access AHB for N+1 times."]
pub type Ac320PrioCntR = crate::FieldReader;
#[doc = "Field `AC320_PRIO_CNT` writer - AC320 Priority Counter. AC320 can continuously access AHB for N+1 times."]
pub type Ac320PrioCntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NDMA_PRIO_CNT` reader - NDMA Priority Counter. NDMA can continuously access AHB for N+1 times."]
pub type NdmaPrioCntR = crate::FieldReader;
#[doc = "Field `NDMA_PRIO_CNT` writer - NDMA Priority Counter. NDMA can continuously access AHB for N+1 times."]
pub type NdmaPrioCntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DMA_AUTO_GATING` reader - DMA Auto Clock Gating. 0=Enable auto gating, 1=Disable auto gating. Set to 1 for continuous mode."]
pub type DmaAutoGatingR = crate::BitReader;
#[doc = "Field `DMA_AUTO_GATING` writer - DMA Auto Clock Gating. 0=Enable auto gating, 1=Disable auto gating. Set to 1 for continuous mode."]
pub type DmaAutoGatingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - NDMA/CPU Priority. 00=CPU>NDMA, 01=NDMA>CPU"]
    #[inline(always)]
    pub fn ndma_cpu_prio(&self) -> NdmaCpuPrioR {
        NdmaCpuPrioR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - AC320 Priority Counter. AC320 can continuously access AHB for N+1 times."]
    #[inline(always)]
    pub fn ac320_prio_cnt(&self) -> Ac320PrioCntR {
        Ac320PrioCntR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - NDMA Priority Counter. NDMA can continuously access AHB for N+1 times."]
    #[inline(always)]
    pub fn ndma_prio_cnt(&self) -> NdmaPrioCntR {
        NdmaPrioCntR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 16 - DMA Auto Clock Gating. 0=Enable auto gating, 1=Disable auto gating. Set to 1 for continuous mode."]
    #[inline(always)]
    pub fn dma_auto_gating(&self) -> DmaAutoGatingR {
        DmaAutoGatingR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - NDMA/CPU Priority. 00=CPU>NDMA, 01=NDMA>CPU"]
    #[inline(always)]
    pub fn ndma_cpu_prio(&mut self) -> NdmaCpuPrioW<'_, DmaPtyCfgSpec> {
        NdmaCpuPrioW::new(self, 0)
    }
    #[doc = "Bits 4:6 - AC320 Priority Counter. AC320 can continuously access AHB for N+1 times."]
    #[inline(always)]
    pub fn ac320_prio_cnt(&mut self) -> Ac320PrioCntW<'_, DmaPtyCfgSpec> {
        Ac320PrioCntW::new(self, 4)
    }
    #[doc = "Bits 7:9 - NDMA Priority Counter. NDMA can continuously access AHB for N+1 times."]
    #[inline(always)]
    pub fn ndma_prio_cnt(&mut self) -> NdmaPrioCntW<'_, DmaPtyCfgSpec> {
        NdmaPrioCntW::new(self, 7)
    }
    #[doc = "Bit 16 - DMA Auto Clock Gating. 0=Enable auto gating, 1=Disable auto gating. Set to 1 for continuous mode."]
    #[inline(always)]
    pub fn dma_auto_gating(&mut self) -> DmaAutoGatingW<'_, DmaPtyCfgSpec> {
        DmaAutoGatingW::new(self, 16)
    }
}
#[doc = "DMA Priority Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_pty_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_pty_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaPtyCfgSpec;
impl crate::RegisterSpec for DmaPtyCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_pty_cfg::R`](R) reader structure"]
impl crate::Readable for DmaPtyCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_pty_cfg::W`](W) writer structure"]
impl crate::Writable for DmaPtyCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_PTY_CFG to value 0x0190"]
impl crate::Resettable for DmaPtyCfgSpec {
    const RESET_VALUE: u32 = 0x0190;
}
