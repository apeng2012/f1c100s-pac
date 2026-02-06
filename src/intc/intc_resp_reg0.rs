#[doc = "Register `INTC_RESP_REG0` reader"]
pub type R = crate::R<IntcRespReg0Spec>;
#[doc = "Register `INTC_RESP_REG0` writer"]
pub type W = crate::W<IntcRespReg0Spec>;
#[doc = "Field `INTC_RESP0` reader - Interrupt Response Bits \\[31:0\\]. If set, interrupts with lower or same priority are masked."]
pub type IntcResp0R = crate::FieldReader<u32>;
#[doc = "Field `INTC_RESP0` writer - Interrupt Response Bits \\[31:0\\]. If set, interrupts with lower or same priority are masked."]
pub type IntcResp0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt Response Bits \\[31:0\\]. If set, interrupts with lower or same priority are masked."]
    #[inline(always)]
    pub fn intc_resp0(&self) -> IntcResp0R {
        IntcResp0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Response Bits \\[31:0\\]. If set, interrupts with lower or same priority are masked."]
    #[inline(always)]
    pub fn intc_resp0(&mut self) -> IntcResp0W<'_, IntcRespReg0Spec> {
        IntcResp0W::new(self, 0)
    }
}
#[doc = "Interrupt Response Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_resp_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_resp_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcRespReg0Spec;
impl crate::RegisterSpec for IntcRespReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc_resp_reg0::R`](R) reader structure"]
impl crate::Readable for IntcRespReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`intc_resp_reg0::W`](W) writer structure"]
impl crate::Writable for IntcRespReg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC_RESP_REG0 to value 0"]
impl crate::Resettable for IntcRespReg0Spec {}
