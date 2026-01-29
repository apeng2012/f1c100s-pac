#[doc = "Register `PF_EINT_DEB` reader"]
pub type R = crate::R<PfEintDebSpec>;
#[doc = "Register `PF_EINT_DEB` writer"]
pub type W = crate::W<PfEintDebSpec>;
#[doc = "Field `DEB_CLK_SEL` reader - Debounce Clock Select (0: LOSC, 1: HOSC)"]
pub type DebClkSelR = crate::BitReader;
#[doc = "Field `DEB_CLK_SEL` writer - Debounce Clock Select (0: LOSC, 1: HOSC)"]
pub type DebClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEB_CLK_PRE` reader - Debounce Clock Prescale"]
pub type DebClkPreR = crate::FieldReader;
#[doc = "Field `DEB_CLK_PRE` writer - Debounce Clock Prescale"]
pub type DebClkPreW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Debounce Clock Select (0: LOSC, 1: HOSC)"]
    #[inline(always)]
    pub fn deb_clk_sel(&self) -> DebClkSelR {
        DebClkSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Debounce Clock Prescale"]
    #[inline(always)]
    pub fn deb_clk_pre(&self) -> DebClkPreR {
        DebClkPreR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Debounce Clock Select (0: LOSC, 1: HOSC)"]
    #[inline(always)]
    pub fn deb_clk_sel(&mut self) -> DebClkSelW<'_, PfEintDebSpec> {
        DebClkSelW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Debounce Clock Prescale"]
    #[inline(always)]
    pub fn deb_clk_pre(&mut self) -> DebClkPreW<'_, PfEintDebSpec> {
        DebClkPreW::new(self, 4)
    }
}
#[doc = "PF External Interrupt Debounce Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_eint_deb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_eint_deb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PfEintDebSpec;
impl crate::RegisterSpec for PfEintDebSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pf_eint_deb::R`](R) reader structure"]
impl crate::Readable for PfEintDebSpec {}
#[doc = "`write(|w| ..)` method takes [`pf_eint_deb::W`](W) writer structure"]
impl crate::Writable for PfEintDebSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PF_EINT_DEB to value 0"]
impl crate::Resettable for PfEintDebSpec {}
