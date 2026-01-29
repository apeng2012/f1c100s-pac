#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tmr_irq_en: TmrIrqEn,
    tmr_irq_sta: TmrIrqSta,
    _reserved2: [u8; 0x08],
    tmr0_ctrl: Tmr0Ctrl,
    tmr0_intv: Tmr0Intv,
    tmr0_cur: Tmr0Cur,
    _reserved5: [u8; 0x04],
    tmr1_ctrl: Tmr1Ctrl,
    tmr1_intv: Tmr1Intv,
    tmr1_cur: Tmr1Cur,
    _reserved8: [u8; 0x04],
    tmr2_ctrl: Tmr2Ctrl,
    tmr2_intv: Tmr2Intv,
    tmr2_cur: Tmr2Cur,
    _reserved11: [u8; 0x44],
    avs_cnt_ctl: AvsCntCtl,
    avs_cnt0: AvsCnt0,
    avs_cnt1: AvsCnt1,
    avs_cnt_div: AvsCntDiv,
    _reserved15: [u8; 0x10],
    wdog_irq_en: WdogIrqEn,
    wdog_irq_sta: WdogIrqSta,
    _reserved17: [u8; 0x08],
    wdog_ctrl: WdogCtrl,
    wdog_cfg: WdogCfg,
    wdog_mode: WdogMode,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer IRQ Enable Register"]
    #[inline(always)]
    pub const fn tmr_irq_en(&self) -> &TmrIrqEn {
        &self.tmr_irq_en
    }
    #[doc = "0x04 - Timer Status Register"]
    #[inline(always)]
    pub const fn tmr_irq_sta(&self) -> &TmrIrqSta {
        &self.tmr_irq_sta
    }
    #[doc = "0x10 - Timer 0 Control Register"]
    #[inline(always)]
    pub const fn tmr0_ctrl(&self) -> &Tmr0Ctrl {
        &self.tmr0_ctrl
    }
    #[doc = "0x14 - Timer 0 Interval Value Register"]
    #[inline(always)]
    pub const fn tmr0_intv(&self) -> &Tmr0Intv {
        &self.tmr0_intv
    }
    #[doc = "0x18 - Timer 0 Current Value Register"]
    #[inline(always)]
    pub const fn tmr0_cur(&self) -> &Tmr0Cur {
        &self.tmr0_cur
    }
    #[doc = "0x20 - Timer 1 Control Register"]
    #[inline(always)]
    pub const fn tmr1_ctrl(&self) -> &Tmr1Ctrl {
        &self.tmr1_ctrl
    }
    #[doc = "0x24 - Timer 1 Interval Value Register"]
    #[inline(always)]
    pub const fn tmr1_intv(&self) -> &Tmr1Intv {
        &self.tmr1_intv
    }
    #[doc = "0x28 - Timer 1 Current Value Register"]
    #[inline(always)]
    pub const fn tmr1_cur(&self) -> &Tmr1Cur {
        &self.tmr1_cur
    }
    #[doc = "0x30 - Timer 2 Control Register"]
    #[inline(always)]
    pub const fn tmr2_ctrl(&self) -> &Tmr2Ctrl {
        &self.tmr2_ctrl
    }
    #[doc = "0x34 - Timer 2 Interval Value Register"]
    #[inline(always)]
    pub const fn tmr2_intv(&self) -> &Tmr2Intv {
        &self.tmr2_intv
    }
    #[doc = "0x38 - Timer 2 Current Value Register"]
    #[inline(always)]
    pub const fn tmr2_cur(&self) -> &Tmr2Cur {
        &self.tmr2_cur
    }
    #[doc = "0x80 - AVS Counter Control Register"]
    #[inline(always)]
    pub const fn avs_cnt_ctl(&self) -> &AvsCntCtl {
        &self.avs_cnt_ctl
    }
    #[doc = "0x84 - AVS Counter 0 Register"]
    #[inline(always)]
    pub const fn avs_cnt0(&self) -> &AvsCnt0 {
        &self.avs_cnt0
    }
    #[doc = "0x88 - AVS Counter 1 Register"]
    #[inline(always)]
    pub const fn avs_cnt1(&self) -> &AvsCnt1 {
        &self.avs_cnt1
    }
    #[doc = "0x8c - AVS Counter Divisor Register"]
    #[inline(always)]
    pub const fn avs_cnt_div(&self) -> &AvsCntDiv {
        &self.avs_cnt_div
    }
    #[doc = "0xa0 - Watchdog IRQ Enable Register"]
    #[inline(always)]
    pub const fn wdog_irq_en(&self) -> &WdogIrqEn {
        &self.wdog_irq_en
    }
    #[doc = "0xa4 - Watchdog Status Register"]
    #[inline(always)]
    pub const fn wdog_irq_sta(&self) -> &WdogIrqSta {
        &self.wdog_irq_sta
    }
    #[doc = "0xb0 - Watchdog Control Register"]
    #[inline(always)]
    pub const fn wdog_ctrl(&self) -> &WdogCtrl {
        &self.wdog_ctrl
    }
    #[doc = "0xb4 - Watchdog Configuration Register"]
    #[inline(always)]
    pub const fn wdog_cfg(&self) -> &WdogCfg {
        &self.wdog_cfg
    }
    #[doc = "0xb8 - Watchdog Mode Register"]
    #[inline(always)]
    pub const fn wdog_mode(&self) -> &WdogMode {
        &self.wdog_mode
    }
}
#[doc = "TMR_IRQ_EN (rw) register accessor: Timer IRQ Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr_irq_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr_irq_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr_irq_en`] module"]
#[doc(alias = "TMR_IRQ_EN")]
pub type TmrIrqEn = crate::Reg<tmr_irq_en::TmrIrqEnSpec>;
#[doc = "Timer IRQ Enable Register"]
pub mod tmr_irq_en;
#[doc = "TMR_IRQ_STA (rw) register accessor: Timer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr_irq_sta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr_irq_sta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr_irq_sta`] module"]
#[doc(alias = "TMR_IRQ_STA")]
pub type TmrIrqSta = crate::Reg<tmr_irq_sta::TmrIrqStaSpec>;
#[doc = "Timer Status Register"]
pub mod tmr_irq_sta;
#[doc = "TMR0_CTRL (rw) register accessor: Timer 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr0_ctrl`] module"]
#[doc(alias = "TMR0_CTRL")]
pub type Tmr0Ctrl = crate::Reg<tmr0_ctrl::Tmr0CtrlSpec>;
#[doc = "Timer 0 Control Register"]
pub mod tmr0_ctrl;
#[doc = "TMR0_INTV (rw) register accessor: Timer 0 Interval Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr0_intv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr0_intv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr0_intv`] module"]
#[doc(alias = "TMR0_INTV")]
pub type Tmr0Intv = crate::Reg<tmr0_intv::Tmr0IntvSpec>;
#[doc = "Timer 0 Interval Value Register"]
pub mod tmr0_intv;
#[doc = "TMR0_CUR (r) register accessor: Timer 0 Current Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr0_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr0_cur`] module"]
#[doc(alias = "TMR0_CUR")]
pub type Tmr0Cur = crate::Reg<tmr0_cur::Tmr0CurSpec>;
#[doc = "Timer 0 Current Value Register"]
pub mod tmr0_cur;
#[doc = "TMR1_CTRL (rw) register accessor: Timer 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr1_ctrl`] module"]
#[doc(alias = "TMR1_CTRL")]
pub type Tmr1Ctrl = crate::Reg<tmr1_ctrl::Tmr1CtrlSpec>;
#[doc = "Timer 1 Control Register"]
pub mod tmr1_ctrl;
#[doc = "TMR1_INTV (rw) register accessor: Timer 1 Interval Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr1_intv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr1_intv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr1_intv`] module"]
#[doc(alias = "TMR1_INTV")]
pub type Tmr1Intv = crate::Reg<tmr1_intv::Tmr1IntvSpec>;
#[doc = "Timer 1 Interval Value Register"]
pub mod tmr1_intv;
#[doc = "TMR1_CUR (r) register accessor: Timer 1 Current Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr1_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr1_cur`] module"]
#[doc(alias = "TMR1_CUR")]
pub type Tmr1Cur = crate::Reg<tmr1_cur::Tmr1CurSpec>;
#[doc = "Timer 1 Current Value Register"]
pub mod tmr1_cur;
#[doc = "TMR2_CTRL (rw) register accessor: Timer 2 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr2_ctrl`] module"]
#[doc(alias = "TMR2_CTRL")]
pub type Tmr2Ctrl = crate::Reg<tmr2_ctrl::Tmr2CtrlSpec>;
#[doc = "Timer 2 Control Register"]
pub mod tmr2_ctrl;
#[doc = "TMR2_INTV (rw) register accessor: Timer 2 Interval Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr2_intv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr2_intv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr2_intv`] module"]
#[doc(alias = "TMR2_INTV")]
pub type Tmr2Intv = crate::Reg<tmr2_intv::Tmr2IntvSpec>;
#[doc = "Timer 2 Interval Value Register"]
pub mod tmr2_intv;
#[doc = "TMR2_CUR (r) register accessor: Timer 2 Current Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr2_cur::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr2_cur`] module"]
#[doc(alias = "TMR2_CUR")]
pub type Tmr2Cur = crate::Reg<tmr2_cur::Tmr2CurSpec>;
#[doc = "Timer 2 Current Value Register"]
pub mod tmr2_cur;
#[doc = "AVS_CNT_CTL (rw) register accessor: AVS Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`avs_cnt_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`avs_cnt_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@avs_cnt_ctl`] module"]
#[doc(alias = "AVS_CNT_CTL")]
pub type AvsCntCtl = crate::Reg<avs_cnt_ctl::AvsCntCtlSpec>;
#[doc = "AVS Counter Control Register"]
pub mod avs_cnt_ctl;
#[doc = "AVS_CNT0 (rw) register accessor: AVS Counter 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`avs_cnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`avs_cnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@avs_cnt0`] module"]
#[doc(alias = "AVS_CNT0")]
pub type AvsCnt0 = crate::Reg<avs_cnt0::AvsCnt0Spec>;
#[doc = "AVS Counter 0 Register"]
pub mod avs_cnt0;
#[doc = "AVS_CNT1 (rw) register accessor: AVS Counter 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`avs_cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`avs_cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@avs_cnt1`] module"]
#[doc(alias = "AVS_CNT1")]
pub type AvsCnt1 = crate::Reg<avs_cnt1::AvsCnt1Spec>;
#[doc = "AVS Counter 1 Register"]
pub mod avs_cnt1;
#[doc = "AVS_CNT_DIV (rw) register accessor: AVS Counter Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`avs_cnt_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`avs_cnt_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@avs_cnt_div`] module"]
#[doc(alias = "AVS_CNT_DIV")]
pub type AvsCntDiv = crate::Reg<avs_cnt_div::AvsCntDivSpec>;
#[doc = "AVS Counter Divisor Register"]
pub mod avs_cnt_div;
#[doc = "WDOG_IRQ_EN (rw) register accessor: Watchdog IRQ Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdog_irq_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdog_irq_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdog_irq_en`] module"]
#[doc(alias = "WDOG_IRQ_EN")]
pub type WdogIrqEn = crate::Reg<wdog_irq_en::WdogIrqEnSpec>;
#[doc = "Watchdog IRQ Enable Register"]
pub mod wdog_irq_en;
#[doc = "WDOG_IRQ_STA (rw) register accessor: Watchdog Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdog_irq_sta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdog_irq_sta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdog_irq_sta`] module"]
#[doc(alias = "WDOG_IRQ_STA")]
pub type WdogIrqSta = crate::Reg<wdog_irq_sta::WdogIrqStaSpec>;
#[doc = "Watchdog Status Register"]
pub mod wdog_irq_sta;
#[doc = "WDOG_CTRL (rw) register accessor: Watchdog Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdog_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdog_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdog_ctrl`] module"]
#[doc(alias = "WDOG_CTRL")]
pub type WdogCtrl = crate::Reg<wdog_ctrl::WdogCtrlSpec>;
#[doc = "Watchdog Control Register"]
pub mod wdog_ctrl;
#[doc = "WDOG_CFG (rw) register accessor: Watchdog Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdog_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdog_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdog_cfg`] module"]
#[doc(alias = "WDOG_CFG")]
pub type WdogCfg = crate::Reg<wdog_cfg::WdogCfgSpec>;
#[doc = "Watchdog Configuration Register"]
pub mod wdog_cfg;
#[doc = "WDOG_MODE (rw) register accessor: Watchdog Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdog_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdog_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdog_mode`] module"]
#[doc(alias = "WDOG_MODE")]
pub type WdogMode = crate::Reg<wdog_mode::WdogModeSpec>;
#[doc = "Watchdog Mode Register"]
pub mod wdog_mode;
