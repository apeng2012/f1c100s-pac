#[doc = "Register `INTC_MASK_REG1` reader"]
pub type R = crate::R<IntcMaskReg1Spec>;
#[doc = "Register `INTC_MASK_REG1` writer"]
pub type W = crate::W<IntcMaskReg1Spec>;
#[doc = "Field `INTC_MASK1` reader - Interrupt Source \\[63:32\\] Mask Bits. 0=No effect, 1=Masked. Pending bit still set even if masked."]
pub type IntcMask1R = crate::FieldReader<u32>;
#[doc = "Field `INTC_MASK1` writer - Interrupt Source \\[63:32\\] Mask Bits. 0=No effect, 1=Masked. Pending bit still set even if masked."]
pub type IntcMask1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt Source \\[63:32\\] Mask Bits. 0=No effect, 1=Masked. Pending bit still set even if masked."]
    #[inline(always)]
    pub fn intc_mask1(&self) -> IntcMask1R {
        IntcMask1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Source \\[63:32\\] Mask Bits. 0=No effect, 1=Masked. Pending bit still set even if masked."]
    #[inline(always)]
    pub fn intc_mask1(&mut self) -> IntcMask1W<'_, IntcMaskReg1Spec> {
        IntcMask1W::new(self, 0)
    }
}
#[doc = "Interrupt Mask Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_mask_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_mask_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcMaskReg1Spec;
impl crate::RegisterSpec for IntcMaskReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc_mask_reg1::R`](R) reader structure"]
impl crate::Readable for IntcMaskReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`intc_mask_reg1::W`](W) writer structure"]
impl crate::Writable for IntcMaskReg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC_MASK_REG1 to value 0"]
impl crate::Resettable for IntcMaskReg1Spec {}
