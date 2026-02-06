#[doc = "Register `INTC_MASK_REG0` reader"]
pub type R = crate::R<IntcMaskReg0Spec>;
#[doc = "Register `INTC_MASK_REG0` writer"]
pub type W = crate::W<IntcMaskReg0Spec>;
#[doc = "Field `INTC_MASK0` reader - Interrupt Source \\[31:0\\] Mask Bits. 0=No effect, 1=Masked. Pending bit still set even if masked."]
pub type IntcMask0R = crate::FieldReader<u32>;
#[doc = "Field `INTC_MASK0` writer - Interrupt Source \\[31:0\\] Mask Bits. 0=No effect, 1=Masked. Pending bit still set even if masked."]
pub type IntcMask0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt Source \\[31:0\\] Mask Bits. 0=No effect, 1=Masked. Pending bit still set even if masked."]
    #[inline(always)]
    pub fn intc_mask0(&self) -> IntcMask0R {
        IntcMask0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Source \\[31:0\\] Mask Bits. 0=No effect, 1=Masked. Pending bit still set even if masked."]
    #[inline(always)]
    pub fn intc_mask0(&mut self) -> IntcMask0W<'_, IntcMaskReg0Spec> {
        IntcMask0W::new(self, 0)
    }
}
#[doc = "Interrupt Mask Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_mask_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_mask_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcMaskReg0Spec;
impl crate::RegisterSpec for IntcMaskReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc_mask_reg0::R`](R) reader structure"]
impl crate::Readable for IntcMaskReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`intc_mask_reg0::W`](W) writer structure"]
impl crate::Writable for IntcMaskReg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC_MASK_REG0 to value 0"]
impl crate::Resettable for IntcMaskReg0Spec {}
