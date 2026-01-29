#[doc = "Register `PLL_PERIPH_CTRL` reader"]
pub type R = crate::R<PllPeriphCtrlSpec>;
#[doc = "Register `PLL_PERIPH_CTRL` writer"]
pub type W = crate::W<PllPeriphCtrlSpec>;
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
#[doc = "Field `PLL_24M_POST_DIV` reader - PLL 24M Output Clock Post Divider (1/2/3/4)"]
pub type Pll24mPostDivR = crate::FieldReader;
#[doc = "Field `PLL_24M_POST_DIV` writer - PLL 24M Output Clock Post Divider (1/2/3/4)"]
pub type Pll24mPostDivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLL_24M_OUT_EN` reader - PLL 24MHz Output Enable"]
pub type Pll24mOutEnR = crate::BitReader;
#[doc = "Field `PLL_24M_OUT_EN` writer - PLL 24MHz Output Enable"]
pub type Pll24mOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - PLL Lock Status"]
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
    #[doc = "Bits 16:17 - PLL 24M Output Clock Post Divider (1/2/3/4)"]
    #[inline(always)]
    pub fn pll_24m_post_div(&self) -> Pll24mPostDivR {
        Pll24mPostDivR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - PLL 24MHz Output Enable"]
    #[inline(always)]
    pub fn pll_24m_out_en(&self) -> Pll24mOutEnR {
        Pll24mOutEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 28 - PLL Lock Status"]
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
    pub fn pll_factor_m(&mut self) -> PllFactorMW<'_, PllPeriphCtrlSpec> {
        PllFactorMW::new(self, 0)
    }
    #[doc = "Bits 4:5 - PLL Factor K (K=Factor+1, range 1-4)"]
    #[inline(always)]
    pub fn pll_factor_k(&mut self) -> PllFactorKW<'_, PllPeriphCtrlSpec> {
        PllFactorKW::new(self, 4)
    }
    #[doc = "Bits 8:12 - PLL Factor N (N=Factor+1, range 1-32)"]
    #[inline(always)]
    pub fn pll_factor_n(&mut self) -> PllFactorNW<'_, PllPeriphCtrlSpec> {
        PllFactorNW::new(self, 8)
    }
    #[doc = "Bits 16:17 - PLL 24M Output Clock Post Divider (1/2/3/4)"]
    #[inline(always)]
    pub fn pll_24m_post_div(&mut self) -> Pll24mPostDivW<'_, PllPeriphCtrlSpec> {
        Pll24mPostDivW::new(self, 16)
    }
    #[doc = "Bit 18 - PLL 24MHz Output Enable"]
    #[inline(always)]
    pub fn pll_24m_out_en(&mut self) -> Pll24mOutEnW<'_, PllPeriphCtrlSpec> {
        Pll24mOutEnW::new(self, 18)
    }
    #[doc = "Bit 31 - PLL Enable"]
    #[inline(always)]
    pub fn pll_en(&mut self) -> PllEnW<'_, PllPeriphCtrlSpec> {
        PllEnW::new(self, 31)
    }
}
#[doc = "PLL Peripheral Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_periph_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_periph_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllPeriphCtrlSpec;
impl crate::RegisterSpec for PllPeriphCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_periph_ctrl::R`](R) reader structure"]
impl crate::Readable for PllPeriphCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_periph_ctrl::W`](W) writer structure"]
impl crate::Writable for PllPeriphCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_PERIPH_CTRL to value 0x0004_1801"]
impl crate::Resettable for PllPeriphCtrlSpec {
    const RESET_VALUE: u32 = 0x0004_1801;
}
