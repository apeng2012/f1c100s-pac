#[doc = "Register `PLL_CPU_CTRL` reader"]
pub type R = crate::R<PllCpuCtrlSpec>;
#[doc = "Register `PLL_CPU_CTRL` writer"]
pub type W = crate::W<PllCpuCtrlSpec>;
#[doc = "Field `PLL_FACTOR_M` reader - PLL Factor M (M=Factor+1, range 1-4)"]
pub type PllFactorMR = crate::FieldReader;
#[doc = "Field `PLL_FACTOR_M` writer - PLL Factor M (M=Factor+1, range 1-4)"]
pub type PllFactorMW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLL_FACTOR_K` reader - PLL Factor K (K=Factor+1, range 1-4)"]
pub type PllFactorKR = crate::FieldReader;
#[doc = "Field `PLL_FACTOR_K` writer - PLL Factor K (K=Factor+1, range 1-4)"]
pub type PllFactorKW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLL_FACTOR_N` reader - PLL Factor N (N=Factor+1, range 1-32)"]
pub type PllFactorNR = crate::FieldReader;
#[doc = "Field `PLL_FACTOR_N` writer - PLL Factor N (N=Factor+1, range 1-32)"]
pub type PllFactorNW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PLL_OUT_EXT_DIV_P` reader - PLL Output External Divider P: 00=/1, 01=/2, 10=/4"]
pub type PllOutExtDivPR = crate::FieldReader;
#[doc = "Field `PLL_OUT_EXT_DIV_P` writer - PLL Output External Divider P: 00=/1, 01=/2, 10=/4"]
pub type PllOutExtDivPW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LOCK` reader - PLL Lock Status (read-only)"]
pub type LockR = crate::BitReader;
#[doc = "Field `PLL_EN` reader - PLL Enable"]
pub type PllEnR = crate::BitReader;
#[doc = "Field `PLL_EN` writer - PLL Enable"]
pub type PllEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - PLL Factor M (M=Factor+1, range 1-4)"]
    #[inline(always)]
    pub fn pll_factor_m(&self) -> PllFactorMR {
        PllFactorMR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - PLL Factor K (K=Factor+1, range 1-4)"]
    #[inline(always)]
    pub fn pll_factor_k(&self) -> PllFactorKR {
        PllFactorKR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:12 - PLL Factor N (N=Factor+1, range 1-32)"]
    #[inline(always)]
    pub fn pll_factor_n(&self) -> PllFactorNR {
        PllFactorNR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - PLL Output External Divider P: 00=/1, 01=/2, 10=/4"]
    #[inline(always)]
    pub fn pll_out_ext_div_p(&self) -> PllOutExtDivPR {
        PllOutExtDivPR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 28 - PLL Lock Status (read-only)"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - PLL Enable"]
    #[inline(always)]
    pub fn pll_en(&self) -> PllEnR {
        PllEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL Factor M (M=Factor+1, range 1-4)"]
    #[inline(always)]
    pub fn pll_factor_m(&mut self) -> PllFactorMW<'_, PllCpuCtrlSpec> {
        PllFactorMW::new(self, 0)
    }
    #[doc = "Bits 4:5 - PLL Factor K (K=Factor+1, range 1-4)"]
    #[inline(always)]
    pub fn pll_factor_k(&mut self) -> PllFactorKW<'_, PllCpuCtrlSpec> {
        PllFactorKW::new(self, 4)
    }
    #[doc = "Bits 8:12 - PLL Factor N (N=Factor+1, range 1-32)"]
    #[inline(always)]
    pub fn pll_factor_n(&mut self) -> PllFactorNW<'_, PllCpuCtrlSpec> {
        PllFactorNW::new(self, 8)
    }
    #[doc = "Bits 16:17 - PLL Output External Divider P: 00=/1, 01=/2, 10=/4"]
    #[inline(always)]
    pub fn pll_out_ext_div_p(&mut self) -> PllOutExtDivPW<'_, PllCpuCtrlSpec> {
        PllOutExtDivPW::new(self, 16)
    }
    #[doc = "Bit 31 - PLL Enable"]
    #[inline(always)]
    pub fn pll_en(&mut self) -> PllEnW<'_, PllCpuCtrlSpec> {
        PllEnW::new(self, 31)
    }
}
#[doc = "PLL CPU Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_cpu_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cpu_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllCpuCtrlSpec;
impl crate::RegisterSpec for PllCpuCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_cpu_ctrl::R`](R) reader structure"]
impl crate::Readable for PllCpuCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_cpu_ctrl::W`](W) writer structure"]
impl crate::Writable for PllCpuCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_CPU_CTRL to value 0x1000"]
impl crate::Resettable for PllCpuCtrlSpec {
    const RESET_VALUE: u32 = 0x1000;
}
