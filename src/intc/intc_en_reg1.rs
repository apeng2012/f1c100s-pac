#[doc = "Register `INTC_EN_REG1` reader"]
pub type R = crate::R<IntcEnReg1Spec>;
#[doc = "Register `INTC_EN_REG1` writer"]
pub type W = crate::W<IntcEnReg1Spec>;
#[doc = "Field `INTC_EN1` reader - Interrupt Source \\[63:32\\] Enable Bits. 0=Disabled, 1=Enabled"]
pub type IntcEn1R = crate::FieldReader<u32>;
#[doc = "Field `INTC_EN1` writer - Interrupt Source \\[63:32\\] Enable Bits. 0=Disabled, 1=Enabled"]
pub type IntcEn1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt Source \\[63:32\\] Enable Bits. 0=Disabled, 1=Enabled"]
    #[inline(always)]
    pub fn intc_en1(&self) -> IntcEn1R {
        IntcEn1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Source \\[63:32\\] Enable Bits. 0=Disabled, 1=Enabled"]
    #[inline(always)]
    pub fn intc_en1(&mut self) -> IntcEn1W<'_, IntcEnReg1Spec> {
        IntcEn1W::new(self, 0)
    }
}
#[doc = "Interrupt Enable Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_en_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_en_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcEnReg1Spec;
impl crate::RegisterSpec for IntcEnReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc_en_reg1::R`](R) reader structure"]
impl crate::Readable for IntcEnReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`intc_en_reg1::W`](W) writer structure"]
impl crate::Writable for IntcEnReg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC_EN_REG1 to value 0"]
impl crate::Resettable for IntcEnReg1Spec {}
