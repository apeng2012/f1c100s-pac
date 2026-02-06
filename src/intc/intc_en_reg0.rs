#[doc = "Register `INTC_EN_REG0` reader"]
pub type R = crate::R<IntcEnReg0Spec>;
#[doc = "Register `INTC_EN_REG0` writer"]
pub type W = crate::W<IntcEnReg0Spec>;
#[doc = "Field `INTC_EN0` reader - Interrupt Source \\[31:0\\] Enable Bits. 0=Disabled, 1=Enabled"]
pub type IntcEn0R = crate::FieldReader<u32>;
#[doc = "Field `INTC_EN0` writer - Interrupt Source \\[31:0\\] Enable Bits. 0=Disabled, 1=Enabled"]
pub type IntcEn0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt Source \\[31:0\\] Enable Bits. 0=Disabled, 1=Enabled"]
    #[inline(always)]
    pub fn intc_en0(&self) -> IntcEn0R {
        IntcEn0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Source \\[31:0\\] Enable Bits. 0=Disabled, 1=Enabled"]
    #[inline(always)]
    pub fn intc_en0(&mut self) -> IntcEn0W<'_, IntcEnReg0Spec> {
        IntcEn0W::new(self, 0)
    }
}
#[doc = "Interrupt Enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_en_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_en_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcEnReg0Spec;
impl crate::RegisterSpec for IntcEnReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc_en_reg0::R`](R) reader structure"]
impl crate::Readable for IntcEnReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`intc_en_reg0::W`](W) writer structure"]
impl crate::Writable for IntcEnReg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC_EN_REG0 to value 0"]
impl crate::Resettable for IntcEnReg0Spec {}
