#[doc = "Register `INTC_PRIO_REG0` reader"]
pub type R = crate::R<IntcPrioReg0Spec>;
#[doc = "Register `INTC_PRIO_REG0` writer"]
pub type W = crate::W<IntcPrioReg0Spec>;
#[doc = "Field `IRQ1_PRIO` reader - IRQ 1 Priority. 0=Lowest, 3=Highest"]
pub type Irq1PrioR = crate::FieldReader;
#[doc = "Field `IRQ1_PRIO` writer - IRQ 1 Priority. 0=Lowest, 3=Highest"]
pub type Irq1PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IRQ2_PRIO` reader - IRQ 2 Priority. 0=Lowest, 3=Highest"]
pub type Irq2PrioR = crate::FieldReader;
#[doc = "Field `IRQ2_PRIO` writer - IRQ 2 Priority. 0=Lowest, 3=Highest"]
pub type Irq2PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IRQ3_PRIO` reader - IRQ 3 Priority. 0=Lowest, 3=Highest"]
pub type Irq3PrioR = crate::FieldReader;
#[doc = "Field `IRQ3_PRIO` writer - IRQ 3 Priority. 0=Lowest, 3=Highest"]
pub type Irq3PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IRQ4_PRIO` reader - IRQ 4 Priority. 0=Lowest, 3=Highest"]
pub type Irq4PrioR = crate::FieldReader;
#[doc = "Field `IRQ4_PRIO` writer - IRQ 4 Priority. 0=Lowest, 3=Highest"]
pub type Irq4PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IRQ5_PRIO` reader - IRQ 5 Priority. 0=Lowest, 3=Highest"]
pub type Irq5PrioR = crate::FieldReader;
#[doc = "Field `IRQ5_PRIO` writer - IRQ 5 Priority. 0=Lowest, 3=Highest"]
pub type Irq5PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IRQ6_PRIO` reader - IRQ 6 Priority. 0=Lowest, 3=Highest"]
pub type Irq6PrioR = crate::FieldReader;
#[doc = "Field `IRQ6_PRIO` writer - IRQ 6 Priority. 0=Lowest, 3=Highest"]
pub type Irq6PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IRQ7_PRIO` reader - IRQ 7 Priority. 0=Lowest, 3=Highest"]
pub type Irq7PrioR = crate::FieldReader;
#[doc = "Field `IRQ7_PRIO` writer - IRQ 7 Priority. 0=Lowest, 3=Highest"]
pub type Irq7PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IRQ8_PRIO` reader - IRQ 8 Priority. 0=Lowest, 3=Highest"]
pub type Irq8PrioR = crate::FieldReader;
#[doc = "Field `IRQ8_PRIO` writer - IRQ 8 Priority. 0=Lowest, 3=Highest"]
pub type Irq8PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IRQ9_PRIO` reader - IRQ 9 Priority. 0=Lowest, 3=Highest"]
pub type Irq9PrioR = crate::FieldReader;
#[doc = "Field `IRQ9_PRIO` writer - IRQ 9 Priority. 0=Lowest, 3=Highest"]
pub type Irq9PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IRQ10_PRIO` reader - IRQ 10 Priority. 0=Lowest, 3=Highest"]
pub type Irq10PrioR = crate::FieldReader;
#[doc = "Field `IRQ10_PRIO` writer - IRQ 10 Priority. 0=Lowest, 3=Highest"]
pub type Irq10PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IRQ11_PRIO` reader - IRQ 11 Priority. 0=Lowest, 3=Highest"]
pub type Irq11PrioR = crate::FieldReader;
#[doc = "Field `IRQ11_PRIO` writer - IRQ 11 Priority. 0=Lowest, 3=Highest"]
pub type Irq11PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IRQ12_PRIO` reader - IRQ 12 Priority. 0=Lowest, 3=Highest"]
pub type Irq12PrioR = crate::FieldReader;
#[doc = "Field `IRQ12_PRIO` writer - IRQ 12 Priority. 0=Lowest, 3=Highest"]
pub type Irq12PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IRQ13_PRIO` reader - IRQ 13 Priority. 0=Lowest, 3=Highest"]
pub type Irq13PrioR = crate::FieldReader;
#[doc = "Field `IRQ13_PRIO` writer - IRQ 13 Priority. 0=Lowest, 3=Highest"]
pub type Irq13PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IRQ14_PRIO` reader - IRQ 14 Priority. 0=Lowest, 3=Highest"]
pub type Irq14PrioR = crate::FieldReader;
#[doc = "Field `IRQ14_PRIO` writer - IRQ 14 Priority. 0=Lowest, 3=Highest"]
pub type Irq14PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IRQ15_PRIO` reader - IRQ 15 Priority. 0=Lowest, 3=Highest"]
pub type Irq15PrioR = crate::FieldReader;
#[doc = "Field `IRQ15_PRIO` writer - IRQ 15 Priority. 0=Lowest, 3=Highest"]
pub type Irq15PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 2:3 - IRQ 1 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq1_prio(&self) -> Irq1PrioR {
        Irq1PrioR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - IRQ 2 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq2_prio(&self) -> Irq2PrioR {
        Irq2PrioR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - IRQ 3 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq3_prio(&self) -> Irq3PrioR {
        Irq3PrioR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - IRQ 4 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq4_prio(&self) -> Irq4PrioR {
        Irq4PrioR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - IRQ 5 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq5_prio(&self) -> Irq5PrioR {
        Irq5PrioR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - IRQ 6 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq6_prio(&self) -> Irq6PrioR {
        Irq6PrioR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - IRQ 7 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq7_prio(&self) -> Irq7PrioR {
        Irq7PrioR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - IRQ 8 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq8_prio(&self) -> Irq8PrioR {
        Irq8PrioR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - IRQ 9 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq9_prio(&self) -> Irq9PrioR {
        Irq9PrioR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - IRQ 10 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq10_prio(&self) -> Irq10PrioR {
        Irq10PrioR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - IRQ 11 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq11_prio(&self) -> Irq11PrioR {
        Irq11PrioR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - IRQ 12 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq12_prio(&self) -> Irq12PrioR {
        Irq12PrioR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - IRQ 13 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq13_prio(&self) -> Irq13PrioR {
        Irq13PrioR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - IRQ 14 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq14_prio(&self) -> Irq14PrioR {
        Irq14PrioR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - IRQ 15 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq15_prio(&self) -> Irq15PrioR {
        Irq15PrioR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - IRQ 1 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq1_prio(&mut self) -> Irq1PrioW<'_, IntcPrioReg0Spec> {
        Irq1PrioW::new(self, 2)
    }
    #[doc = "Bits 4:5 - IRQ 2 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq2_prio(&mut self) -> Irq2PrioW<'_, IntcPrioReg0Spec> {
        Irq2PrioW::new(self, 4)
    }
    #[doc = "Bits 6:7 - IRQ 3 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq3_prio(&mut self) -> Irq3PrioW<'_, IntcPrioReg0Spec> {
        Irq3PrioW::new(self, 6)
    }
    #[doc = "Bits 8:9 - IRQ 4 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq4_prio(&mut self) -> Irq4PrioW<'_, IntcPrioReg0Spec> {
        Irq4PrioW::new(self, 8)
    }
    #[doc = "Bits 10:11 - IRQ 5 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq5_prio(&mut self) -> Irq5PrioW<'_, IntcPrioReg0Spec> {
        Irq5PrioW::new(self, 10)
    }
    #[doc = "Bits 12:13 - IRQ 6 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq6_prio(&mut self) -> Irq6PrioW<'_, IntcPrioReg0Spec> {
        Irq6PrioW::new(self, 12)
    }
    #[doc = "Bits 14:15 - IRQ 7 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq7_prio(&mut self) -> Irq7PrioW<'_, IntcPrioReg0Spec> {
        Irq7PrioW::new(self, 14)
    }
    #[doc = "Bits 16:17 - IRQ 8 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq8_prio(&mut self) -> Irq8PrioW<'_, IntcPrioReg0Spec> {
        Irq8PrioW::new(self, 16)
    }
    #[doc = "Bits 18:19 - IRQ 9 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq9_prio(&mut self) -> Irq9PrioW<'_, IntcPrioReg0Spec> {
        Irq9PrioW::new(self, 18)
    }
    #[doc = "Bits 20:21 - IRQ 10 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq10_prio(&mut self) -> Irq10PrioW<'_, IntcPrioReg0Spec> {
        Irq10PrioW::new(self, 20)
    }
    #[doc = "Bits 22:23 - IRQ 11 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq11_prio(&mut self) -> Irq11PrioW<'_, IntcPrioReg0Spec> {
        Irq11PrioW::new(self, 22)
    }
    #[doc = "Bits 24:25 - IRQ 12 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq12_prio(&mut self) -> Irq12PrioW<'_, IntcPrioReg0Spec> {
        Irq12PrioW::new(self, 24)
    }
    #[doc = "Bits 26:27 - IRQ 13 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq13_prio(&mut self) -> Irq13PrioW<'_, IntcPrioReg0Spec> {
        Irq13PrioW::new(self, 26)
    }
    #[doc = "Bits 28:29 - IRQ 14 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq14_prio(&mut self) -> Irq14PrioW<'_, IntcPrioReg0Spec> {
        Irq14PrioW::new(self, 28)
    }
    #[doc = "Bits 30:31 - IRQ 15 Priority. 0=Lowest, 3=Highest"]
    #[inline(always)]
    pub fn irq15_prio(&mut self) -> Irq15PrioW<'_, IntcPrioReg0Spec> {
        Irq15PrioW::new(self, 30)
    }
}
#[doc = "Interrupt Source Priority Register 0 (IRQ 1-15, 2-bit each, 4 levels)\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_prio_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_prio_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntcPrioReg0Spec;
impl crate::RegisterSpec for IntcPrioReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intc_prio_reg0::R`](R) reader structure"]
impl crate::Readable for IntcPrioReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`intc_prio_reg0::W`](W) writer structure"]
impl crate::Writable for IntcPrioReg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTC_PRIO_REG0 to value 0"]
impl crate::Resettable for IntcPrioReg0Spec {}
