#[doc = "Register `CPU_CLK_SRC` reader"]
pub type R = crate::R<CpuClkSrcSpec>;
#[doc = "Register `CPU_CLK_SRC` writer"]
pub type W = crate::W<CpuClkSrcSpec>;
#[doc = "Field `CPU_CLK_SRC_SEL` reader - CPU Clock Source Select: 00=LOSC, 01=OSC24M, 1X=PLL_CPU"]
pub type CpuClkSrcSelR = crate::FieldReader;
#[doc = "Field `CPU_CLK_SRC_SEL` writer - CPU Clock Source Select: 00=LOSC, 01=OSC24M, 1X=PLL_CPU"]
pub type CpuClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 16:17 - CPU Clock Source Select: 00=LOSC, 01=OSC24M, 1X=PLL_CPU"]
    #[inline(always)]
    pub fn cpu_clk_src_sel(&self) -> CpuClkSrcSelR {
        CpuClkSrcSelR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - CPU Clock Source Select: 00=LOSC, 01=OSC24M, 1X=PLL_CPU"]
    #[inline(always)]
    pub fn cpu_clk_src_sel(&mut self) -> CpuClkSrcSelW<'_, CpuClkSrcSpec> {
        CpuClkSrcSelW::new(self, 16)
    }
}
#[doc = "CPU Clock Source Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_clk_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_clk_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuClkSrcSpec;
impl crate::RegisterSpec for CpuClkSrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_clk_src::R`](R) reader structure"]
impl crate::Readable for CpuClkSrcSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu_clk_src::W`](W) writer structure"]
impl crate::Writable for CpuClkSrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_CLK_SRC to value 0x0001_0000"]
impl crate::Resettable for CpuClkSrcSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
