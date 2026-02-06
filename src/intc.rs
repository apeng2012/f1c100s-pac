#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intc_vector: IntcVector,
    intc_base_addr: IntcBaseAddr,
    _reserved2: [u8; 0x04],
    nmi_int_ctrl: NmiIntCtrl,
    intc_pend_reg0: IntcPendReg0,
    intc_pend_reg1: IntcPendReg1,
    _reserved5: [u8; 0x08],
    intc_en_reg0: IntcEnReg0,
    intc_en_reg1: IntcEnReg1,
    _reserved7: [u8; 0x08],
    intc_mask_reg0: IntcMaskReg0,
    intc_mask_reg1: IntcMaskReg1,
    _reserved9: [u8; 0x08],
    intc_resp_reg0: IntcRespReg0,
    intc_resp_reg1: IntcRespReg1,
    _reserved11: [u8; 0x08],
    intc_ff_reg0: IntcFfReg0,
    intc_ff_reg1: IntcFfReg1,
    _reserved13: [u8; 0x08],
    intc_prio_reg0: IntcPrioReg0,
    intc_prio_reg1: IntcPrioReg1,
    intc_prio_reg2: IntcPrioReg2,
    intc_prio_reg3: IntcPrioReg3,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Vector Register"]
    #[inline(always)]
    pub const fn intc_vector(&self) -> &IntcVector {
        &self.intc_vector
    }
    #[doc = "0x04 - Interrupt Base Address Register"]
    #[inline(always)]
    pub const fn intc_base_addr(&self) -> &IntcBaseAddr {
        &self.intc_base_addr
    }
    #[doc = "0x0c - NMI Interrupt Control Register"]
    #[inline(always)]
    pub const fn nmi_int_ctrl(&self) -> &NmiIntCtrl {
        &self.nmi_int_ctrl
    }
    #[doc = "0x10 - Interrupt IRQ Pending Register 0"]
    #[inline(always)]
    pub const fn intc_pend_reg0(&self) -> &IntcPendReg0 {
        &self.intc_pend_reg0
    }
    #[doc = "0x14 - Interrupt IRQ Pending Register 1"]
    #[inline(always)]
    pub const fn intc_pend_reg1(&self) -> &IntcPendReg1 {
        &self.intc_pend_reg1
    }
    #[doc = "0x20 - Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn intc_en_reg0(&self) -> &IntcEnReg0 {
        &self.intc_en_reg0
    }
    #[doc = "0x24 - Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn intc_en_reg1(&self) -> &IntcEnReg1 {
        &self.intc_en_reg1
    }
    #[doc = "0x30 - Interrupt Mask Register 0"]
    #[inline(always)]
    pub const fn intc_mask_reg0(&self) -> &IntcMaskReg0 {
        &self.intc_mask_reg0
    }
    #[doc = "0x34 - Interrupt Mask Register 1"]
    #[inline(always)]
    pub const fn intc_mask_reg1(&self) -> &IntcMaskReg1 {
        &self.intc_mask_reg1
    }
    #[doc = "0x40 - Interrupt Response Register 0"]
    #[inline(always)]
    pub const fn intc_resp_reg0(&self) -> &IntcRespReg0 {
        &self.intc_resp_reg0
    }
    #[doc = "0x44 - Interrupt Response Register 1"]
    #[inline(always)]
    pub const fn intc_resp_reg1(&self) -> &IntcRespReg1 {
        &self.intc_resp_reg1
    }
    #[doc = "0x50 - Interrupt Fast Forcing Register 0"]
    #[inline(always)]
    pub const fn intc_ff_reg0(&self) -> &IntcFfReg0 {
        &self.intc_ff_reg0
    }
    #[doc = "0x54 - Interrupt Fast Forcing Register 1"]
    #[inline(always)]
    pub const fn intc_ff_reg1(&self) -> &IntcFfReg1 {
        &self.intc_ff_reg1
    }
    #[doc = "0x60 - Interrupt Source Priority Register 0 (IRQ 1-15, 2-bit each, 4 levels)"]
    #[inline(always)]
    pub const fn intc_prio_reg0(&self) -> &IntcPrioReg0 {
        &self.intc_prio_reg0
    }
    #[doc = "0x64 - Interrupt Source Priority Register 1 (IRQ 16-31, 2-bit each, 4 levels)"]
    #[inline(always)]
    pub const fn intc_prio_reg1(&self) -> &IntcPrioReg1 {
        &self.intc_prio_reg1
    }
    #[doc = "0x68 - Interrupt Source Priority Register 2 (IRQ 32-47, 2-bit each, 4 levels)"]
    #[inline(always)]
    pub const fn intc_prio_reg2(&self) -> &IntcPrioReg2 {
        &self.intc_prio_reg2
    }
    #[doc = "0x6c - Interrupt Source Priority Register 3 (IRQ 48-63, 2-bit each, 4 levels)"]
    #[inline(always)]
    pub const fn intc_prio_reg3(&self) -> &IntcPrioReg3 {
        &self.intc_prio_reg3
    }
}
#[doc = "INTC_VECTOR (r) register accessor: Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_vector::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc_vector`] module"]
#[doc(alias = "INTC_VECTOR")]
pub type IntcVector = crate::Reg<intc_vector::IntcVectorSpec>;
#[doc = "Interrupt Vector Register"]
pub mod intc_vector;
#[doc = "INTC_BASE_ADDR (rw) register accessor: Interrupt Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_base_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_base_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc_base_addr`] module"]
#[doc(alias = "INTC_BASE_ADDR")]
pub type IntcBaseAddr = crate::Reg<intc_base_addr::IntcBaseAddrSpec>;
#[doc = "Interrupt Base Address Register"]
pub mod intc_base_addr;
#[doc = "NMI_INT_CTRL (rw) register accessor: NMI Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmi_int_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmi_int_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmi_int_ctrl`] module"]
#[doc(alias = "NMI_INT_CTRL")]
pub type NmiIntCtrl = crate::Reg<nmi_int_ctrl::NmiIntCtrlSpec>;
#[doc = "NMI Interrupt Control Register"]
pub mod nmi_int_ctrl;
#[doc = "INTC_PEND_REG0 (rw) register accessor: Interrupt IRQ Pending Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_pend_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_pend_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc_pend_reg0`] module"]
#[doc(alias = "INTC_PEND_REG0")]
pub type IntcPendReg0 = crate::Reg<intc_pend_reg0::IntcPendReg0Spec>;
#[doc = "Interrupt IRQ Pending Register 0"]
pub mod intc_pend_reg0;
#[doc = "INTC_PEND_REG1 (rw) register accessor: Interrupt IRQ Pending Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_pend_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_pend_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc_pend_reg1`] module"]
#[doc(alias = "INTC_PEND_REG1")]
pub type IntcPendReg1 = crate::Reg<intc_pend_reg1::IntcPendReg1Spec>;
#[doc = "Interrupt IRQ Pending Register 1"]
pub mod intc_pend_reg1;
#[doc = "INTC_EN_REG0 (rw) register accessor: Interrupt Enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_en_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_en_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc_en_reg0`] module"]
#[doc(alias = "INTC_EN_REG0")]
pub type IntcEnReg0 = crate::Reg<intc_en_reg0::IntcEnReg0Spec>;
#[doc = "Interrupt Enable Register 0"]
pub mod intc_en_reg0;
#[doc = "INTC_EN_REG1 (rw) register accessor: Interrupt Enable Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_en_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_en_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc_en_reg1`] module"]
#[doc(alias = "INTC_EN_REG1")]
pub type IntcEnReg1 = crate::Reg<intc_en_reg1::IntcEnReg1Spec>;
#[doc = "Interrupt Enable Register 1"]
pub mod intc_en_reg1;
#[doc = "INTC_MASK_REG0 (rw) register accessor: Interrupt Mask Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_mask_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_mask_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc_mask_reg0`] module"]
#[doc(alias = "INTC_MASK_REG0")]
pub type IntcMaskReg0 = crate::Reg<intc_mask_reg0::IntcMaskReg0Spec>;
#[doc = "Interrupt Mask Register 0"]
pub mod intc_mask_reg0;
#[doc = "INTC_MASK_REG1 (rw) register accessor: Interrupt Mask Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_mask_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_mask_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc_mask_reg1`] module"]
#[doc(alias = "INTC_MASK_REG1")]
pub type IntcMaskReg1 = crate::Reg<intc_mask_reg1::IntcMaskReg1Spec>;
#[doc = "Interrupt Mask Register 1"]
pub mod intc_mask_reg1;
#[doc = "INTC_RESP_REG0 (rw) register accessor: Interrupt Response Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_resp_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_resp_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc_resp_reg0`] module"]
#[doc(alias = "INTC_RESP_REG0")]
pub type IntcRespReg0 = crate::Reg<intc_resp_reg0::IntcRespReg0Spec>;
#[doc = "Interrupt Response Register 0"]
pub mod intc_resp_reg0;
#[doc = "INTC_RESP_REG1 (rw) register accessor: Interrupt Response Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_resp_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_resp_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc_resp_reg1`] module"]
#[doc(alias = "INTC_RESP_REG1")]
pub type IntcRespReg1 = crate::Reg<intc_resp_reg1::IntcRespReg1Spec>;
#[doc = "Interrupt Response Register 1"]
pub mod intc_resp_reg1;
#[doc = "INTC_FF_REG0 (rw) register accessor: Interrupt Fast Forcing Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_ff_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_ff_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc_ff_reg0`] module"]
#[doc(alias = "INTC_FF_REG0")]
pub type IntcFfReg0 = crate::Reg<intc_ff_reg0::IntcFfReg0Spec>;
#[doc = "Interrupt Fast Forcing Register 0"]
pub mod intc_ff_reg0;
#[doc = "INTC_FF_REG1 (rw) register accessor: Interrupt Fast Forcing Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_ff_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_ff_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc_ff_reg1`] module"]
#[doc(alias = "INTC_FF_REG1")]
pub type IntcFfReg1 = crate::Reg<intc_ff_reg1::IntcFfReg1Spec>;
#[doc = "Interrupt Fast Forcing Register 1"]
pub mod intc_ff_reg1;
#[doc = "INTC_PRIO_REG0 (rw) register accessor: Interrupt Source Priority Register 0 (IRQ 1-15, 2-bit each, 4 levels)\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_prio_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_prio_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc_prio_reg0`] module"]
#[doc(alias = "INTC_PRIO_REG0")]
pub type IntcPrioReg0 = crate::Reg<intc_prio_reg0::IntcPrioReg0Spec>;
#[doc = "Interrupt Source Priority Register 0 (IRQ 1-15, 2-bit each, 4 levels)"]
pub mod intc_prio_reg0;
#[doc = "INTC_PRIO_REG1 (rw) register accessor: Interrupt Source Priority Register 1 (IRQ 16-31, 2-bit each, 4 levels)\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_prio_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_prio_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc_prio_reg1`] module"]
#[doc(alias = "INTC_PRIO_REG1")]
pub type IntcPrioReg1 = crate::Reg<intc_prio_reg1::IntcPrioReg1Spec>;
#[doc = "Interrupt Source Priority Register 1 (IRQ 16-31, 2-bit each, 4 levels)"]
pub mod intc_prio_reg1;
#[doc = "INTC_PRIO_REG2 (rw) register accessor: Interrupt Source Priority Register 2 (IRQ 32-47, 2-bit each, 4 levels)\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_prio_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_prio_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc_prio_reg2`] module"]
#[doc(alias = "INTC_PRIO_REG2")]
pub type IntcPrioReg2 = crate::Reg<intc_prio_reg2::IntcPrioReg2Spec>;
#[doc = "Interrupt Source Priority Register 2 (IRQ 32-47, 2-bit each, 4 levels)"]
pub mod intc_prio_reg2;
#[doc = "INTC_PRIO_REG3 (rw) register accessor: Interrupt Source Priority Register 3 (IRQ 48-63, 2-bit each, 4 levels)\n\nYou can [`read`](crate::Reg::read) this register and get [`intc_prio_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intc_prio_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intc_prio_reg3`] module"]
#[doc(alias = "INTC_PRIO_REG3")]
pub type IntcPrioReg3 = crate::Reg<intc_prio_reg3::IntcPrioReg3Spec>;
#[doc = "Interrupt Source Priority Register 3 (IRQ 48-63, 2-bit each, 4 levels)"]
pub mod intc_prio_reg3;
