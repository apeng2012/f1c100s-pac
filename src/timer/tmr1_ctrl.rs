#[doc = "Register `TMR1_CTRL` reader"]
pub type R = crate::R<Tmr1CtrlSpec>;
#[doc = "Register `TMR1_CTRL` writer"]
pub type W = crate::W<Tmr1CtrlSpec>;
#[doc = "Field `TMR1_EN` reader - Timer 1 Enable"]
pub type Tmr1EnR = crate::BitReader;
#[doc = "Field `TMR1_EN` writer - Timer 1 Enable"]
pub type Tmr1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1_RELOAD` reader - Timer 1 Reload"]
pub type Tmr1ReloadR = crate::BitReader;
#[doc = "Field `TMR1_RELOAD` writer - Timer 1 Reload"]
pub type Tmr1ReloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1_CLK_SRC` reader - Timer 1 Clock Source"]
pub type Tmr1ClkSrcR = crate::FieldReader;
#[doc = "Field `TMR1_CLK_SRC` writer - Timer 1 Clock Source"]
pub type Tmr1ClkSrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR1_CLK_PRES` reader - Timer 1 Clock Prescale"]
pub type Tmr1ClkPresR = crate::FieldReader;
#[doc = "Field `TMR1_CLK_PRES` writer - Timer 1 Clock Prescale"]
pub type Tmr1ClkPresW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TMR1_MODE` reader - Timer 1 Mode"]
pub type Tmr1ModeR = crate::BitReader;
#[doc = "Field `TMR1_MODE` writer - Timer 1 Mode"]
pub type Tmr1ModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer 1 Enable"]
    #[inline(always)]
    pub fn tmr1_en(&self) -> Tmr1EnR {
        Tmr1EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 1 Reload"]
    #[inline(always)]
    pub fn tmr1_reload(&self) -> Tmr1ReloadR {
        Tmr1ReloadR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Timer 1 Clock Source"]
    #[inline(always)]
    pub fn tmr1_clk_src(&self) -> Tmr1ClkSrcR {
        Tmr1ClkSrcR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Timer 1 Clock Prescale"]
    #[inline(always)]
    pub fn tmr1_clk_pres(&self) -> Tmr1ClkPresR {
        Tmr1ClkPresR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Timer 1 Mode"]
    #[inline(always)]
    pub fn tmr1_mode(&self) -> Tmr1ModeR {
        Tmr1ModeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 1 Enable"]
    #[inline(always)]
    pub fn tmr1_en(&mut self) -> Tmr1EnW<'_, Tmr1CtrlSpec> {
        Tmr1EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer 1 Reload"]
    #[inline(always)]
    pub fn tmr1_reload(&mut self) -> Tmr1ReloadW<'_, Tmr1CtrlSpec> {
        Tmr1ReloadW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Timer 1 Clock Source"]
    #[inline(always)]
    pub fn tmr1_clk_src(&mut self) -> Tmr1ClkSrcW<'_, Tmr1CtrlSpec> {
        Tmr1ClkSrcW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Timer 1 Clock Prescale"]
    #[inline(always)]
    pub fn tmr1_clk_pres(&mut self) -> Tmr1ClkPresW<'_, Tmr1CtrlSpec> {
        Tmr1ClkPresW::new(self, 4)
    }
    #[doc = "Bit 7 - Timer 1 Mode"]
    #[inline(always)]
    pub fn tmr1_mode(&mut self) -> Tmr1ModeW<'_, Tmr1CtrlSpec> {
        Tmr1ModeW::new(self, 7)
    }
}
#[doc = "Timer 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr1CtrlSpec;
impl crate::RegisterSpec for Tmr1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr1_ctrl::R`](R) reader structure"]
impl crate::Readable for Tmr1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr1_ctrl::W`](W) writer structure"]
impl crate::Writable for Tmr1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMR1_CTRL to value 0x04"]
impl crate::Resettable for Tmr1CtrlSpec {
    const RESET_VALUE: u32 = 0x04;
}
