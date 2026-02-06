#[doc = "Register `INTC_PEND_REG0` reader"]
pub type R = crate::R<IntcPendReg0Spec>;
#[doc = "Register `INTC_PEND_REG0` writer"]
pub type W = crate::W<IntcPendReg0Spec>;
#[doc = "Field `INTC_PEND0` reader - Interrupt Source \\[31:0\\] Pending/Clear Bit. 0=Not pending, 1=Pending"]
pub type IntcPend0R = crate::FieldReader<u32>;
#[doc = "Field `INTC_PEND0` writer - Interrupt Source \\[31:0\\] Pending/Clear Bit. 0=Not pending, 1=Pending"]
pub type IntcPend0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt Source \\[31:0\\] Pending/Clear Bit. 0=Not pending, 1=Pending"]
    #[inline(always)]
    pub fn intc_pend0(&self) -> IntcPend0R {
        IntcPend0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Source \\[31:0\\] Pending/Clear Bit. 0=Not pending, 1=Pending"]
    #[inline(always)]
    pub fn intc_pend0(&mut self) -> IntcPend0W<'_, IntcPendReg0Spec> {
        IntcPend0W::new(self, 0)
    }
}
#[doc = "Interrupt IRQ Pending Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_pend_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_pend_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcPendReg0Spec;
impl crate::RegisterSpec for IntcPendReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc_pend_reg0::R`](R) reader structure"]
impl crate::Readable for IntcPendReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`intc_pend_reg0::W`](W) writer structure"]
impl crate::Writable for IntcPendReg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC_PEND_REG0 to value 0"]
impl crate::Resettable for IntcPendReg0Spec {}
