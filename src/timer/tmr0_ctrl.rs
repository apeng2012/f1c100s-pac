#[doc = "Register `TMR0_CTRL` reader"]
pub type R = crate::R<Tmr0CtrlSpec>;
#[doc = "Register `TMR0_CTRL` writer"]
pub type W = crate::W<Tmr0CtrlSpec>;
#[doc = "Field `TMR0_EN` reader - Timer 0 Enable"]
pub type Tmr0EnR = crate::BitReader;
#[doc = "Field `TMR0_EN` writer - Timer 0 Enable"]
pub type Tmr0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR0_RELOAD` reader - Timer 0 Reload"]
pub type Tmr0ReloadR = crate::BitReader;
#[doc = "Field `TMR0_RELOAD` writer - Timer 0 Reload"]
pub type Tmr0ReloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR0_CLK_SRC` reader - Timer 0 Clock Source (0: LOSC, 1: OSC24M)"]
pub type Tmr0ClkSrcR = crate::FieldReader;
#[doc = "Field `TMR0_CLK_SRC` writer - Timer 0 Clock Source (0: LOSC, 1: OSC24M)"]
pub type Tmr0ClkSrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR0_CLK_PRES` reader - Timer 0 Clock Prescale"]
pub type Tmr0ClkPresR = crate::FieldReader;
#[doc = "Field `TMR0_CLK_PRES` writer - Timer 0 Clock Prescale"]
pub type Tmr0ClkPresW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TMR0_MODE` reader - Timer 0 Mode (0: Continuous, 1: Single)"]
pub type Tmr0ModeR = crate::BitReader;
#[doc = "Field `TMR0_MODE` writer - Timer 0 Mode (0: Continuous, 1: Single)"]
pub type Tmr0ModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer 0 Enable"]
    #[inline(always)]
    pub fn tmr0_en(&self) -> Tmr0EnR {
        Tmr0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 0 Reload"]
    #[inline(always)]
    pub fn tmr0_reload(&self) -> Tmr0ReloadR {
        Tmr0ReloadR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Timer 0 Clock Source (0: LOSC, 1: OSC24M)"]
    #[inline(always)]
    pub fn tmr0_clk_src(&self) -> Tmr0ClkSrcR {
        Tmr0ClkSrcR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Timer 0 Clock Prescale"]
    #[inline(always)]
    pub fn tmr0_clk_pres(&self) -> Tmr0ClkPresR {
        Tmr0ClkPresR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Timer 0 Mode (0: Continuous, 1: Single)"]
    #[inline(always)]
    pub fn tmr0_mode(&self) -> Tmr0ModeR {
        Tmr0ModeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer 0 Enable"]
    #[inline(always)]
    pub fn tmr0_en(&mut self) -> Tmr0EnW<'_, Tmr0CtrlSpec> {
        Tmr0EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer 0 Reload"]
    #[inline(always)]
    pub fn tmr0_reload(&mut self) -> Tmr0ReloadW<'_, Tmr0CtrlSpec> {
        Tmr0ReloadW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Timer 0 Clock Source (0: LOSC, 1: OSC24M)"]
    #[inline(always)]
    pub fn tmr0_clk_src(&mut self) -> Tmr0ClkSrcW<'_, Tmr0CtrlSpec> {
        Tmr0ClkSrcW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Timer 0 Clock Prescale"]
    #[inline(always)]
    pub fn tmr0_clk_pres(&mut self) -> Tmr0ClkPresW<'_, Tmr0CtrlSpec> {
        Tmr0ClkPresW::new(self, 4)
    }
    #[doc = "Bit 7 - Timer 0 Mode (0: Continuous, 1: Single)"]
    #[inline(always)]
    pub fn tmr0_mode(&mut self) -> Tmr0ModeW<'_, Tmr0CtrlSpec> {
        Tmr0ModeW::new(self, 7)
    }
}
#[doc = "Timer 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmr0CtrlSpec;
impl crate::RegisterSpec for Tmr0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr0_ctrl::R`](R) reader structure"]
impl crate::Readable for Tmr0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr0_ctrl::W`](W) writer structure"]
impl crate::Writable for Tmr0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMR0_CTRL to value 0x04"]
impl crate::Resettable for Tmr0CtrlSpec {
    const RESET_VALUE: u32 = 0x04;
}
