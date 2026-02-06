#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dma_int_ctrl: DmaIntCtrl,
    dma_int_sta: DmaIntSta,
    dma_pty_cfg: DmaPtyCfg,
    _reserved3: [u8; 0xf4],
    ndma_cfg_reg0: NdmaCfgReg0,
    ndma_src_adr_reg0: NdmaSrcAdrReg0,
    ndma_des_adr_reg0: NdmaDesAdrReg0,
    ndma_byte_cnt_reg0: NdmaByteCntReg0,
    _reserved7: [u8; 0x10],
    ndma_cfg_reg1: NdmaCfgReg1,
    ndma_src_adr_reg1: NdmaSrcAdrReg1,
    ndma_des_adr_reg1: NdmaDesAdrReg1,
    ndma_byte_cnt_reg1: NdmaByteCntReg1,
    _reserved11: [u8; 0x10],
    ndma_cfg_reg2: NdmaCfgReg2,
    ndma_src_adr_reg2: NdmaSrcAdrReg2,
    ndma_des_adr_reg2: NdmaDesAdrReg2,
    ndma_byte_cnt_reg2: NdmaByteCntReg2,
    _reserved15: [u8; 0x10],
    ndma_cfg_reg3: NdmaCfgReg3,
    ndma_src_adr_reg3: NdmaSrcAdrReg3,
    ndma_des_adr_reg3: NdmaDesAdrReg3,
    ndma_byte_cnt_reg3: NdmaByteCntReg3,
    _reserved19: [u8; 0x0190],
    ddma_cfg_reg0: DdmaCfgReg0,
    ddma_src_adr_reg0: DdmaSrcAdrReg0,
    ddma_des_adr_reg0: DdmaDesAdrReg0,
    ddma_byte_cnt_reg0: DdmaByteCntReg0,
    _reserved23: [u8; 0x08],
    ddma_par_reg0: DdmaParReg0,
    _reserved24: [u8; 0x04],
    ddma_cfg_reg1: DdmaCfgReg1,
    ddma_src_adr_reg1: DdmaSrcAdrReg1,
    ddma_des_adr_reg1: DdmaDesAdrReg1,
    ddma_byte_cnt_reg1: DdmaByteCntReg1,
    _reserved28: [u8; 0x08],
    ddma_par_reg1: DdmaParReg1,
    _reserved29: [u8; 0x04],
    ddma_cfg_reg2: DdmaCfgReg2,
    ddma_src_adr_reg2: DdmaSrcAdrReg2,
    ddma_des_adr_reg2: DdmaDesAdrReg2,
    ddma_byte_cnt_reg2: DdmaByteCntReg2,
    _reserved33: [u8; 0x08],
    ddma_par_reg2: DdmaParReg2,
    _reserved34: [u8; 0x04],
    ddma_cfg_reg3: DdmaCfgReg3,
    ddma_src_adr_reg3: DdmaSrcAdrReg3,
    ddma_des_adr_reg3: DdmaDesAdrReg3,
    ddma_byte_cnt_reg3: DdmaByteCntReg3,
    _reserved38: [u8; 0x08],
    ddma_par_reg3: DdmaParReg3,
    ddma_gen_data: DdmaGenData,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Interrupt Control Register"]
    #[inline(always)]
    pub const fn dma_int_ctrl(&self) -> &DmaIntCtrl {
        &self.dma_int_ctrl
    }
    #[doc = "0x04 - DMA Interrupt Status Register (write 1 to clear)"]
    #[inline(always)]
    pub const fn dma_int_sta(&self) -> &DmaIntSta {
        &self.dma_int_sta
    }
    #[doc = "0x08 - DMA Priority Configure Register"]
    #[inline(always)]
    pub const fn dma_pty_cfg(&self) -> &DmaPtyCfg {
        &self.dma_pty_cfg
    }
    #[doc = "0x100 - Normal DMA Channel 0 Configure Register"]
    #[inline(always)]
    pub const fn ndma_cfg_reg0(&self) -> &NdmaCfgReg0 {
        &self.ndma_cfg_reg0
    }
    #[doc = "0x104 - Normal DMA Channel 0 Source Address Register"]
    #[inline(always)]
    pub const fn ndma_src_adr_reg0(&self) -> &NdmaSrcAdrReg0 {
        &self.ndma_src_adr_reg0
    }
    #[doc = "0x108 - Normal DMA Channel 0 Destination Address Register"]
    #[inline(always)]
    pub const fn ndma_des_adr_reg0(&self) -> &NdmaDesAdrReg0 {
        &self.ndma_des_adr_reg0
    }
    #[doc = "0x10c - Normal DMA Channel 0 Byte Counter Register (max 128K)"]
    #[inline(always)]
    pub const fn ndma_byte_cnt_reg0(&self) -> &NdmaByteCntReg0 {
        &self.ndma_byte_cnt_reg0
    }
    #[doc = "0x120 - Normal DMA Channel 1 Configure Register"]
    #[inline(always)]
    pub const fn ndma_cfg_reg1(&self) -> &NdmaCfgReg1 {
        &self.ndma_cfg_reg1
    }
    #[doc = "0x124 - Normal DMA Channel 1 Source Address Register"]
    #[inline(always)]
    pub const fn ndma_src_adr_reg1(&self) -> &NdmaSrcAdrReg1 {
        &self.ndma_src_adr_reg1
    }
    #[doc = "0x128 - Normal DMA Channel 1 Destination Address Register"]
    #[inline(always)]
    pub const fn ndma_des_adr_reg1(&self) -> &NdmaDesAdrReg1 {
        &self.ndma_des_adr_reg1
    }
    #[doc = "0x12c - Normal DMA Channel 1 Byte Counter Register"]
    #[inline(always)]
    pub const fn ndma_byte_cnt_reg1(&self) -> &NdmaByteCntReg1 {
        &self.ndma_byte_cnt_reg1
    }
    #[doc = "0x140 - Normal DMA Channel 2 Configure Register"]
    #[inline(always)]
    pub const fn ndma_cfg_reg2(&self) -> &NdmaCfgReg2 {
        &self.ndma_cfg_reg2
    }
    #[doc = "0x144 - Normal DMA Channel 2 Source Address Register"]
    #[inline(always)]
    pub const fn ndma_src_adr_reg2(&self) -> &NdmaSrcAdrReg2 {
        &self.ndma_src_adr_reg2
    }
    #[doc = "0x148 - Normal DMA Channel 2 Destination Address Register"]
    #[inline(always)]
    pub const fn ndma_des_adr_reg2(&self) -> &NdmaDesAdrReg2 {
        &self.ndma_des_adr_reg2
    }
    #[doc = "0x14c - Normal DMA Channel 2 Byte Counter Register"]
    #[inline(always)]
    pub const fn ndma_byte_cnt_reg2(&self) -> &NdmaByteCntReg2 {
        &self.ndma_byte_cnt_reg2
    }
    #[doc = "0x160 - Normal DMA Channel 3 Configure Register"]
    #[inline(always)]
    pub const fn ndma_cfg_reg3(&self) -> &NdmaCfgReg3 {
        &self.ndma_cfg_reg3
    }
    #[doc = "0x164 - Normal DMA Channel 3 Source Address Register"]
    #[inline(always)]
    pub const fn ndma_src_adr_reg3(&self) -> &NdmaSrcAdrReg3 {
        &self.ndma_src_adr_reg3
    }
    #[doc = "0x168 - Normal DMA Channel 3 Destination Address Register"]
    #[inline(always)]
    pub const fn ndma_des_adr_reg3(&self) -> &NdmaDesAdrReg3 {
        &self.ndma_des_adr_reg3
    }
    #[doc = "0x16c - Normal DMA Channel 3 Byte Counter Register"]
    #[inline(always)]
    pub const fn ndma_byte_cnt_reg3(&self) -> &NdmaByteCntReg3 {
        &self.ndma_byte_cnt_reg3
    }
    #[doc = "0x300 - Dedicated DMA Channel 0 Configure Register"]
    #[inline(always)]
    pub const fn ddma_cfg_reg0(&self) -> &DdmaCfgReg0 {
        &self.ddma_cfg_reg0
    }
    #[doc = "0x304 - Dedicated DMA Channel 0 Source Address Register"]
    #[inline(always)]
    pub const fn ddma_src_adr_reg0(&self) -> &DdmaSrcAdrReg0 {
        &self.ddma_src_adr_reg0
    }
    #[doc = "0x308 - Dedicated DMA Channel 0 Destination Address Register"]
    #[inline(always)]
    pub const fn ddma_des_adr_reg0(&self) -> &DdmaDesAdrReg0 {
        &self.ddma_des_adr_reg0
    }
    #[doc = "0x30c - Dedicated DMA Channel 0 Byte Counter Register (max 0x1000000)"]
    #[inline(always)]
    pub const fn ddma_byte_cnt_reg0(&self) -> &DdmaByteCntReg0 {
        &self.ddma_byte_cnt_reg0
    }
    #[doc = "0x318 - Dedicated DMA Channel 0 Parameter Register"]
    #[inline(always)]
    pub const fn ddma_par_reg0(&self) -> &DdmaParReg0 {
        &self.ddma_par_reg0
    }
    #[doc = "0x320 - Dedicated DMA Channel 1 Configure Register"]
    #[inline(always)]
    pub const fn ddma_cfg_reg1(&self) -> &DdmaCfgReg1 {
        &self.ddma_cfg_reg1
    }
    #[doc = "0x324 - Dedicated DMA Channel 1 Source Address Register"]
    #[inline(always)]
    pub const fn ddma_src_adr_reg1(&self) -> &DdmaSrcAdrReg1 {
        &self.ddma_src_adr_reg1
    }
    #[doc = "0x328 - Dedicated DMA Channel 1 Destination Address Register"]
    #[inline(always)]
    pub const fn ddma_des_adr_reg1(&self) -> &DdmaDesAdrReg1 {
        &self.ddma_des_adr_reg1
    }
    #[doc = "0x32c - Dedicated DMA Channel 1 Byte Counter Register"]
    #[inline(always)]
    pub const fn ddma_byte_cnt_reg1(&self) -> &DdmaByteCntReg1 {
        &self.ddma_byte_cnt_reg1
    }
    #[doc = "0x338 - Dedicated DMA Channel 1 Parameter Register"]
    #[inline(always)]
    pub const fn ddma_par_reg1(&self) -> &DdmaParReg1 {
        &self.ddma_par_reg1
    }
    #[doc = "0x340 - Dedicated DMA Channel 2 Configure Register"]
    #[inline(always)]
    pub const fn ddma_cfg_reg2(&self) -> &DdmaCfgReg2 {
        &self.ddma_cfg_reg2
    }
    #[doc = "0x344 - Dedicated DMA Channel 2 Source Address Register"]
    #[inline(always)]
    pub const fn ddma_src_adr_reg2(&self) -> &DdmaSrcAdrReg2 {
        &self.ddma_src_adr_reg2
    }
    #[doc = "0x348 - Dedicated DMA Channel 2 Destination Address Register"]
    #[inline(always)]
    pub const fn ddma_des_adr_reg2(&self) -> &DdmaDesAdrReg2 {
        &self.ddma_des_adr_reg2
    }
    #[doc = "0x34c - Dedicated DMA Channel 2 Byte Counter Register"]
    #[inline(always)]
    pub const fn ddma_byte_cnt_reg2(&self) -> &DdmaByteCntReg2 {
        &self.ddma_byte_cnt_reg2
    }
    #[doc = "0x358 - Dedicated DMA Channel 2 Parameter Register"]
    #[inline(always)]
    pub const fn ddma_par_reg2(&self) -> &DdmaParReg2 {
        &self.ddma_par_reg2
    }
    #[doc = "0x360 - Dedicated DMA Channel 3 Configure Register (supports memory set mode, INCR8)"]
    #[inline(always)]
    pub const fn ddma_cfg_reg3(&self) -> &DdmaCfgReg3 {
        &self.ddma_cfg_reg3
    }
    #[doc = "0x364 - Dedicated DMA Channel 3 Source Address Register"]
    #[inline(always)]
    pub const fn ddma_src_adr_reg3(&self) -> &DdmaSrcAdrReg3 {
        &self.ddma_src_adr_reg3
    }
    #[doc = "0x368 - Dedicated DMA Channel 3 Destination Address Register"]
    #[inline(always)]
    pub const fn ddma_des_adr_reg3(&self) -> &DdmaDesAdrReg3 {
        &self.ddma_des_adr_reg3
    }
    #[doc = "0x36c - Dedicated DMA Channel 3 Byte Counter Register"]
    #[inline(always)]
    pub const fn ddma_byte_cnt_reg3(&self) -> &DdmaByteCntReg3 {
        &self.ddma_byte_cnt_reg3
    }
    #[doc = "0x378 - Dedicated DMA Channel 3 Parameter Register"]
    #[inline(always)]
    pub const fn ddma_par_reg3(&self) -> &DdmaParReg3 {
        &self.ddma_par_reg3
    }
    #[doc = "0x37c - Dedicated DMA General Data Register (only valid for DDMA3, used with memory set mode)"]
    #[inline(always)]
    pub const fn ddma_gen_data(&self) -> &DdmaGenData {
        &self.ddma_gen_data
    }
}
#[doc = "DMA_INT_CTRL (rw) register accessor: DMA Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_ctrl`] module"]
#[doc(alias = "DMA_INT_CTRL")]
pub type DmaIntCtrl = crate::Reg<dma_int_ctrl::DmaIntCtrlSpec>;
#[doc = "DMA Interrupt Control Register"]
pub mod dma_int_ctrl;
#[doc = "DMA_INT_STA (rw) register accessor: DMA Interrupt Status Register (write 1 to clear)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_sta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_sta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_sta`] module"]
#[doc(alias = "DMA_INT_STA")]
pub type DmaIntSta = crate::Reg<dma_int_sta::DmaIntStaSpec>;
#[doc = "DMA Interrupt Status Register (write 1 to clear)"]
pub mod dma_int_sta;
#[doc = "DMA_PTY_CFG (rw) register accessor: DMA Priority Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_pty_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_pty_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_pty_cfg`] module"]
#[doc(alias = "DMA_PTY_CFG")]
pub type DmaPtyCfg = crate::Reg<dma_pty_cfg::DmaPtyCfgSpec>;
#[doc = "DMA Priority Configure Register"]
pub mod dma_pty_cfg;
#[doc = "NDMA_CFG_REG0 (rw) register accessor: Normal DMA Channel 0 Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_cfg_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_cfg_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndma_cfg_reg0`] module"]
#[doc(alias = "NDMA_CFG_REG0")]
pub type NdmaCfgReg0 = crate::Reg<ndma_cfg_reg0::NdmaCfgReg0Spec>;
#[doc = "Normal DMA Channel 0 Configure Register"]
pub mod ndma_cfg_reg0;
#[doc = "NDMA_SRC_ADR_REG0 (rw) register accessor: Normal DMA Channel 0 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_src_adr_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_src_adr_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndma_src_adr_reg0`] module"]
#[doc(alias = "NDMA_SRC_ADR_REG0")]
pub type NdmaSrcAdrReg0 = crate::Reg<ndma_src_adr_reg0::NdmaSrcAdrReg0Spec>;
#[doc = "Normal DMA Channel 0 Source Address Register"]
pub mod ndma_src_adr_reg0;
#[doc = "NDMA_DES_ADR_REG0 (rw) register accessor: Normal DMA Channel 0 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_des_adr_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_des_adr_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndma_des_adr_reg0`] module"]
#[doc(alias = "NDMA_DES_ADR_REG0")]
pub type NdmaDesAdrReg0 = crate::Reg<ndma_des_adr_reg0::NdmaDesAdrReg0Spec>;
#[doc = "Normal DMA Channel 0 Destination Address Register"]
pub mod ndma_des_adr_reg0;
#[doc = "NDMA_BYTE_CNT_REG0 (rw) register accessor: Normal DMA Channel 0 Byte Counter Register (max 128K)\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_byte_cnt_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_byte_cnt_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndma_byte_cnt_reg0`] module"]
#[doc(alias = "NDMA_BYTE_CNT_REG0")]
pub type NdmaByteCntReg0 = crate::Reg<ndma_byte_cnt_reg0::NdmaByteCntReg0Spec>;
#[doc = "Normal DMA Channel 0 Byte Counter Register (max 128K)"]
pub mod ndma_byte_cnt_reg0;
#[doc = "NDMA_CFG_REG1 (rw) register accessor: Normal DMA Channel 1 Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_cfg_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_cfg_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndma_cfg_reg1`] module"]
#[doc(alias = "NDMA_CFG_REG1")]
pub type NdmaCfgReg1 = crate::Reg<ndma_cfg_reg1::NdmaCfgReg1Spec>;
#[doc = "Normal DMA Channel 1 Configure Register"]
pub mod ndma_cfg_reg1;
#[doc = "NDMA_SRC_ADR_REG1 (rw) register accessor: Normal DMA Channel 1 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_src_adr_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_src_adr_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndma_src_adr_reg1`] module"]
#[doc(alias = "NDMA_SRC_ADR_REG1")]
pub type NdmaSrcAdrReg1 = crate::Reg<ndma_src_adr_reg1::NdmaSrcAdrReg1Spec>;
#[doc = "Normal DMA Channel 1 Source Address Register"]
pub mod ndma_src_adr_reg1;
#[doc = "NDMA_DES_ADR_REG1 (rw) register accessor: Normal DMA Channel 1 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_des_adr_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_des_adr_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndma_des_adr_reg1`] module"]
#[doc(alias = "NDMA_DES_ADR_REG1")]
pub type NdmaDesAdrReg1 = crate::Reg<ndma_des_adr_reg1::NdmaDesAdrReg1Spec>;
#[doc = "Normal DMA Channel 1 Destination Address Register"]
pub mod ndma_des_adr_reg1;
#[doc = "NDMA_BYTE_CNT_REG1 (rw) register accessor: Normal DMA Channel 1 Byte Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_byte_cnt_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_byte_cnt_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndma_byte_cnt_reg1`] module"]
#[doc(alias = "NDMA_BYTE_CNT_REG1")]
pub type NdmaByteCntReg1 = crate::Reg<ndma_byte_cnt_reg1::NdmaByteCntReg1Spec>;
#[doc = "Normal DMA Channel 1 Byte Counter Register"]
pub mod ndma_byte_cnt_reg1;
#[doc = "NDMA_CFG_REG2 (rw) register accessor: Normal DMA Channel 2 Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_cfg_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_cfg_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndma_cfg_reg2`] module"]
#[doc(alias = "NDMA_CFG_REG2")]
pub type NdmaCfgReg2 = crate::Reg<ndma_cfg_reg2::NdmaCfgReg2Spec>;
#[doc = "Normal DMA Channel 2 Configure Register"]
pub mod ndma_cfg_reg2;
#[doc = "NDMA_SRC_ADR_REG2 (rw) register accessor: Normal DMA Channel 2 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_src_adr_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_src_adr_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndma_src_adr_reg2`] module"]
#[doc(alias = "NDMA_SRC_ADR_REG2")]
pub type NdmaSrcAdrReg2 = crate::Reg<ndma_src_adr_reg2::NdmaSrcAdrReg2Spec>;
#[doc = "Normal DMA Channel 2 Source Address Register"]
pub mod ndma_src_adr_reg2;
#[doc = "NDMA_DES_ADR_REG2 (rw) register accessor: Normal DMA Channel 2 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_des_adr_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_des_adr_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndma_des_adr_reg2`] module"]
#[doc(alias = "NDMA_DES_ADR_REG2")]
pub type NdmaDesAdrReg2 = crate::Reg<ndma_des_adr_reg2::NdmaDesAdrReg2Spec>;
#[doc = "Normal DMA Channel 2 Destination Address Register"]
pub mod ndma_des_adr_reg2;
#[doc = "NDMA_BYTE_CNT_REG2 (rw) register accessor: Normal DMA Channel 2 Byte Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_byte_cnt_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_byte_cnt_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndma_byte_cnt_reg2`] module"]
#[doc(alias = "NDMA_BYTE_CNT_REG2")]
pub type NdmaByteCntReg2 = crate::Reg<ndma_byte_cnt_reg2::NdmaByteCntReg2Spec>;
#[doc = "Normal DMA Channel 2 Byte Counter Register"]
pub mod ndma_byte_cnt_reg2;
#[doc = "NDMA_CFG_REG3 (rw) register accessor: Normal DMA Channel 3 Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_cfg_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_cfg_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndma_cfg_reg3`] module"]
#[doc(alias = "NDMA_CFG_REG3")]
pub type NdmaCfgReg3 = crate::Reg<ndma_cfg_reg3::NdmaCfgReg3Spec>;
#[doc = "Normal DMA Channel 3 Configure Register"]
pub mod ndma_cfg_reg3;
#[doc = "NDMA_SRC_ADR_REG3 (rw) register accessor: Normal DMA Channel 3 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_src_adr_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_src_adr_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndma_src_adr_reg3`] module"]
#[doc(alias = "NDMA_SRC_ADR_REG3")]
pub type NdmaSrcAdrReg3 = crate::Reg<ndma_src_adr_reg3::NdmaSrcAdrReg3Spec>;
#[doc = "Normal DMA Channel 3 Source Address Register"]
pub mod ndma_src_adr_reg3;
#[doc = "NDMA_DES_ADR_REG3 (rw) register accessor: Normal DMA Channel 3 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_des_adr_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_des_adr_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndma_des_adr_reg3`] module"]
#[doc(alias = "NDMA_DES_ADR_REG3")]
pub type NdmaDesAdrReg3 = crate::Reg<ndma_des_adr_reg3::NdmaDesAdrReg3Spec>;
#[doc = "Normal DMA Channel 3 Destination Address Register"]
pub mod ndma_des_adr_reg3;
#[doc = "NDMA_BYTE_CNT_REG3 (rw) register accessor: Normal DMA Channel 3 Byte Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndma_byte_cnt_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndma_byte_cnt_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndma_byte_cnt_reg3`] module"]
#[doc(alias = "NDMA_BYTE_CNT_REG3")]
pub type NdmaByteCntReg3 = crate::Reg<ndma_byte_cnt_reg3::NdmaByteCntReg3Spec>;
#[doc = "Normal DMA Channel 3 Byte Counter Register"]
pub mod ndma_byte_cnt_reg3;
#[doc = "DDMA_CFG_REG0 (rw) register accessor: Dedicated DMA Channel 0 Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_cfg_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_cfg_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_cfg_reg0`] module"]
#[doc(alias = "DDMA_CFG_REG0")]
pub type DdmaCfgReg0 = crate::Reg<ddma_cfg_reg0::DdmaCfgReg0Spec>;
#[doc = "Dedicated DMA Channel 0 Configure Register"]
pub mod ddma_cfg_reg0;
#[doc = "DDMA_SRC_ADR_REG0 (rw) register accessor: Dedicated DMA Channel 0 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_src_adr_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_src_adr_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_src_adr_reg0`] module"]
#[doc(alias = "DDMA_SRC_ADR_REG0")]
pub type DdmaSrcAdrReg0 = crate::Reg<ddma_src_adr_reg0::DdmaSrcAdrReg0Spec>;
#[doc = "Dedicated DMA Channel 0 Source Address Register"]
pub mod ddma_src_adr_reg0;
#[doc = "DDMA_DES_ADR_REG0 (rw) register accessor: Dedicated DMA Channel 0 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_des_adr_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_des_adr_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_des_adr_reg0`] module"]
#[doc(alias = "DDMA_DES_ADR_REG0")]
pub type DdmaDesAdrReg0 = crate::Reg<ddma_des_adr_reg0::DdmaDesAdrReg0Spec>;
#[doc = "Dedicated DMA Channel 0 Destination Address Register"]
pub mod ddma_des_adr_reg0;
#[doc = "DDMA_BYTE_CNT_REG0 (rw) register accessor: Dedicated DMA Channel 0 Byte Counter Register (max 0x1000000)\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_byte_cnt_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_byte_cnt_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_byte_cnt_reg0`] module"]
#[doc(alias = "DDMA_BYTE_CNT_REG0")]
pub type DdmaByteCntReg0 = crate::Reg<ddma_byte_cnt_reg0::DdmaByteCntReg0Spec>;
#[doc = "Dedicated DMA Channel 0 Byte Counter Register (max 0x1000000)"]
pub mod ddma_byte_cnt_reg0;
#[doc = "DDMA_PAR_REG0 (rw) register accessor: Dedicated DMA Channel 0 Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_par_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_par_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_par_reg0`] module"]
#[doc(alias = "DDMA_PAR_REG0")]
pub type DdmaParReg0 = crate::Reg<ddma_par_reg0::DdmaParReg0Spec>;
#[doc = "Dedicated DMA Channel 0 Parameter Register"]
pub mod ddma_par_reg0;
#[doc = "DDMA_CFG_REG1 (rw) register accessor: Dedicated DMA Channel 1 Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_cfg_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_cfg_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_cfg_reg1`] module"]
#[doc(alias = "DDMA_CFG_REG1")]
pub type DdmaCfgReg1 = crate::Reg<ddma_cfg_reg1::DdmaCfgReg1Spec>;
#[doc = "Dedicated DMA Channel 1 Configure Register"]
pub mod ddma_cfg_reg1;
#[doc = "DDMA_SRC_ADR_REG1 (rw) register accessor: Dedicated DMA Channel 1 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_src_adr_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_src_adr_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_src_adr_reg1`] module"]
#[doc(alias = "DDMA_SRC_ADR_REG1")]
pub type DdmaSrcAdrReg1 = crate::Reg<ddma_src_adr_reg1::DdmaSrcAdrReg1Spec>;
#[doc = "Dedicated DMA Channel 1 Source Address Register"]
pub mod ddma_src_adr_reg1;
#[doc = "DDMA_DES_ADR_REG1 (rw) register accessor: Dedicated DMA Channel 1 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_des_adr_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_des_adr_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_des_adr_reg1`] module"]
#[doc(alias = "DDMA_DES_ADR_REG1")]
pub type DdmaDesAdrReg1 = crate::Reg<ddma_des_adr_reg1::DdmaDesAdrReg1Spec>;
#[doc = "Dedicated DMA Channel 1 Destination Address Register"]
pub mod ddma_des_adr_reg1;
#[doc = "DDMA_BYTE_CNT_REG1 (rw) register accessor: Dedicated DMA Channel 1 Byte Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_byte_cnt_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_byte_cnt_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_byte_cnt_reg1`] module"]
#[doc(alias = "DDMA_BYTE_CNT_REG1")]
pub type DdmaByteCntReg1 = crate::Reg<ddma_byte_cnt_reg1::DdmaByteCntReg1Spec>;
#[doc = "Dedicated DMA Channel 1 Byte Counter Register"]
pub mod ddma_byte_cnt_reg1;
#[doc = "DDMA_PAR_REG1 (rw) register accessor: Dedicated DMA Channel 1 Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_par_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_par_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_par_reg1`] module"]
#[doc(alias = "DDMA_PAR_REG1")]
pub type DdmaParReg1 = crate::Reg<ddma_par_reg1::DdmaParReg1Spec>;
#[doc = "Dedicated DMA Channel 1 Parameter Register"]
pub mod ddma_par_reg1;
#[doc = "DDMA_CFG_REG2 (rw) register accessor: Dedicated DMA Channel 2 Configure Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_cfg_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_cfg_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_cfg_reg2`] module"]
#[doc(alias = "DDMA_CFG_REG2")]
pub type DdmaCfgReg2 = crate::Reg<ddma_cfg_reg2::DdmaCfgReg2Spec>;
#[doc = "Dedicated DMA Channel 2 Configure Register"]
pub mod ddma_cfg_reg2;
#[doc = "DDMA_SRC_ADR_REG2 (rw) register accessor: Dedicated DMA Channel 2 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_src_adr_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_src_adr_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_src_adr_reg2`] module"]
#[doc(alias = "DDMA_SRC_ADR_REG2")]
pub type DdmaSrcAdrReg2 = crate::Reg<ddma_src_adr_reg2::DdmaSrcAdrReg2Spec>;
#[doc = "Dedicated DMA Channel 2 Source Address Register"]
pub mod ddma_src_adr_reg2;
#[doc = "DDMA_DES_ADR_REG2 (rw) register accessor: Dedicated DMA Channel 2 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_des_adr_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_des_adr_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_des_adr_reg2`] module"]
#[doc(alias = "DDMA_DES_ADR_REG2")]
pub type DdmaDesAdrReg2 = crate::Reg<ddma_des_adr_reg2::DdmaDesAdrReg2Spec>;
#[doc = "Dedicated DMA Channel 2 Destination Address Register"]
pub mod ddma_des_adr_reg2;
#[doc = "DDMA_BYTE_CNT_REG2 (rw) register accessor: Dedicated DMA Channel 2 Byte Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_byte_cnt_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_byte_cnt_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_byte_cnt_reg2`] module"]
#[doc(alias = "DDMA_BYTE_CNT_REG2")]
pub type DdmaByteCntReg2 = crate::Reg<ddma_byte_cnt_reg2::DdmaByteCntReg2Spec>;
#[doc = "Dedicated DMA Channel 2 Byte Counter Register"]
pub mod ddma_byte_cnt_reg2;
#[doc = "DDMA_PAR_REG2 (rw) register accessor: Dedicated DMA Channel 2 Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_par_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_par_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_par_reg2`] module"]
#[doc(alias = "DDMA_PAR_REG2")]
pub type DdmaParReg2 = crate::Reg<ddma_par_reg2::DdmaParReg2Spec>;
#[doc = "Dedicated DMA Channel 2 Parameter Register"]
pub mod ddma_par_reg2;
#[doc = "DDMA_CFG_REG3 (rw) register accessor: Dedicated DMA Channel 3 Configure Register (supports memory set mode, INCR8)\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_cfg_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_cfg_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_cfg_reg3`] module"]
#[doc(alias = "DDMA_CFG_REG3")]
pub type DdmaCfgReg3 = crate::Reg<ddma_cfg_reg3::DdmaCfgReg3Spec>;
#[doc = "Dedicated DMA Channel 3 Configure Register (supports memory set mode, INCR8)"]
pub mod ddma_cfg_reg3;
#[doc = "DDMA_SRC_ADR_REG3 (rw) register accessor: Dedicated DMA Channel 3 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_src_adr_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_src_adr_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_src_adr_reg3`] module"]
#[doc(alias = "DDMA_SRC_ADR_REG3")]
pub type DdmaSrcAdrReg3 = crate::Reg<ddma_src_adr_reg3::DdmaSrcAdrReg3Spec>;
#[doc = "Dedicated DMA Channel 3 Source Address Register"]
pub mod ddma_src_adr_reg3;
#[doc = "DDMA_DES_ADR_REG3 (rw) register accessor: Dedicated DMA Channel 3 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_des_adr_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_des_adr_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_des_adr_reg3`] module"]
#[doc(alias = "DDMA_DES_ADR_REG3")]
pub type DdmaDesAdrReg3 = crate::Reg<ddma_des_adr_reg3::DdmaDesAdrReg3Spec>;
#[doc = "Dedicated DMA Channel 3 Destination Address Register"]
pub mod ddma_des_adr_reg3;
#[doc = "DDMA_BYTE_CNT_REG3 (rw) register accessor: Dedicated DMA Channel 3 Byte Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_byte_cnt_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_byte_cnt_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_byte_cnt_reg3`] module"]
#[doc(alias = "DDMA_BYTE_CNT_REG3")]
pub type DdmaByteCntReg3 = crate::Reg<ddma_byte_cnt_reg3::DdmaByteCntReg3Spec>;
#[doc = "Dedicated DMA Channel 3 Byte Counter Register"]
pub mod ddma_byte_cnt_reg3;
#[doc = "DDMA_PAR_REG3 (rw) register accessor: Dedicated DMA Channel 3 Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_par_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_par_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_par_reg3`] module"]
#[doc(alias = "DDMA_PAR_REG3")]
pub type DdmaParReg3 = crate::Reg<ddma_par_reg3::DdmaParReg3Spec>;
#[doc = "Dedicated DMA Channel 3 Parameter Register"]
pub mod ddma_par_reg3;
#[doc = "DDMA_GEN_DATA (rw) register accessor: Dedicated DMA General Data Register (only valid for DDMA3, used with memory set mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ddma_gen_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddma_gen_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddma_gen_data`] module"]
#[doc(alias = "DDMA_GEN_DATA")]
pub type DdmaGenData = crate::Reg<ddma_gen_data::DdmaGenDataSpec>;
#[doc = "Dedicated DMA General Data Register (only valid for DDMA3, used with memory set mode)"]
pub mod ddma_gen_data;
