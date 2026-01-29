#[doc = "Register `DBG_DLL` reader"]
pub type R = crate::R<DbgDllSpec>;
#[doc = "Register `DBG_DLL` writer"]
pub type W = crate::W<DbgDllSpec>;
#[doc = "Field `DBG_DLL` reader - Debug DLL value"]
pub type DbgDllR = crate::FieldReader;
#[doc = "Field `DBG_DLL` writer - Debug DLL value"]
pub type DbgDllW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Debug DLL value"]
    #[inline(always)]
    pub fn dbg_dll(&self) -> DbgDllR {
        DbgDllR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Debug DLL value"]
    #[inline(always)]
    pub fn dbg_dll(&mut self) -> DbgDllW<'_, DbgDllSpec> {
        DbgDllW::new(self, 0)
    }
}
#[doc = "Debug Divisor Latch Low\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_dll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_dll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgDllSpec;
impl crate::RegisterSpec for DbgDllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_dll::R`](R) reader structure"]
impl crate::Readable for DbgDllSpec {}
#[doc = "`write(|w| ..)` method takes [`dbg_dll::W`](W) writer structure"]
impl crate::Writable for DbgDllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_DLL to value 0"]
impl crate::Resettable for DbgDllSpec {}
