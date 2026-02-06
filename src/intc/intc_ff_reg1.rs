#[doc = "Register `INTC_FF_REG1` reader"]
pub type R = crate::R<IntcFfReg1Spec>;
#[doc = "Register `INTC_FF_REG1` writer"]
pub type W = crate::W<IntcFfReg1Spec>;
#[doc = "Field `INTC_FF1` reader - Fast forcing on interrupt source \\[63:32\\]. 0=No effect, 1=Force. Only valid when interrupt enable bit is set."]
pub type IntcFf1R = crate::FieldReader<u32>;
#[doc = "Field `INTC_FF1` writer - Fast forcing on interrupt source \\[63:32\\]. 0=No effect, 1=Force. Only valid when interrupt enable bit is set."]
pub type IntcFf1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Fast forcing on interrupt source \\[63:32\\]. 0=No effect, 1=Force. Only valid when interrupt enable bit is set."]
    #[inline(always)]
    pub fn intc_ff1(&self) -> IntcFf1R {
        IntcFf1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Fast forcing on interrupt source \\[63:32\\]. 0=No effect, 1=Force. Only valid when interrupt enable bit is set."]
    #[inline(always)]
    pub fn intc_ff1(&mut self) -> IntcFf1W<'_, IntcFfReg1Spec> {
        IntcFf1W::new(self, 0)
    }
}
#[doc = "Interrupt Fast Forcing Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_ff_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_ff_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcFfReg1Spec;
impl crate::RegisterSpec for IntcFfReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc_ff_reg1::R`](R) reader structure"]
impl crate::Readable for IntcFfReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`intc_ff_reg1::W`](W) writer structure"]
impl crate::Writable for IntcFfReg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC_FF_REG1 to value 0"]
impl crate::Resettable for IntcFfReg1Spec {}
