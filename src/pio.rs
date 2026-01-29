#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pa_cfg0: PaCfg0,
    pa_cfg1: PaCfg1,
    pa_cfg2: PaCfg2,
    pa_cfg3: PaCfg3,
    pa_data: PaData,
    pa_drv0: PaDrv0,
    pa_drv1: PaDrv1,
    pa_pull0: PaPull0,
    pa_pull1: PaPull1,
    pb_cfg0: PbCfg0,
    pb_cfg1: PbCfg1,
    pb_cfg2: PbCfg2,
    pb_cfg3: PbCfg3,
    pb_data: PbData,
    pb_drv0: PbDrv0,
    pb_drv1: PbDrv1,
    pb_pull0: PbPull0,
    pb_pull1: PbPull1,
    pc_cfg0: PcCfg0,
    pc_cfg1: PcCfg1,
    pc_cfg2: PcCfg2,
    pc_cfg3: PcCfg3,
    pc_data: PcData,
    pc_drv0: PcDrv0,
    pc_drv1: PcDrv1,
    pc_pull0: PcPull0,
    pc_pull1: PcPull1,
    pd_cfg0: PdCfg0,
    pd_cfg1: PdCfg1,
    pd_cfg2: PdCfg2,
    pd_cfg3: PdCfg3,
    pd_data: PdData,
    pd_drv0: PdDrv0,
    pd_drv1: PdDrv1,
    pd_pull0: PdPull0,
    pd_pull1: PdPull1,
    pe_cfg0: PeCfg0,
    pe_cfg1: PeCfg1,
    pe_cfg2: PeCfg2,
    pe_cfg3: PeCfg3,
    pe_data: PeData,
    pe_drv0: PeDrv0,
    pe_drv1: PeDrv1,
    pe_pull0: PePull0,
    pe_pull1: PePull1,
    pf_cfg0: PfCfg0,
    pf_cfg1: PfCfg1,
    pf_cfg2: PfCfg2,
    pf_cfg3: PfCfg3,
    pf_data: PfData,
    pf_drv0: PfDrv0,
    pf_drv1: PfDrv1,
    pf_pull0: PfPull0,
    pf_pull1: PfPull1,
    _reserved54: [u8; 0x0128],
    pd_eint_cfg0: PdEintCfg0,
    pd_eint_cfg1: PdEintCfg1,
    pd_eint_cfg2: PdEintCfg2,
    pd_eint_cfg3: PdEintCfg3,
    pd_eint_ctl: PdEintCtl,
    pd_eint_sta: PdEintSta,
    pd_eint_deb: PdEintDeb,
    _reserved61: [u8; 0x04],
    pe_eint_cfg0: PeEintCfg0,
    pe_eint_cfg1: PeEintCfg1,
    pe_eint_cfg2: PeEintCfg2,
    pe_eint_cfg3: PeEintCfg3,
    pe_eint_ctl: PeEintCtl,
    pe_eint_sta: PeEintSta,
    pe_eint_deb: PeEintDeb,
    _reserved68: [u8; 0x04],
    pf_eint_cfg0: PfEintCfg0,
    pf_eint_cfg1: PfEintCfg1,
    pf_eint_cfg2: PfEintCfg2,
    pf_eint_cfg3: PfEintCfg3,
    pf_eint_ctl: PfEintCtl,
    pf_eint_sta: PfEintSta,
    pf_eint_deb: PfEintDeb,
    _reserved75: [u8; 0x64],
    sdr_pad_drv: SdrPadDrv,
    sdr_pad_pull: SdrPadPull,
}
impl RegisterBlock {
    #[doc = "0x00 - PA Configure Register 0"]
    #[inline(always)]
    pub const fn pa_cfg0(&self) -> &PaCfg0 {
        &self.pa_cfg0
    }
    #[doc = "0x04 - PA Configure Register 1"]
    #[inline(always)]
    pub const fn pa_cfg1(&self) -> &PaCfg1 {
        &self.pa_cfg1
    }
    #[doc = "0x08 - PA Configure Register 2"]
    #[inline(always)]
    pub const fn pa_cfg2(&self) -> &PaCfg2 {
        &self.pa_cfg2
    }
    #[doc = "0x0c - PA Configure Register 3"]
    #[inline(always)]
    pub const fn pa_cfg3(&self) -> &PaCfg3 {
        &self.pa_cfg3
    }
    #[doc = "0x10 - PA Data Register"]
    #[inline(always)]
    pub const fn pa_data(&self) -> &PaData {
        &self.pa_data
    }
    #[doc = "0x14 - PA Multi-Driving Register 0"]
    #[inline(always)]
    pub const fn pa_drv0(&self) -> &PaDrv0 {
        &self.pa_drv0
    }
    #[doc = "0x18 - PA Multi-Driving Register 1"]
    #[inline(always)]
    pub const fn pa_drv1(&self) -> &PaDrv1 {
        &self.pa_drv1
    }
    #[doc = "0x1c - PA Pull Register 0"]
    #[inline(always)]
    pub const fn pa_pull0(&self) -> &PaPull0 {
        &self.pa_pull0
    }
    #[doc = "0x20 - PA Pull Register 1"]
    #[inline(always)]
    pub const fn pa_pull1(&self) -> &PaPull1 {
        &self.pa_pull1
    }
    #[doc = "0x24 - PB Configure Register 0"]
    #[inline(always)]
    pub const fn pb_cfg0(&self) -> &PbCfg0 {
        &self.pb_cfg0
    }
    #[doc = "0x28 - PB Configure Register 1"]
    #[inline(always)]
    pub const fn pb_cfg1(&self) -> &PbCfg1 {
        &self.pb_cfg1
    }
    #[doc = "0x2c - PB Configure Register 2"]
    #[inline(always)]
    pub const fn pb_cfg2(&self) -> &PbCfg2 {
        &self.pb_cfg2
    }
    #[doc = "0x30 - PB Configure Register 3"]
    #[inline(always)]
    pub const fn pb_cfg3(&self) -> &PbCfg3 {
        &self.pb_cfg3
    }
    #[doc = "0x34 - PB Data Register"]
    #[inline(always)]
    pub const fn pb_data(&self) -> &PbData {
        &self.pb_data
    }
    #[doc = "0x38 - PB Multi-Driving Register 0"]
    #[inline(always)]
    pub const fn pb_drv0(&self) -> &PbDrv0 {
        &self.pb_drv0
    }
    #[doc = "0x3c - PB Multi-Driving Register 1"]
    #[inline(always)]
    pub const fn pb_drv1(&self) -> &PbDrv1 {
        &self.pb_drv1
    }
    #[doc = "0x40 - PB Pull Register 0"]
    #[inline(always)]
    pub const fn pb_pull0(&self) -> &PbPull0 {
        &self.pb_pull0
    }
    #[doc = "0x44 - PB Pull Register 1"]
    #[inline(always)]
    pub const fn pb_pull1(&self) -> &PbPull1 {
        &self.pb_pull1
    }
    #[doc = "0x48 - PC Configure Register 0"]
    #[inline(always)]
    pub const fn pc_cfg0(&self) -> &PcCfg0 {
        &self.pc_cfg0
    }
    #[doc = "0x4c - PC Configure Register 1"]
    #[inline(always)]
    pub const fn pc_cfg1(&self) -> &PcCfg1 {
        &self.pc_cfg1
    }
    #[doc = "0x50 - PC Configure Register 2"]
    #[inline(always)]
    pub const fn pc_cfg2(&self) -> &PcCfg2 {
        &self.pc_cfg2
    }
    #[doc = "0x54 - PC Configure Register 3"]
    #[inline(always)]
    pub const fn pc_cfg3(&self) -> &PcCfg3 {
        &self.pc_cfg3
    }
    #[doc = "0x58 - PC Data Register"]
    #[inline(always)]
    pub const fn pc_data(&self) -> &PcData {
        &self.pc_data
    }
    #[doc = "0x5c - PC Multi-Driving Register 0"]
    #[inline(always)]
    pub const fn pc_drv0(&self) -> &PcDrv0 {
        &self.pc_drv0
    }
    #[doc = "0x60 - PC Multi-Driving Register 1"]
    #[inline(always)]
    pub const fn pc_drv1(&self) -> &PcDrv1 {
        &self.pc_drv1
    }
    #[doc = "0x64 - PC Pull Register 0"]
    #[inline(always)]
    pub const fn pc_pull0(&self) -> &PcPull0 {
        &self.pc_pull0
    }
    #[doc = "0x68 - PC Pull Register 1"]
    #[inline(always)]
    pub const fn pc_pull1(&self) -> &PcPull1 {
        &self.pc_pull1
    }
    #[doc = "0x6c - PD Configure Register 0"]
    #[inline(always)]
    pub const fn pd_cfg0(&self) -> &PdCfg0 {
        &self.pd_cfg0
    }
    #[doc = "0x70 - PD Configure Register 1"]
    #[inline(always)]
    pub const fn pd_cfg1(&self) -> &PdCfg1 {
        &self.pd_cfg1
    }
    #[doc = "0x74 - PD Configure Register 2"]
    #[inline(always)]
    pub const fn pd_cfg2(&self) -> &PdCfg2 {
        &self.pd_cfg2
    }
    #[doc = "0x78 - PD Configure Register 3"]
    #[inline(always)]
    pub const fn pd_cfg3(&self) -> &PdCfg3 {
        &self.pd_cfg3
    }
    #[doc = "0x7c - PD Data Register"]
    #[inline(always)]
    pub const fn pd_data(&self) -> &PdData {
        &self.pd_data
    }
    #[doc = "0x80 - PD Multi-Driving Register 0"]
    #[inline(always)]
    pub const fn pd_drv0(&self) -> &PdDrv0 {
        &self.pd_drv0
    }
    #[doc = "0x84 - PD Multi-Driving Register 1"]
    #[inline(always)]
    pub const fn pd_drv1(&self) -> &PdDrv1 {
        &self.pd_drv1
    }
    #[doc = "0x88 - PD Pull Register 0"]
    #[inline(always)]
    pub const fn pd_pull0(&self) -> &PdPull0 {
        &self.pd_pull0
    }
    #[doc = "0x8c - PD Pull Register 1"]
    #[inline(always)]
    pub const fn pd_pull1(&self) -> &PdPull1 {
        &self.pd_pull1
    }
    #[doc = "0x90 - PE Configure Register 0"]
    #[inline(always)]
    pub const fn pe_cfg0(&self) -> &PeCfg0 {
        &self.pe_cfg0
    }
    #[doc = "0x94 - PE Configure Register 1"]
    #[inline(always)]
    pub const fn pe_cfg1(&self) -> &PeCfg1 {
        &self.pe_cfg1
    }
    #[doc = "0x98 - PE Configure Register 2"]
    #[inline(always)]
    pub const fn pe_cfg2(&self) -> &PeCfg2 {
        &self.pe_cfg2
    }
    #[doc = "0x9c - PE Configure Register 3"]
    #[inline(always)]
    pub const fn pe_cfg3(&self) -> &PeCfg3 {
        &self.pe_cfg3
    }
    #[doc = "0xa0 - PE Data Register"]
    #[inline(always)]
    pub const fn pe_data(&self) -> &PeData {
        &self.pe_data
    }
    #[doc = "0xa4 - PE Multi-Driving Register 0"]
    #[inline(always)]
    pub const fn pe_drv0(&self) -> &PeDrv0 {
        &self.pe_drv0
    }
    #[doc = "0xa8 - PE Multi-Driving Register 1"]
    #[inline(always)]
    pub const fn pe_drv1(&self) -> &PeDrv1 {
        &self.pe_drv1
    }
    #[doc = "0xac - PE Pull Register 0"]
    #[inline(always)]
    pub const fn pe_pull0(&self) -> &PePull0 {
        &self.pe_pull0
    }
    #[doc = "0xb0 - PE Pull Register 1"]
    #[inline(always)]
    pub const fn pe_pull1(&self) -> &PePull1 {
        &self.pe_pull1
    }
    #[doc = "0xb4 - PF Configure Register 0"]
    #[inline(always)]
    pub const fn pf_cfg0(&self) -> &PfCfg0 {
        &self.pf_cfg0
    }
    #[doc = "0xb8 - PF Configure Register 1"]
    #[inline(always)]
    pub const fn pf_cfg1(&self) -> &PfCfg1 {
        &self.pf_cfg1
    }
    #[doc = "0xbc - PF Configure Register 2"]
    #[inline(always)]
    pub const fn pf_cfg2(&self) -> &PfCfg2 {
        &self.pf_cfg2
    }
    #[doc = "0xc0 - PF Configure Register 3"]
    #[inline(always)]
    pub const fn pf_cfg3(&self) -> &PfCfg3 {
        &self.pf_cfg3
    }
    #[doc = "0xc4 - PF Data Register"]
    #[inline(always)]
    pub const fn pf_data(&self) -> &PfData {
        &self.pf_data
    }
    #[doc = "0xc8 - PF Multi-Driving Register 0"]
    #[inline(always)]
    pub const fn pf_drv0(&self) -> &PfDrv0 {
        &self.pf_drv0
    }
    #[doc = "0xcc - PF Multi-Driving Register 1"]
    #[inline(always)]
    pub const fn pf_drv1(&self) -> &PfDrv1 {
        &self.pf_drv1
    }
    #[doc = "0xd0 - PF Pull Register 0"]
    #[inline(always)]
    pub const fn pf_pull0(&self) -> &PfPull0 {
        &self.pf_pull0
    }
    #[doc = "0xd4 - PF Pull Register 1"]
    #[inline(always)]
    pub const fn pf_pull1(&self) -> &PfPull1 {
        &self.pf_pull1
    }
    #[doc = "0x200 - PD External Interrupt Configure Register 0"]
    #[inline(always)]
    pub const fn pd_eint_cfg0(&self) -> &PdEintCfg0 {
        &self.pd_eint_cfg0
    }
    #[doc = "0x204 - PD External Interrupt Configure Register 1"]
    #[inline(always)]
    pub const fn pd_eint_cfg1(&self) -> &PdEintCfg1 {
        &self.pd_eint_cfg1
    }
    #[doc = "0x208 - PD External Interrupt Configure Register 2"]
    #[inline(always)]
    pub const fn pd_eint_cfg2(&self) -> &PdEintCfg2 {
        &self.pd_eint_cfg2
    }
    #[doc = "0x20c - PD External Interrupt Configure Register 3"]
    #[inline(always)]
    pub const fn pd_eint_cfg3(&self) -> &PdEintCfg3 {
        &self.pd_eint_cfg3
    }
    #[doc = "0x210 - PD External Interrupt Control Register"]
    #[inline(always)]
    pub const fn pd_eint_ctl(&self) -> &PdEintCtl {
        &self.pd_eint_ctl
    }
    #[doc = "0x214 - PD External Interrupt Status Register"]
    #[inline(always)]
    pub const fn pd_eint_sta(&self) -> &PdEintSta {
        &self.pd_eint_sta
    }
    #[doc = "0x218 - PD External Interrupt Debounce Register"]
    #[inline(always)]
    pub const fn pd_eint_deb(&self) -> &PdEintDeb {
        &self.pd_eint_deb
    }
    #[doc = "0x220 - PE External Interrupt Configure Register 0"]
    #[inline(always)]
    pub const fn pe_eint_cfg0(&self) -> &PeEintCfg0 {
        &self.pe_eint_cfg0
    }
    #[doc = "0x224 - PE External Interrupt Configure Register 1"]
    #[inline(always)]
    pub const fn pe_eint_cfg1(&self) -> &PeEintCfg1 {
        &self.pe_eint_cfg1
    }
    #[doc = "0x228 - PE External Interrupt Configure Register 2"]
    #[inline(always)]
    pub const fn pe_eint_cfg2(&self) -> &PeEintCfg2 {
        &self.pe_eint_cfg2
    }
    #[doc = "0x22c - PE External Interrupt Configure Register 3"]
    #[inline(always)]
    pub const fn pe_eint_cfg3(&self) -> &PeEintCfg3 {
        &self.pe_eint_cfg3
    }
    #[doc = "0x230 - PE External Interrupt Control Register"]
    #[inline(always)]
    pub const fn pe_eint_ctl(&self) -> &PeEintCtl {
        &self.pe_eint_ctl
    }
    #[doc = "0x234 - PE External Interrupt Status Register"]
    #[inline(always)]
    pub const fn pe_eint_sta(&self) -> &PeEintSta {
        &self.pe_eint_sta
    }
    #[doc = "0x238 - PE External Interrupt Debounce Register"]
    #[inline(always)]
    pub const fn pe_eint_deb(&self) -> &PeEintDeb {
        &self.pe_eint_deb
    }
    #[doc = "0x240 - PF External Interrupt Configure Register 0"]
    #[inline(always)]
    pub const fn pf_eint_cfg0(&self) -> &PfEintCfg0 {
        &self.pf_eint_cfg0
    }
    #[doc = "0x244 - PF External Interrupt Configure Register 1"]
    #[inline(always)]
    pub const fn pf_eint_cfg1(&self) -> &PfEintCfg1 {
        &self.pf_eint_cfg1
    }
    #[doc = "0x248 - PF External Interrupt Configure Register 2"]
    #[inline(always)]
    pub const fn pf_eint_cfg2(&self) -> &PfEintCfg2 {
        &self.pf_eint_cfg2
    }
    #[doc = "0x24c - PF External Interrupt Configure Register 3"]
    #[inline(always)]
    pub const fn pf_eint_cfg3(&self) -> &PfEintCfg3 {
        &self.pf_eint_cfg3
    }
    #[doc = "0x250 - PF External Interrupt Control Register"]
    #[inline(always)]
    pub const fn pf_eint_ctl(&self) -> &PfEintCtl {
        &self.pf_eint_ctl
    }
    #[doc = "0x254 - PF External Interrupt Status Register"]
    #[inline(always)]
    pub const fn pf_eint_sta(&self) -> &PfEintSta {
        &self.pf_eint_sta
    }
    #[doc = "0x258 - PF External Interrupt Debounce Register"]
    #[inline(always)]
    pub const fn pf_eint_deb(&self) -> &PfEintDeb {
        &self.pf_eint_deb
    }
    #[doc = "0x2c0 - SDRAM Pad Multi-Driving Register"]
    #[inline(always)]
    pub const fn sdr_pad_drv(&self) -> &SdrPadDrv {
        &self.sdr_pad_drv
    }
    #[doc = "0x2c4 - SDRAM Pad Pull Register"]
    #[inline(always)]
    pub const fn sdr_pad_pull(&self) -> &SdrPadPull {
        &self.sdr_pad_pull
    }
}
#[doc = "PA_CFG0 (rw) register accessor: PA Configure Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_cfg0`] module"]
#[doc(alias = "PA_CFG0")]
pub type PaCfg0 = crate::Reg<pa_cfg0::PaCfg0Spec>;
#[doc = "PA Configure Register 0"]
pub mod pa_cfg0;
#[doc = "PA_CFG1 (rw) register accessor: PA Configure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_cfg1`] module"]
#[doc(alias = "PA_CFG1")]
pub type PaCfg1 = crate::Reg<pa_cfg1::PaCfg1Spec>;
#[doc = "PA Configure Register 1"]
pub mod pa_cfg1;
#[doc = "PA_CFG2 (rw) register accessor: PA Configure Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_cfg2`] module"]
#[doc(alias = "PA_CFG2")]
pub type PaCfg2 = crate::Reg<pa_cfg2::PaCfg2Spec>;
#[doc = "PA Configure Register 2"]
pub mod pa_cfg2;
#[doc = "PA_CFG3 (rw) register accessor: PA Configure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_cfg3`] module"]
#[doc(alias = "PA_CFG3")]
pub type PaCfg3 = crate::Reg<pa_cfg3::PaCfg3Spec>;
#[doc = "PA Configure Register 3"]
pub mod pa_cfg3;
#[doc = "PA_DATA (rw) register accessor: PA Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_data`] module"]
#[doc(alias = "PA_DATA")]
pub type PaData = crate::Reg<pa_data::PaDataSpec>;
#[doc = "PA Data Register"]
pub mod pa_data;
#[doc = "PA_DRV0 (rw) register accessor: PA Multi-Driving Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_drv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_drv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_drv0`] module"]
#[doc(alias = "PA_DRV0")]
pub type PaDrv0 = crate::Reg<pa_drv0::PaDrv0Spec>;
#[doc = "PA Multi-Driving Register 0"]
pub mod pa_drv0;
#[doc = "PA_DRV1 (rw) register accessor: PA Multi-Driving Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_drv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_drv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_drv1`] module"]
#[doc(alias = "PA_DRV1")]
pub type PaDrv1 = crate::Reg<pa_drv1::PaDrv1Spec>;
#[doc = "PA Multi-Driving Register 1"]
pub mod pa_drv1;
#[doc = "PA_PULL0 (rw) register accessor: PA Pull Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_pull0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_pull0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_pull0`] module"]
#[doc(alias = "PA_PULL0")]
pub type PaPull0 = crate::Reg<pa_pull0::PaPull0Spec>;
#[doc = "PA Pull Register 0"]
pub mod pa_pull0;
#[doc = "PA_PULL1 (rw) register accessor: PA Pull Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pa_pull1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa_pull1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_pull1`] module"]
#[doc(alias = "PA_PULL1")]
pub type PaPull1 = crate::Reg<pa_pull1::PaPull1Spec>;
#[doc = "PA Pull Register 1"]
pub mod pa_pull1;
#[doc = "PB_CFG0 (rw) register accessor: PB Configure Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_cfg0`] module"]
#[doc(alias = "PB_CFG0")]
pub type PbCfg0 = crate::Reg<pb_cfg0::PbCfg0Spec>;
#[doc = "PB Configure Register 0"]
pub mod pb_cfg0;
#[doc = "PB_CFG1 (rw) register accessor: PB Configure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_cfg1`] module"]
#[doc(alias = "PB_CFG1")]
pub type PbCfg1 = crate::Reg<pb_cfg1::PbCfg1Spec>;
#[doc = "PB Configure Register 1"]
pub mod pb_cfg1;
#[doc = "PB_CFG2 (rw) register accessor: PB Configure Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_cfg2`] module"]
#[doc(alias = "PB_CFG2")]
pub type PbCfg2 = crate::Reg<pb_cfg2::PbCfg2Spec>;
#[doc = "PB Configure Register 2"]
pub mod pb_cfg2;
#[doc = "PB_CFG3 (rw) register accessor: PB Configure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_cfg3`] module"]
#[doc(alias = "PB_CFG3")]
pub type PbCfg3 = crate::Reg<pb_cfg3::PbCfg3Spec>;
#[doc = "PB Configure Register 3"]
pub mod pb_cfg3;
#[doc = "PB_DATA (rw) register accessor: PB Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_data`] module"]
#[doc(alias = "PB_DATA")]
pub type PbData = crate::Reg<pb_data::PbDataSpec>;
#[doc = "PB Data Register"]
pub mod pb_data;
#[doc = "PB_DRV0 (rw) register accessor: PB Multi-Driving Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_drv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_drv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_drv0`] module"]
#[doc(alias = "PB_DRV0")]
pub type PbDrv0 = crate::Reg<pb_drv0::PbDrv0Spec>;
#[doc = "PB Multi-Driving Register 0"]
pub mod pb_drv0;
#[doc = "PB_DRV1 (rw) register accessor: PB Multi-Driving Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_drv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_drv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_drv1`] module"]
#[doc(alias = "PB_DRV1")]
pub type PbDrv1 = crate::Reg<pb_drv1::PbDrv1Spec>;
#[doc = "PB Multi-Driving Register 1"]
pub mod pb_drv1;
#[doc = "PB_PULL0 (rw) register accessor: PB Pull Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_pull0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_pull0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_pull0`] module"]
#[doc(alias = "PB_PULL0")]
pub type PbPull0 = crate::Reg<pb_pull0::PbPull0Spec>;
#[doc = "PB Pull Register 0"]
pub mod pb_pull0;
#[doc = "PB_PULL1 (rw) register accessor: PB Pull Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pb_pull1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb_pull1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pb_pull1`] module"]
#[doc(alias = "PB_PULL1")]
pub type PbPull1 = crate::Reg<pb_pull1::PbPull1Spec>;
#[doc = "PB Pull Register 1"]
pub mod pb_pull1;
#[doc = "PC_CFG0 (rw) register accessor: PC Configure Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_cfg0`] module"]
#[doc(alias = "PC_CFG0")]
pub type PcCfg0 = crate::Reg<pc_cfg0::PcCfg0Spec>;
#[doc = "PC Configure Register 0"]
pub mod pc_cfg0;
#[doc = "PC_CFG1 (rw) register accessor: PC Configure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_cfg1`] module"]
#[doc(alias = "PC_CFG1")]
pub type PcCfg1 = crate::Reg<pc_cfg1::PcCfg1Spec>;
#[doc = "PC Configure Register 1"]
pub mod pc_cfg1;
#[doc = "PC_CFG2 (rw) register accessor: PC Configure Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_cfg2`] module"]
#[doc(alias = "PC_CFG2")]
pub type PcCfg2 = crate::Reg<pc_cfg2::PcCfg2Spec>;
#[doc = "PC Configure Register 2"]
pub mod pc_cfg2;
#[doc = "PC_CFG3 (rw) register accessor: PC Configure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_cfg3`] module"]
#[doc(alias = "PC_CFG3")]
pub type PcCfg3 = crate::Reg<pc_cfg3::PcCfg3Spec>;
#[doc = "PC Configure Register 3"]
pub mod pc_cfg3;
#[doc = "PC_DATA (rw) register accessor: PC Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_data`] module"]
#[doc(alias = "PC_DATA")]
pub type PcData = crate::Reg<pc_data::PcDataSpec>;
#[doc = "PC Data Register"]
pub mod pc_data;
#[doc = "PC_DRV0 (rw) register accessor: PC Multi-Driving Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_drv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_drv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_drv0`] module"]
#[doc(alias = "PC_DRV0")]
pub type PcDrv0 = crate::Reg<pc_drv0::PcDrv0Spec>;
#[doc = "PC Multi-Driving Register 0"]
pub mod pc_drv0;
#[doc = "PC_DRV1 (rw) register accessor: PC Multi-Driving Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_drv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_drv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_drv1`] module"]
#[doc(alias = "PC_DRV1")]
pub type PcDrv1 = crate::Reg<pc_drv1::PcDrv1Spec>;
#[doc = "PC Multi-Driving Register 1"]
pub mod pc_drv1;
#[doc = "PC_PULL0 (rw) register accessor: PC Pull Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_pull0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_pull0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_pull0`] module"]
#[doc(alias = "PC_PULL0")]
pub type PcPull0 = crate::Reg<pc_pull0::PcPull0Spec>;
#[doc = "PC Pull Register 0"]
pub mod pc_pull0;
#[doc = "PC_PULL1 (rw) register accessor: PC Pull Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pc_pull1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc_pull1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc_pull1`] module"]
#[doc(alias = "PC_PULL1")]
pub type PcPull1 = crate::Reg<pc_pull1::PcPull1Spec>;
#[doc = "PC Pull Register 1"]
pub mod pc_pull1;
#[doc = "PD_CFG0 (rw) register accessor: PD Configure Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_cfg0`] module"]
#[doc(alias = "PD_CFG0")]
pub type PdCfg0 = crate::Reg<pd_cfg0::PdCfg0Spec>;
#[doc = "PD Configure Register 0"]
pub mod pd_cfg0;
#[doc = "PD_CFG1 (rw) register accessor: PD Configure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_cfg1`] module"]
#[doc(alias = "PD_CFG1")]
pub type PdCfg1 = crate::Reg<pd_cfg1::PdCfg1Spec>;
#[doc = "PD Configure Register 1"]
pub mod pd_cfg1;
#[doc = "PD_CFG2 (rw) register accessor: PD Configure Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_cfg2`] module"]
#[doc(alias = "PD_CFG2")]
pub type PdCfg2 = crate::Reg<pd_cfg2::PdCfg2Spec>;
#[doc = "PD Configure Register 2"]
pub mod pd_cfg2;
#[doc = "PD_CFG3 (rw) register accessor: PD Configure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_cfg3`] module"]
#[doc(alias = "PD_CFG3")]
pub type PdCfg3 = crate::Reg<pd_cfg3::PdCfg3Spec>;
#[doc = "PD Configure Register 3"]
pub mod pd_cfg3;
#[doc = "PD_DATA (rw) register accessor: PD Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_data`] module"]
#[doc(alias = "PD_DATA")]
pub type PdData = crate::Reg<pd_data::PdDataSpec>;
#[doc = "PD Data Register"]
pub mod pd_data;
#[doc = "PD_DRV0 (rw) register accessor: PD Multi-Driving Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_drv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_drv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_drv0`] module"]
#[doc(alias = "PD_DRV0")]
pub type PdDrv0 = crate::Reg<pd_drv0::PdDrv0Spec>;
#[doc = "PD Multi-Driving Register 0"]
pub mod pd_drv0;
#[doc = "PD_DRV1 (rw) register accessor: PD Multi-Driving Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_drv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_drv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_drv1`] module"]
#[doc(alias = "PD_DRV1")]
pub type PdDrv1 = crate::Reg<pd_drv1::PdDrv1Spec>;
#[doc = "PD Multi-Driving Register 1"]
pub mod pd_drv1;
#[doc = "PD_PULL0 (rw) register accessor: PD Pull Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_pull0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_pull0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_pull0`] module"]
#[doc(alias = "PD_PULL0")]
pub type PdPull0 = crate::Reg<pd_pull0::PdPull0Spec>;
#[doc = "PD Pull Register 0"]
pub mod pd_pull0;
#[doc = "PD_PULL1 (rw) register accessor: PD Pull Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_pull1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_pull1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_pull1`] module"]
#[doc(alias = "PD_PULL1")]
pub type PdPull1 = crate::Reg<pd_pull1::PdPull1Spec>;
#[doc = "PD Pull Register 1"]
pub mod pd_pull1;
#[doc = "PE_CFG0 (rw) register accessor: PE Configure Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_cfg0`] module"]
#[doc(alias = "PE_CFG0")]
pub type PeCfg0 = crate::Reg<pe_cfg0::PeCfg0Spec>;
#[doc = "PE Configure Register 0"]
pub mod pe_cfg0;
#[doc = "PE_CFG1 (rw) register accessor: PE Configure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_cfg1`] module"]
#[doc(alias = "PE_CFG1")]
pub type PeCfg1 = crate::Reg<pe_cfg1::PeCfg1Spec>;
#[doc = "PE Configure Register 1"]
pub mod pe_cfg1;
#[doc = "PE_CFG2 (rw) register accessor: PE Configure Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_cfg2`] module"]
#[doc(alias = "PE_CFG2")]
pub type PeCfg2 = crate::Reg<pe_cfg2::PeCfg2Spec>;
#[doc = "PE Configure Register 2"]
pub mod pe_cfg2;
#[doc = "PE_CFG3 (rw) register accessor: PE Configure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_cfg3`] module"]
#[doc(alias = "PE_CFG3")]
pub type PeCfg3 = crate::Reg<pe_cfg3::PeCfg3Spec>;
#[doc = "PE Configure Register 3"]
pub mod pe_cfg3;
#[doc = "PE_DATA (rw) register accessor: PE Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_data`] module"]
#[doc(alias = "PE_DATA")]
pub type PeData = crate::Reg<pe_data::PeDataSpec>;
#[doc = "PE Data Register"]
pub mod pe_data;
#[doc = "PE_DRV0 (rw) register accessor: PE Multi-Driving Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_drv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_drv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_drv0`] module"]
#[doc(alias = "PE_DRV0")]
pub type PeDrv0 = crate::Reg<pe_drv0::PeDrv0Spec>;
#[doc = "PE Multi-Driving Register 0"]
pub mod pe_drv0;
#[doc = "PE_DRV1 (rw) register accessor: PE Multi-Driving Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_drv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_drv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_drv1`] module"]
#[doc(alias = "PE_DRV1")]
pub type PeDrv1 = crate::Reg<pe_drv1::PeDrv1Spec>;
#[doc = "PE Multi-Driving Register 1"]
pub mod pe_drv1;
#[doc = "PE_PULL0 (rw) register accessor: PE Pull Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_pull0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_pull0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_pull0`] module"]
#[doc(alias = "PE_PULL0")]
pub type PePull0 = crate::Reg<pe_pull0::PePull0Spec>;
#[doc = "PE Pull Register 0"]
pub mod pe_pull0;
#[doc = "PE_PULL1 (rw) register accessor: PE Pull Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_pull1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_pull1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_pull1`] module"]
#[doc(alias = "PE_PULL1")]
pub type PePull1 = crate::Reg<pe_pull1::PePull1Spec>;
#[doc = "PE Pull Register 1"]
pub mod pe_pull1;
#[doc = "PF_CFG0 (rw) register accessor: PF Configure Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_cfg0`] module"]
#[doc(alias = "PF_CFG0")]
pub type PfCfg0 = crate::Reg<pf_cfg0::PfCfg0Spec>;
#[doc = "PF Configure Register 0"]
pub mod pf_cfg0;
#[doc = "PF_CFG1 (rw) register accessor: PF Configure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_cfg1`] module"]
#[doc(alias = "PF_CFG1")]
pub type PfCfg1 = crate::Reg<pf_cfg1::PfCfg1Spec>;
#[doc = "PF Configure Register 1"]
pub mod pf_cfg1;
#[doc = "PF_CFG2 (rw) register accessor: PF Configure Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_cfg2`] module"]
#[doc(alias = "PF_CFG2")]
pub type PfCfg2 = crate::Reg<pf_cfg2::PfCfg2Spec>;
#[doc = "PF Configure Register 2"]
pub mod pf_cfg2;
#[doc = "PF_CFG3 (rw) register accessor: PF Configure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_cfg3`] module"]
#[doc(alias = "PF_CFG3")]
pub type PfCfg3 = crate::Reg<pf_cfg3::PfCfg3Spec>;
#[doc = "PF Configure Register 3"]
pub mod pf_cfg3;
#[doc = "PF_DATA (rw) register accessor: PF Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_data`] module"]
#[doc(alias = "PF_DATA")]
pub type PfData = crate::Reg<pf_data::PfDataSpec>;
#[doc = "PF Data Register"]
pub mod pf_data;
#[doc = "PF_DRV0 (rw) register accessor: PF Multi-Driving Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_drv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_drv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_drv0`] module"]
#[doc(alias = "PF_DRV0")]
pub type PfDrv0 = crate::Reg<pf_drv0::PfDrv0Spec>;
#[doc = "PF Multi-Driving Register 0"]
pub mod pf_drv0;
#[doc = "PF_DRV1 (rw) register accessor: PF Multi-Driving Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_drv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_drv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_drv1`] module"]
#[doc(alias = "PF_DRV1")]
pub type PfDrv1 = crate::Reg<pf_drv1::PfDrv1Spec>;
#[doc = "PF Multi-Driving Register 1"]
pub mod pf_drv1;
#[doc = "PF_PULL0 (rw) register accessor: PF Pull Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_pull0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_pull0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_pull0`] module"]
#[doc(alias = "PF_PULL0")]
pub type PfPull0 = crate::Reg<pf_pull0::PfPull0Spec>;
#[doc = "PF Pull Register 0"]
pub mod pf_pull0;
#[doc = "PF_PULL1 (rw) register accessor: PF Pull Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_pull1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_pull1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_pull1`] module"]
#[doc(alias = "PF_PULL1")]
pub type PfPull1 = crate::Reg<pf_pull1::PfPull1Spec>;
#[doc = "PF Pull Register 1"]
pub mod pf_pull1;
#[doc = "PD_EINT_CFG0 (rw) register accessor: PD External Interrupt Configure Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_eint_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_eint_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_eint_cfg0`] module"]
#[doc(alias = "PD_EINT_CFG0")]
pub type PdEintCfg0 = crate::Reg<pd_eint_cfg0::PdEintCfg0Spec>;
#[doc = "PD External Interrupt Configure Register 0"]
pub mod pd_eint_cfg0;
#[doc = "PD_EINT_CFG1 (rw) register accessor: PD External Interrupt Configure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_eint_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_eint_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_eint_cfg1`] module"]
#[doc(alias = "PD_EINT_CFG1")]
pub type PdEintCfg1 = crate::Reg<pd_eint_cfg1::PdEintCfg1Spec>;
#[doc = "PD External Interrupt Configure Register 1"]
pub mod pd_eint_cfg1;
#[doc = "PD_EINT_CFG2 (rw) register accessor: PD External Interrupt Configure Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_eint_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_eint_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_eint_cfg2`] module"]
#[doc(alias = "PD_EINT_CFG2")]
pub type PdEintCfg2 = crate::Reg<pd_eint_cfg2::PdEintCfg2Spec>;
#[doc = "PD External Interrupt Configure Register 2"]
pub mod pd_eint_cfg2;
#[doc = "PD_EINT_CFG3 (rw) register accessor: PD External Interrupt Configure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_eint_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_eint_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_eint_cfg3`] module"]
#[doc(alias = "PD_EINT_CFG3")]
pub type PdEintCfg3 = crate::Reg<pd_eint_cfg3::PdEintCfg3Spec>;
#[doc = "PD External Interrupt Configure Register 3"]
pub mod pd_eint_cfg3;
#[doc = "PD_EINT_CTL (rw) register accessor: PD External Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_eint_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_eint_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_eint_ctl`] module"]
#[doc(alias = "PD_EINT_CTL")]
pub type PdEintCtl = crate::Reg<pd_eint_ctl::PdEintCtlSpec>;
#[doc = "PD External Interrupt Control Register"]
pub mod pd_eint_ctl;
#[doc = "PD_EINT_STA (rw) register accessor: PD External Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_eint_sta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_eint_sta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_eint_sta`] module"]
#[doc(alias = "PD_EINT_STA")]
pub type PdEintSta = crate::Reg<pd_eint_sta::PdEintStaSpec>;
#[doc = "PD External Interrupt Status Register"]
pub mod pd_eint_sta;
#[doc = "PD_EINT_DEB (rw) register accessor: PD External Interrupt Debounce Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pd_eint_deb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd_eint_deb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_eint_deb`] module"]
#[doc(alias = "PD_EINT_DEB")]
pub type PdEintDeb = crate::Reg<pd_eint_deb::PdEintDebSpec>;
#[doc = "PD External Interrupt Debounce Register"]
pub mod pd_eint_deb;
#[doc = "PE_EINT_CFG0 (rw) register accessor: PE External Interrupt Configure Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_eint_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_eint_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_eint_cfg0`] module"]
#[doc(alias = "PE_EINT_CFG0")]
pub type PeEintCfg0 = crate::Reg<pe_eint_cfg0::PeEintCfg0Spec>;
#[doc = "PE External Interrupt Configure Register 0"]
pub mod pe_eint_cfg0;
#[doc = "PE_EINT_CFG1 (rw) register accessor: PE External Interrupt Configure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_eint_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_eint_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_eint_cfg1`] module"]
#[doc(alias = "PE_EINT_CFG1")]
pub type PeEintCfg1 = crate::Reg<pe_eint_cfg1::PeEintCfg1Spec>;
#[doc = "PE External Interrupt Configure Register 1"]
pub mod pe_eint_cfg1;
#[doc = "PE_EINT_CFG2 (rw) register accessor: PE External Interrupt Configure Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_eint_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_eint_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_eint_cfg2`] module"]
#[doc(alias = "PE_EINT_CFG2")]
pub type PeEintCfg2 = crate::Reg<pe_eint_cfg2::PeEintCfg2Spec>;
#[doc = "PE External Interrupt Configure Register 2"]
pub mod pe_eint_cfg2;
#[doc = "PE_EINT_CFG3 (rw) register accessor: PE External Interrupt Configure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_eint_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_eint_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_eint_cfg3`] module"]
#[doc(alias = "PE_EINT_CFG3")]
pub type PeEintCfg3 = crate::Reg<pe_eint_cfg3::PeEintCfg3Spec>;
#[doc = "PE External Interrupt Configure Register 3"]
pub mod pe_eint_cfg3;
#[doc = "PE_EINT_CTL (rw) register accessor: PE External Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_eint_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_eint_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_eint_ctl`] module"]
#[doc(alias = "PE_EINT_CTL")]
pub type PeEintCtl = crate::Reg<pe_eint_ctl::PeEintCtlSpec>;
#[doc = "PE External Interrupt Control Register"]
pub mod pe_eint_ctl;
#[doc = "PE_EINT_STA (rw) register accessor: PE External Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_eint_sta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_eint_sta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_eint_sta`] module"]
#[doc(alias = "PE_EINT_STA")]
pub type PeEintSta = crate::Reg<pe_eint_sta::PeEintStaSpec>;
#[doc = "PE External Interrupt Status Register"]
pub mod pe_eint_sta;
#[doc = "PE_EINT_DEB (rw) register accessor: PE External Interrupt Debounce Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe_eint_deb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe_eint_deb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pe_eint_deb`] module"]
#[doc(alias = "PE_EINT_DEB")]
pub type PeEintDeb = crate::Reg<pe_eint_deb::PeEintDebSpec>;
#[doc = "PE External Interrupt Debounce Register"]
pub mod pe_eint_deb;
#[doc = "PF_EINT_CFG0 (rw) register accessor: PF External Interrupt Configure Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_eint_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_eint_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_eint_cfg0`] module"]
#[doc(alias = "PF_EINT_CFG0")]
pub type PfEintCfg0 = crate::Reg<pf_eint_cfg0::PfEintCfg0Spec>;
#[doc = "PF External Interrupt Configure Register 0"]
pub mod pf_eint_cfg0;
#[doc = "PF_EINT_CFG1 (rw) register accessor: PF External Interrupt Configure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_eint_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_eint_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_eint_cfg1`] module"]
#[doc(alias = "PF_EINT_CFG1")]
pub type PfEintCfg1 = crate::Reg<pf_eint_cfg1::PfEintCfg1Spec>;
#[doc = "PF External Interrupt Configure Register 1"]
pub mod pf_eint_cfg1;
#[doc = "PF_EINT_CFG2 (rw) register accessor: PF External Interrupt Configure Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_eint_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_eint_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_eint_cfg2`] module"]
#[doc(alias = "PF_EINT_CFG2")]
pub type PfEintCfg2 = crate::Reg<pf_eint_cfg2::PfEintCfg2Spec>;
#[doc = "PF External Interrupt Configure Register 2"]
pub mod pf_eint_cfg2;
#[doc = "PF_EINT_CFG3 (rw) register accessor: PF External Interrupt Configure Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_eint_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_eint_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_eint_cfg3`] module"]
#[doc(alias = "PF_EINT_CFG3")]
pub type PfEintCfg3 = crate::Reg<pf_eint_cfg3::PfEintCfg3Spec>;
#[doc = "PF External Interrupt Configure Register 3"]
pub mod pf_eint_cfg3;
#[doc = "PF_EINT_CTL (rw) register accessor: PF External Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_eint_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_eint_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_eint_ctl`] module"]
#[doc(alias = "PF_EINT_CTL")]
pub type PfEintCtl = crate::Reg<pf_eint_ctl::PfEintCtlSpec>;
#[doc = "PF External Interrupt Control Register"]
pub mod pf_eint_ctl;
#[doc = "PF_EINT_STA (rw) register accessor: PF External Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_eint_sta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_eint_sta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_eint_sta`] module"]
#[doc(alias = "PF_EINT_STA")]
pub type PfEintSta = crate::Reg<pf_eint_sta::PfEintStaSpec>;
#[doc = "PF External Interrupt Status Register"]
pub mod pf_eint_sta;
#[doc = "PF_EINT_DEB (rw) register accessor: PF External Interrupt Debounce Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pf_eint_deb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pf_eint_deb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pf_eint_deb`] module"]
#[doc(alias = "PF_EINT_DEB")]
pub type PfEintDeb = crate::Reg<pf_eint_deb::PfEintDebSpec>;
#[doc = "PF External Interrupt Debounce Register"]
pub mod pf_eint_deb;
#[doc = "SDR_PAD_DRV (rw) register accessor: SDRAM Pad Multi-Driving Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdr_pad_drv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdr_pad_drv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdr_pad_drv`] module"]
#[doc(alias = "SDR_PAD_DRV")]
pub type SdrPadDrv = crate::Reg<sdr_pad_drv::SdrPadDrvSpec>;
#[doc = "SDRAM Pad Multi-Driving Register"]
pub mod sdr_pad_drv;
#[doc = "SDR_PAD_PULL (rw) register accessor: SDRAM Pad Pull Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdr_pad_pull::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdr_pad_pull::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdr_pad_pull`] module"]
#[doc(alias = "SDR_PAD_PULL")]
pub type SdrPadPull = crate::Reg<sdr_pad_pull::SdrPadPullSpec>;
#[doc = "SDRAM Pad Pull Register"]
pub mod sdr_pad_pull;
