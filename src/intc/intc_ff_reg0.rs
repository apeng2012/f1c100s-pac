#[doc = "Register `INTC_FF_REG0` reader"]
pub type R = crate::R<IntcFfReg0Spec>;
#[doc = "Register `INTC_FF_REG0` writer"]
pub type W = crate::W<IntcFfReg0Spec>;
#[doc = "Field `INTC_FF0` reader - Fast forcing on interrupt source \\[31:0\\]. 0=No effect, 1=Force. Only valid when interrupt enable bit is set."]
pub type IntcFf0R = crate::FieldReader<u32>;
#[doc = "Field `INTC_FF0` writer - Fast forcing on interrupt source \\[31:0\\]. 0=No effect, 1=Force. Only valid when interrupt enable bit is set."]
pub type IntcFf0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Fast forcing on interrupt source \\[31:0\\]. 0=No effect, 1=Force. Only valid when interrupt enable bit is set."]
    #[inline(always)]
    pub fn intc_ff0(&self) -> IntcFf0R {
        IntcFf0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Fast forcing on interrupt source \\[31:0\\]. 0=No effect, 1=Force. Only valid when interrupt enable bit is set."]
    #[inline(always)]
    pub fn intc_ff0(&mut self) -> IntcFf0W<'_, IntcFfReg0Spec> {
        IntcFf0W::new(self, 0)
    }
}
#[doc = "Interrupt Fast Forcing Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_ff_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_ff_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcFfReg0Spec;
impl crate::RegisterSpec for IntcFfReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc_ff_reg0::R`](R) reader structure"]
impl crate::Readable for IntcFfReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`intc_ff_reg0::W`](W) writer structure"]
impl crate::Writable for IntcFfReg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC_FF_REG0 to value 0"]
impl crate::Resettable for IntcFfReg0Spec {}
