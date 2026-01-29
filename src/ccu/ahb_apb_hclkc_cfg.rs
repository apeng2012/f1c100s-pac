#[doc = "Register `AHB_APB_HCLKC_CFG` reader"]
pub type R = crate::R<AhbApbHclkcCfgSpec>;
#[doc = "Register `AHB_APB_HCLKC_CFG` writer"]
pub type W = crate::W<AhbApbHclkcCfgSpec>;
#[doc = "Field `AHB_CLK_DIV_RATIO` reader - AHB Clock Divide Ratio: 00=/1, 01=/2, 10=/4, 11=/8"]
pub type AhbClkDivRatioR = crate::FieldReader;
#[doc = "Field `AHB_CLK_DIV_RATIO` writer - AHB Clock Divide Ratio: 00=/1, 01=/2, 10=/4, 11=/8"]
pub type AhbClkDivRatioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AHB_PRE_DIV` reader - AHB Clock Pre-divide Ratio: 00=/1, 01=/2, 10=/3, 11=/4"]
pub type AhbPreDivR = crate::FieldReader;
#[doc = "Field `AHB_PRE_DIV` writer - AHB Clock Pre-divide Ratio: 00=/1, 01=/2, 10=/3, 11=/4"]
pub type AhbPreDivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `APB_CLK_RATIO` reader - APB Clock Divide Ratio: 0X=/2, 10=/4, 11=/8"]
pub type ApbClkRatioR = crate::FieldReader;
#[doc = "Field `APB_CLK_RATIO` writer - APB Clock Divide Ratio: 0X=/2, 10=/4, 11=/8"]
pub type ApbClkRatioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AHB_CLK_SRC_SEL` reader - AHB Clock Source: 00=LOSC, 01=OSC24M, 10=CPUCLK, 11=PLL_PERIPH/AHB_PREDIV"]
pub type AhbClkSrcSelR = crate::FieldReader;
#[doc = "Field `AHB_CLK_SRC_SEL` writer - AHB Clock Source: 00=LOSC, 01=OSC24M, 10=CPUCLK, 11=PLL_PERIPH/AHB_PREDIV"]
pub type AhbClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HCLKC_DIV` reader - HCLKC Clock Divide Ratio: 00=/1, 01=/2, 10=/3, 11=/4"]
pub type HclkcDivR = crate::FieldReader;
#[doc = "Field `HCLKC_DIV` writer - HCLKC Clock Divide Ratio: 00=/1, 01=/2, 10=/3, 11=/4"]
pub type HclkcDivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 4:5 - AHB Clock Divide Ratio: 00=/1, 01=/2, 10=/4, 11=/8"]
    #[inline(always)]
    pub fn ahb_clk_div_ratio(&self) -> AhbClkDivRatioR {
        AhbClkDivRatioR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - AHB Clock Pre-divide Ratio: 00=/1, 01=/2, 10=/3, 11=/4"]
    #[inline(always)]
    pub fn ahb_pre_div(&self) -> AhbPreDivR {
        AhbPreDivR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - APB Clock Divide Ratio: 0X=/2, 10=/4, 11=/8"]
    #[inline(always)]
    pub fn apb_clk_ratio(&self) -> ApbClkRatioR {
        ApbClkRatioR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - AHB Clock Source: 00=LOSC, 01=OSC24M, 10=CPUCLK, 11=PLL_PERIPH/AHB_PREDIV"]
    #[inline(always)]
    pub fn ahb_clk_src_sel(&self) -> AhbClkSrcSelR {
        AhbClkSrcSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - HCLKC Clock Divide Ratio: 00=/1, 01=/2, 10=/3, 11=/4"]
    #[inline(always)]
    pub fn hclkc_div(&self) -> HclkcDivR {
        HclkcDivR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - AHB Clock Divide Ratio: 00=/1, 01=/2, 10=/4, 11=/8"]
    #[inline(always)]
    pub fn ahb_clk_div_ratio(&mut self) -> AhbClkDivRatioW<'_, AhbApbHclkcCfgSpec> {
        AhbClkDivRatioW::new(self, 4)
    }
    #[doc = "Bits 6:7 - AHB Clock Pre-divide Ratio: 00=/1, 01=/2, 10=/3, 11=/4"]
    #[inline(always)]
    pub fn ahb_pre_div(&mut self) -> AhbPreDivW<'_, AhbApbHclkcCfgSpec> {
        AhbPreDivW::new(self, 6)
    }
    #[doc = "Bits 8:9 - APB Clock Divide Ratio: 0X=/2, 10=/4, 11=/8"]
    #[inline(always)]
    pub fn apb_clk_ratio(&mut self) -> ApbClkRatioW<'_, AhbApbHclkcCfgSpec> {
        ApbClkRatioW::new(self, 8)
    }
    #[doc = "Bits 12:13 - AHB Clock Source: 00=LOSC, 01=OSC24M, 10=CPUCLK, 11=PLL_PERIPH/AHB_PREDIV"]
    #[inline(always)]
    pub fn ahb_clk_src_sel(&mut self) -> AhbClkSrcSelW<'_, AhbApbHclkcCfgSpec> {
        AhbClkSrcSelW::new(self, 12)
    }
    #[doc = "Bits 16:17 - HCLKC Clock Divide Ratio: 00=/1, 01=/2, 10=/3, 11=/4"]
    #[inline(always)]
    pub fn hclkc_div(&mut self) -> HclkcDivW<'_, AhbApbHclkcCfgSpec> {
        HclkcDivW::new(self, 16)
    }
}
#[doc = "AHB/APB/HCLKC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_apb_hclkc_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_apb_hclkc_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbApbHclkcCfgSpec;
impl crate::RegisterSpec for AhbApbHclkcCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_apb_hclkc_cfg::R`](R) reader structure"]
impl crate::Readable for AhbApbHclkcCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb_apb_hclkc_cfg::W`](W) writer structure"]
impl crate::Writable for AhbApbHclkcCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB_APB_HCLKC_CFG to value 0x0001_1010"]
impl crate::Resettable for AhbApbHclkcCfgSpec {
    const RESET_VALUE: u32 = 0x0001_1010;
}
