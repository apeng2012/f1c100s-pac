#[doc = "Register `INTC_RESP_REG1` reader"]
pub type R = crate::R<IntcRespReg1Spec>;
#[doc = "Register `INTC_RESP_REG1` writer"]
pub type W = crate::W<IntcRespReg1Spec>;
#[doc = "Field `INTC_RESP1` reader - Interrupt Response Bits \\[63:32\\]. If set, interrupts with lower or same priority are masked."]
pub type IntcResp1R = crate::FieldReader<u32>;
#[doc = "Field `INTC_RESP1` writer - Interrupt Response Bits \\[63:32\\]. If set, interrupts with lower or same priority are masked."]
pub type IntcResp1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt Response Bits \\[63:32\\]. If set, interrupts with lower or same priority are masked."]
    #[inline(always)]
    pub fn intc_resp1(&self) -> IntcResp1R {
        IntcResp1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Response Bits \\[63:32\\]. If set, interrupts with lower or same priority are masked."]
    #[inline(always)]
    pub fn intc_resp1(&mut self) -> IntcResp1W<'_, IntcRespReg1Spec> {
        IntcResp1W::new(self, 0)
    }
}
#[doc = "Interrupt Response Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_resp_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_resp_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcRespReg1Spec;
impl crate::RegisterSpec for IntcRespReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc_resp_reg1::R`](R) reader structure"]
impl crate::Readable for IntcRespReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`intc_resp_reg1::W`](W) writer structure"]
impl crate::Writable for IntcRespReg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC_RESP_REG1 to value 0"]
impl crate::Resettable for IntcRespReg1Spec {}
