#[doc = "Register `TMR2_CTRL` reader"]
pub type R = crate::R<Tmr2CtrlSpec>;
#[doc = "Register `TMR2_CTRL` writer"]
pub type W = crate::W<Tmr2CtrlSpec>;
#[doc = "Field `TMR2_EN` reader - Timer 2 Enable"]
pub type Tmr2EnR = crate::BitReader;
#[doc = "Field `TMR2_EN` writer - Timer 2 Enable"]
pub type Tmr2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR2_RELOAD` reader - Timer 2 Reload"]
pub type Tmr2ReloadR = crate::BitReader;
#[doc = "Field `TMR2_RELOAD` writer - Timer 2 Reload"]
pub type Tmr2ReloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR2_CLK_SRC` reader - Timer 2 Clock Source"]
pub type Tmr2ClkSrcR = crate::FieldReader;
#[doc = "Field `TMR2_CLK_SRC` writer - Timer 2 Clock Source"]
pub type Tmr2ClkSrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR2_CLK_PRES` reader - Timer 2 Clock Prescale"]
pub type Tmr2ClkPresR = crate::FieldReader;
#[doc = "Field `TMR2_CLK_PRES` writer - Timer 2 Clock Prescale"]
pub type Tmr2ClkPresW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TMR2_MODE` reader - Timer 2 Mode"]
pub type Tmr2ModeR = crate::BitReader;
#[doc = "Field `TMR2_MODE` writer - Timer 2 Mode"]
pub type Tmr2ModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer 2 Enable"]
    #[inline(always)]
    pub fn tmr2_en(&self) -> Tmr2EnR {
        Tmr2EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 2 Reload"]
    #[inline(always)]
    pub fn tmr2_reload(&self) -> Tmr2ReloadR {
        Tmr2ReloadR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Timer 2 Clock Source"]
    #[inline(always)]
    pub fn tmr2_clk_src(&self) -> Tmr2ClkSrcR {
        Tmr2ClkSrcR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Timer 2 Clock Prescale"]
    #[inline(always)]
    pub fn tmr2_clk_pres(&self) -> Tmr2ClkPresR {
        Tmr2ClkPresR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Timer 2 Mode"]
    #[inline(always)]
    pub fn tmr2_mode(&self) -> Tmr2ModeR {
        Tmr2ModeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 2 Enable"]
    #[inline(always)]
    pub fn tmr2_en(&mut self) -> Tmr2EnW<'_, Tmr2CtrlSpec> {
        Tmr2EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer 2 Reload"]
    #[inline(always)]
    pub fn tmr2_reload(&mut self) -> Tmr2ReloadW<'_, Tmr2CtrlSpec> {
        Tmr2ReloadW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Timer 2 Clock Source"]
    #[inline(always)]
    pub fn tmr2_clk_src(&mut self) -> Tmr2ClkSrcW<'_, Tmr2CtrlSpec> {
        Tmr2ClkSrcW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Timer 2 Clock Prescale"]
    #[inline(always)]
    pub fn tmr2_clk_pres(&mut self) -> Tmr2ClkPresW<'_, Tmr2CtrlSpec> {
        Tmr2ClkPresW::new(self, 4)
    }
    #[doc = "Bit 7 - Timer 2 Mode"]
    #[inline(always)]
    pub fn tmr2_mode(&mut self) -> Tmr2ModeW<'_, Tmr2CtrlSpec> {
        Tmr2ModeW::new(self, 7)
    }
}
#[doc = "Timer 2 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr2_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr2_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr2CtrlSpec;
impl crate::RegisterSpec for Tmr2CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr2_ctrl::R`](R) reader structure"]
impl crate::Readable for Tmr2CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr2_ctrl::W`](W) writer structure"]
impl crate::Writable for Tmr2CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMR2_CTRL to value 0x04"]
impl crate::Resettable for Tmr2CtrlSpec {
    const RESET_VALUE: u32 = 0x04;
}
