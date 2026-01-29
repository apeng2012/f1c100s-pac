#[doc = "Register `DBG_DLH` reader"]
pub type R = crate::R<DbgDlhSpec>;
#[doc = "Register `DBG_DLH` writer"]
pub type W = crate::W<DbgDlhSpec>;
#[doc = "Field `DBG_DLH` reader - Debug DLH value"]
pub type DbgDlhR = crate::FieldReader;
#[doc = "Field `DBG_DLH` writer - Debug DLH value"]
pub type DbgDlhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Debug DLH value"]
    #[inline(always)]
    pub fn dbg_dlh(&self) -> DbgDlhR {
        DbgDlhR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Debug DLH value"]
    #[inline(always)]
    pub fn dbg_dlh(&mut self) -> DbgDlhW<'_, DbgDlhSpec> {
        DbgDlhW::new(self, 0)
    }
}
#[doc = "Debug Divisor Latch High\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_dlh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_dlh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgDlhSpec;
impl crate::RegisterSpec for DbgDlhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_dlh::R`](R) reader structure"]
impl crate::Readable for DbgDlhSpec {}
#[doc = "`write(|w| ..)` method takes [`dbg_dlh::W`](W) writer structure"]
impl crate::Writable for DbgDlhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_DLH to value 0"]
impl crate::Resettable for DbgDlhSpec {}
