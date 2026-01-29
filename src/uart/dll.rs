#[doc = "Register `DLL` reader"]
pub type R = crate::R<DllSpec>;
#[doc = "Register `DLL` writer"]
pub type W = crate::W<DllSpec>;
#[doc = "Field `DLL` reader - Lower 8 bits of baud rate divisor"]
pub type DllR = crate::FieldReader;
#[doc = "Field `DLL` writer - Lower 8 bits of baud rate divisor"]
pub type DllW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Lower 8 bits of baud rate divisor"]
    #[inline(always)]
    pub fn dll(&self) -> DllR {
        DllR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Lower 8 bits of baud rate divisor"]
    #[inline(always)]
    pub fn dll(&mut self) -> DllW<'_, DllSpec> {
        DllW::new(self, 0)
    }
}
#[doc = "Divisor Latch Low Register (DLAB=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DllSpec;
impl crate::RegisterSpec for DllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll::R`](R) reader structure"]
impl crate::Readable for DllSpec {}
#[doc = "`write(|w| ..)` method takes [`dll::W`](W) writer structure"]
impl crate::Writable for DllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DLL to value 0"]
impl crate::Resettable for DllSpec {}
