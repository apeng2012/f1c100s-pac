#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_dll: [u8; 0x04],
    _reserved_1_dlh: [u8; 0x04],
    _reserved_2_fcr: [u8; 0x04],
    lcr: Lcr,
    mcr: Mcr,
    lsr: Lsr,
    msr: Msr,
    scr: Scr,
    _reserved8: [u8; 0x5c],
    usr: Usr,
    tfl: Tfl,
    rfl: Rfl,
    _reserved11: [u8; 0x1c],
    htx: Htx,
    _reserved12: [u8; 0x08],
    dbg_dll: DbgDll,
    dbg_dlh: DbgDlh,
}
impl RegisterBlock {
    #[doc = "0x00 - Divisor Latch Low Register (DLAB=1)"]
    #[inline(always)]
    pub const fn dll(&self) -> &Dll {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Transmit Holding Register"]
    #[inline(always)]
    pub const fn thr(&self) -> &Thr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Receive Buffer Register"]
    #[inline(always)]
    pub const fn rbr(&self) -> &Rbr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x04 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Divisor Latch High Register (DLAB=1)"]
    #[inline(always)]
    pub const fn dlh(&self) -> &Dlh {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Interrupt Identity Register"]
    #[inline(always)]
    pub const fn iir(&self) -> &Iir {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Line Control Register"]
    #[inline(always)]
    pub const fn lcr(&self) -> &Lcr {
        &self.lcr
    }
    #[doc = "0x10 - Modem Control Register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
    #[doc = "0x14 - Line Status Register"]
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
    #[doc = "0x18 - Modem Status Register"]
    #[inline(always)]
    pub const fn msr(&self) -> &Msr {
        &self.msr
    }
    #[doc = "0x1c - Scratch Register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0x7c - UART Status Register"]
    #[inline(always)]
    pub const fn usr(&self) -> &Usr {
        &self.usr
    }
    #[doc = "0x80 - Transmit FIFO Level"]
    #[inline(always)]
    pub const fn tfl(&self) -> &Tfl {
        &self.tfl
    }
    #[doc = "0x84 - Receive FIFO Level"]
    #[inline(always)]
    pub const fn rfl(&self) -> &Rfl {
        &self.rfl
    }
    #[doc = "0xa4 - Halt TX Register"]
    #[inline(always)]
    pub const fn htx(&self) -> &Htx {
        &self.htx
    }
    #[doc = "0xb0 - Debug Divisor Latch Low"]
    #[inline(always)]
    pub const fn dbg_dll(&self) -> &DbgDll {
        &self.dbg_dll
    }
    #[doc = "0xb4 - Debug Divisor Latch High"]
    #[inline(always)]
    pub const fn dbg_dlh(&self) -> &DbgDlh {
        &self.dbg_dlh
    }
}
#[doc = "RBR (r) register accessor: Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rbr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbr`] module"]
#[doc(alias = "RBR")]
pub type Rbr = crate::Reg<rbr::RbrSpec>;
#[doc = "Receive Buffer Register"]
pub mod rbr;
#[doc = "THR (w) register accessor: Transmit Holding Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`] module"]
#[doc(alias = "THR")]
pub type Thr = crate::Reg<thr::ThrSpec>;
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "DLL (rw) register accessor: Divisor Latch Low Register (DLAB=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll`] module"]
#[doc(alias = "DLL")]
pub type Dll = crate::Reg<dll::DllSpec>;
#[doc = "Divisor Latch Low Register (DLAB=1)"]
pub mod dll;
#[doc = "DLH (rw) register accessor: Divisor Latch High Register (DLAB=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dlh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlh`] module"]
#[doc(alias = "DLH")]
pub type Dlh = crate::Reg<dlh::DlhSpec>;
#[doc = "Divisor Latch High Register (DLAB=1)"]
pub mod dlh;
#[doc = "IER (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IIR (r) register accessor: Interrupt Identity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iir`] module"]
#[doc(alias = "IIR")]
pub type Iir = crate::Reg<iir::IirSpec>;
#[doc = "Interrupt Identity Register"]
pub mod iir;
#[doc = "FCR (w) register accessor: FIFO Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`] module"]
#[doc(alias = "FCR")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
#[doc = "FIFO Control Register"]
pub mod fcr;
#[doc = "LCR (rw) register accessor: Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr`] module"]
#[doc(alias = "LCR")]
pub type Lcr = crate::Reg<lcr::LcrSpec>;
#[doc = "Line Control Register"]
pub mod lcr;
#[doc = "MCR (rw) register accessor: Modem Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`] module"]
#[doc(alias = "MCR")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "Modem Control Register"]
pub mod mcr;
#[doc = "LSR (r) register accessor: Line Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`] module"]
#[doc(alias = "LSR")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
#[doc = "Line Status Register"]
pub mod lsr;
#[doc = "MSR (r) register accessor: Modem Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`] module"]
#[doc(alias = "MSR")]
pub type Msr = crate::Reg<msr::MsrSpec>;
#[doc = "Modem Status Register"]
pub mod msr;
#[doc = "SCR (rw) register accessor: Scratch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`] module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "Scratch Register"]
pub mod scr;
#[doc = "USR (r) register accessor: UART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usr`] module"]
#[doc(alias = "USR")]
pub type Usr = crate::Reg<usr::UsrSpec>;
#[doc = "UART Status Register"]
pub mod usr;
#[doc = "TFL (r) register accessor: Transmit FIFO Level\n\nYou can [`read`](crate::Reg::read) this register and get [`tfl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfl`] module"]
#[doc(alias = "TFL")]
pub type Tfl = crate::Reg<tfl::TflSpec>;
#[doc = "Transmit FIFO Level"]
pub mod tfl;
#[doc = "RFL (r) register accessor: Receive FIFO Level\n\nYou can [`read`](crate::Reg::read) this register and get [`rfl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfl`] module"]
#[doc(alias = "RFL")]
pub type Rfl = crate::Reg<rfl::RflSpec>;
#[doc = "Receive FIFO Level"]
pub mod rfl;
#[doc = "HTX (rw) register accessor: Halt TX Register\n\nYou can [`read`](crate::Reg::read) this register and get [`htx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@htx`] module"]
#[doc(alias = "HTX")]
pub type Htx = crate::Reg<htx::HtxSpec>;
#[doc = "Halt TX Register"]
pub mod htx;
#[doc = "DBG_DLL (rw) register accessor: Debug Divisor Latch Low\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_dll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_dll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_dll`] module"]
#[doc(alias = "DBG_DLL")]
pub type DbgDll = crate::Reg<dbg_dll::DbgDllSpec>;
#[doc = "Debug Divisor Latch Low"]
pub mod dbg_dll;
#[doc = "DBG_DLH (rw) register accessor: Debug Divisor Latch High\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_dlh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_dlh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg_dlh`] module"]
#[doc(alias = "DBG_DLH")]
pub type DbgDlh = crate::Reg<dbg_dlh::DbgDlhSpec>;
#[doc = "Debug Divisor Latch High"]
pub mod dbg_dlh;
