#[doc = "Register `INTC_PEND_REG1` reader"]
pub type R = crate::R<IntcPendReg1Spec>;
#[doc = "Register `INTC_PEND_REG1` writer"]
pub type W = crate::W<IntcPendReg1Spec>;
#[doc = "Field `INTC_PEND1` reader - Interrupt Source \\[63:32\\] Pending/Clear Bit. 0=Not pending, 1=Pending"]
pub type IntcPend1R = crate::FieldReader<u32>;
#[doc = "Field `INTC_PEND1` writer - Interrupt Source \\[63:32\\] Pending/Clear Bit. 0=Not pending, 1=Pending"]
pub type IntcPend1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt Source \\[63:32\\] Pending/Clear Bit. 0=Not pending, 1=Pending"]
    #[inline(always)]
    pub fn intc_pend1(&self) -> IntcPend1R {
        IntcPend1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Source \\[63:32\\] Pending/Clear Bit. 0=Not pending, 1=Pending"]
    #[inline(always)]
    pub fn intc_pend1(&mut self) -> IntcPend1W<'_, IntcPendReg1Spec> {
        IntcPend1W::new(self, 0)
    }
}
#[doc = "Interrupt IRQ Pending Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_pend_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_pend_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcPendReg1Spec;
impl crate::RegisterSpec for IntcPendReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc_pend_reg1::R`](R) reader structure"]
impl crate::Readable for IntcPendReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`intc_pend_reg1::W`](W) writer structure"]
impl crate::Writable for IntcPendReg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC_PEND_REG1 to value 0"]
impl crate::Resettable for IntcPendReg1Spec {}
