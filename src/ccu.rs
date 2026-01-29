#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pll_cpu_ctrl: PllCpuCtrl,
    _reserved1: [u8; 0x04],
    pll_audio_ctrl: PllAudioCtrl,
    _reserved2: [u8; 0x04],
    pll_video_ctrl: PllVideoCtrl,
    _reserved3: [u8; 0x04],
    pll_ve_ctrl: PllVeCtrl,
    _reserved4: [u8; 0x04],
    pll_ddr_ctrl: PllDdrCtrl,
    _reserved5: [u8; 0x04],
    pll_periph_ctrl: PllPeriphCtrl,
    _reserved6: [u8; 0x24],
    cpu_clk_src: CpuClkSrc,
    ahb_apb_hclkc_cfg: AhbApbHclkcCfg,
    _reserved8: [u8; 0x08],
    bus_clk_gating0: BusClkGating0,
    bus_clk_gating1: BusClkGating1,
    bus_clk_gating2: BusClkGating2,
    _reserved11: [u8; 0x1c],
    sdmmc0_clk: Sdmmc0Clk,
    sdmmc1_clk: Sdmmc1Clk,
    _reserved13: [u8; 0x20],
    daudio_clk: DaudioClk,
    owa_clk: OwaClk,
    cir_clk: CirClk,
    _reserved16: [u8; 0x10],
    usbphy_clk: UsbphyClk,
    _reserved17: [u8; 0x30],
    dram_gating: DramGating,
    be_clk: BeClk,
    _reserved19: [u8; 0x04],
    fe_clk: FeClk,
    _reserved20: [u8; 0x08],
    tcon_clk: TconClk,
    di_clk: DiClk,
    tve_clk: TveClk,
    tvd_clk: TvdClk,
    _reserved24: [u8; 0x0c],
    csi_clk: CsiClk,
    _reserved25: [u8; 0x04],
    ve_clk: VeClk,
    audio_codec_clk: AudioCodecClk,
    avs_clk: AvsClk,
    _reserved28: [u8; 0xb8],
    pll_stable_time0: PllStableTime0,
    pll_stable_time1: PllStableTime1,
    _reserved30: [u8; 0x18],
    pll_cpu_bias: PllCpuBias,
    pll_audio_bias: PllAudioBias,
    pll_video_bias: PllVideoBias,
    pll_ve_bias: PllVeBias,
    pll_ddr_bias: PllDdrBias,
    pll_periph_bias: PllPeriphBias,
    _reserved36: [u8; 0x18],
    pll_cpu_tun: PllCpuTun,
    _reserved37: [u8; 0x0c],
    pll_ddr_tun: PllDdrTun,
    _reserved38: [u8; 0x20],
    pll_audio_pat_ctrl: PllAudioPatCtrl,
    pll_video_pat_ctrl: PllVideoPatCtrl,
    _reserved40: [u8; 0x04],
    pll_ddr_pat_ctrl: PllDdrPatCtrl,
    _reserved41: [u8; 0x2c],
    bus_soft_rst0: BusSoftRst0,
    bus_soft_rst1: BusSoftRst1,
    _reserved43: [u8; 0x08],
    bus_soft_rst2: BusSoftRst2,
}
impl RegisterBlock {
    #[doc = "0x00 - PLL CPU Control Register"]
    #[inline(always)]
    pub const fn pll_cpu_ctrl(&self) -> &PllCpuCtrl {
        &self.pll_cpu_ctrl
    }
    #[doc = "0x08 - PLL Audio Control Register"]
    #[inline(always)]
    pub const fn pll_audio_ctrl(&self) -> &PllAudioCtrl {
        &self.pll_audio_ctrl
    }
    #[doc = "0x10 - PLL Video Control Register"]
    #[inline(always)]
    pub const fn pll_video_ctrl(&self) -> &PllVideoCtrl {
        &self.pll_video_ctrl
    }
    #[doc = "0x18 - PLL VE Control Register"]
    #[inline(always)]
    pub const fn pll_ve_ctrl(&self) -> &PllVeCtrl {
        &self.pll_ve_ctrl
    }
    #[doc = "0x20 - PLL DDR Control Register"]
    #[inline(always)]
    pub const fn pll_ddr_ctrl(&self) -> &PllDdrCtrl {
        &self.pll_ddr_ctrl
    }
    #[doc = "0x28 - PLL Peripheral Control Register"]
    #[inline(always)]
    pub const fn pll_periph_ctrl(&self) -> &PllPeriphCtrl {
        &self.pll_periph_ctrl
    }
    #[doc = "0x50 - CPU Clock Source Register"]
    #[inline(always)]
    pub const fn cpu_clk_src(&self) -> &CpuClkSrc {
        &self.cpu_clk_src
    }
    #[doc = "0x54 - AHB/APB/HCLKC Configuration Register"]
    #[inline(always)]
    pub const fn ahb_apb_hclkc_cfg(&self) -> &AhbApbHclkcCfg {
        &self.ahb_apb_hclkc_cfg
    }
    #[doc = "0x60 - Bus Clock Gating Register 0"]
    #[inline(always)]
    pub const fn bus_clk_gating0(&self) -> &BusClkGating0 {
        &self.bus_clk_gating0
    }
    #[doc = "0x64 - Bus Clock Gating Register 1"]
    #[inline(always)]
    pub const fn bus_clk_gating1(&self) -> &BusClkGating1 {
        &self.bus_clk_gating1
    }
    #[doc = "0x68 - Bus Clock Gating Register 2"]
    #[inline(always)]
    pub const fn bus_clk_gating2(&self) -> &BusClkGating2 {
        &self.bus_clk_gating2
    }
    #[doc = "0x88 - SDMMC0 Clock Register"]
    #[inline(always)]
    pub const fn sdmmc0_clk(&self) -> &Sdmmc0Clk {
        &self.sdmmc0_clk
    }
    #[doc = "0x8c - SDMMC1 Clock Register"]
    #[inline(always)]
    pub const fn sdmmc1_clk(&self) -> &Sdmmc1Clk {
        &self.sdmmc1_clk
    }
    #[doc = "0xb0 - DAUDIO Clock Register"]
    #[inline(always)]
    pub const fn daudio_clk(&self) -> &DaudioClk {
        &self.daudio_clk
    }
    #[doc = "0xb4 - OWA Clock Register"]
    #[inline(always)]
    pub const fn owa_clk(&self) -> &OwaClk {
        &self.owa_clk
    }
    #[doc = "0xb8 - CIR Clock Register"]
    #[inline(always)]
    pub const fn cir_clk(&self) -> &CirClk {
        &self.cir_clk
    }
    #[doc = "0xcc - USBPHY Clock Register"]
    #[inline(always)]
    pub const fn usbphy_clk(&self) -> &UsbphyClk {
        &self.usbphy_clk
    }
    #[doc = "0x100 - DRAM GATING Register"]
    #[inline(always)]
    pub const fn dram_gating(&self) -> &DramGating {
        &self.dram_gating
    }
    #[doc = "0x104 - BE Clock Register"]
    #[inline(always)]
    pub const fn be_clk(&self) -> &BeClk {
        &self.be_clk
    }
    #[doc = "0x10c - FE Clock Register"]
    #[inline(always)]
    pub const fn fe_clk(&self) -> &FeClk {
        &self.fe_clk
    }
    #[doc = "0x118 - TCON Clock Register"]
    #[inline(always)]
    pub const fn tcon_clk(&self) -> &TconClk {
        &self.tcon_clk
    }
    #[doc = "0x11c - De-interlacer Clock Register"]
    #[inline(always)]
    pub const fn di_clk(&self) -> &DiClk {
        &self.di_clk
    }
    #[doc = "0x120 - TVE Clock Register"]
    #[inline(always)]
    pub const fn tve_clk(&self) -> &TveClk {
        &self.tve_clk
    }
    #[doc = "0x124 - TVD Clock Register"]
    #[inline(always)]
    pub const fn tvd_clk(&self) -> &TvdClk {
        &self.tvd_clk
    }
    #[doc = "0x134 - CSI Clock Register"]
    #[inline(always)]
    pub const fn csi_clk(&self) -> &CsiClk {
        &self.csi_clk
    }
    #[doc = "0x13c - VE Clock Register"]
    #[inline(always)]
    pub const fn ve_clk(&self) -> &VeClk {
        &self.ve_clk
    }
    #[doc = "0x140 - Audio Codec Clock Register"]
    #[inline(always)]
    pub const fn audio_codec_clk(&self) -> &AudioCodecClk {
        &self.audio_codec_clk
    }
    #[doc = "0x144 - AVS Clock Register"]
    #[inline(always)]
    pub const fn avs_clk(&self) -> &AvsClk {
        &self.avs_clk
    }
    #[doc = "0x200 - PLL Stable Time Register 0"]
    #[inline(always)]
    pub const fn pll_stable_time0(&self) -> &PllStableTime0 {
        &self.pll_stable_time0
    }
    #[doc = "0x204 - PLL Stable Time Register 1"]
    #[inline(always)]
    pub const fn pll_stable_time1(&self) -> &PllStableTime1 {
        &self.pll_stable_time1
    }
    #[doc = "0x220 - PLL CPU Bias Register"]
    #[inline(always)]
    pub const fn pll_cpu_bias(&self) -> &PllCpuBias {
        &self.pll_cpu_bias
    }
    #[doc = "0x224 - PLL Audio Bias Register"]
    #[inline(always)]
    pub const fn pll_audio_bias(&self) -> &PllAudioBias {
        &self.pll_audio_bias
    }
    #[doc = "0x228 - PLL Video Bias Register"]
    #[inline(always)]
    pub const fn pll_video_bias(&self) -> &PllVideoBias {
        &self.pll_video_bias
    }
    #[doc = "0x22c - PLL VE Bias Register"]
    #[inline(always)]
    pub const fn pll_ve_bias(&self) -> &PllVeBias {
        &self.pll_ve_bias
    }
    #[doc = "0x230 - PLL DDR Bias Register"]
    #[inline(always)]
    pub const fn pll_ddr_bias(&self) -> &PllDdrBias {
        &self.pll_ddr_bias
    }
    #[doc = "0x234 - PLL Peripheral Bias Register"]
    #[inline(always)]
    pub const fn pll_periph_bias(&self) -> &PllPeriphBias {
        &self.pll_periph_bias
    }
    #[doc = "0x250 - PLL CPU Tuning Register"]
    #[inline(always)]
    pub const fn pll_cpu_tun(&self) -> &PllCpuTun {
        &self.pll_cpu_tun
    }
    #[doc = "0x260 - PLL DDR Tuning Register"]
    #[inline(always)]
    pub const fn pll_ddr_tun(&self) -> &PllDdrTun {
        &self.pll_ddr_tun
    }
    #[doc = "0x284 - PLL Audio Pattern Control Register"]
    #[inline(always)]
    pub const fn pll_audio_pat_ctrl(&self) -> &PllAudioPatCtrl {
        &self.pll_audio_pat_ctrl
    }
    #[doc = "0x288 - PLL Video Pattern Control Register"]
    #[inline(always)]
    pub const fn pll_video_pat_ctrl(&self) -> &PllVideoPatCtrl {
        &self.pll_video_pat_ctrl
    }
    #[doc = "0x290 - PLL DDR Pattern Control Register"]
    #[inline(always)]
    pub const fn pll_ddr_pat_ctrl(&self) -> &PllDdrPatCtrl {
        &self.pll_ddr_pat_ctrl
    }
    #[doc = "0x2c0 - Bus Software Reset Register 0"]
    #[inline(always)]
    pub const fn bus_soft_rst0(&self) -> &BusSoftRst0 {
        &self.bus_soft_rst0
    }
    #[doc = "0x2c4 - Bus Software Reset Register 1"]
    #[inline(always)]
    pub const fn bus_soft_rst1(&self) -> &BusSoftRst1 {
        &self.bus_soft_rst1
    }
    #[doc = "0x2d0 - Bus Software Reset Register 2"]
    #[inline(always)]
    pub const fn bus_soft_rst2(&self) -> &BusSoftRst2 {
        &self.bus_soft_rst2
    }
}
#[doc = "PLL_CPU_CTRL (rw) register accessor: PLL CPU Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_cpu_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cpu_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_cpu_ctrl`] module"]
#[doc(alias = "PLL_CPU_CTRL")]
pub type PllCpuCtrl = crate::Reg<pll_cpu_ctrl::PllCpuCtrlSpec>;
#[doc = "PLL CPU Control Register"]
pub mod pll_cpu_ctrl;
#[doc = "PLL_AUDIO_CTRL (rw) register accessor: PLL Audio Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_audio_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_audio_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_audio_ctrl`] module"]
#[doc(alias = "PLL_AUDIO_CTRL")]
pub type PllAudioCtrl = crate::Reg<pll_audio_ctrl::PllAudioCtrlSpec>;
#[doc = "PLL Audio Control Register"]
pub mod pll_audio_ctrl;
#[doc = "PLL_VIDEO_CTRL (rw) register accessor: PLL Video Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_video_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_video_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_video_ctrl`] module"]
#[doc(alias = "PLL_VIDEO_CTRL")]
pub type PllVideoCtrl = crate::Reg<pll_video_ctrl::PllVideoCtrlSpec>;
#[doc = "PLL Video Control Register"]
pub mod pll_video_ctrl;
#[doc = "PLL_VE_CTRL (rw) register accessor: PLL VE Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_ve_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_ve_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_ve_ctrl`] module"]
#[doc(alias = "PLL_VE_CTRL")]
pub type PllVeCtrl = crate::Reg<pll_ve_ctrl::PllVeCtrlSpec>;
#[doc = "PLL VE Control Register"]
pub mod pll_ve_ctrl;
#[doc = "PLL_DDR_CTRL (rw) register accessor: PLL DDR Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_ddr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_ddr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_ddr_ctrl`] module"]
#[doc(alias = "PLL_DDR_CTRL")]
pub type PllDdrCtrl = crate::Reg<pll_ddr_ctrl::PllDdrCtrlSpec>;
#[doc = "PLL DDR Control Register"]
pub mod pll_ddr_ctrl;
#[doc = "PLL_PERIPH_CTRL (rw) register accessor: PLL Peripheral Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_periph_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_periph_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_periph_ctrl`] module"]
#[doc(alias = "PLL_PERIPH_CTRL")]
pub type PllPeriphCtrl = crate::Reg<pll_periph_ctrl::PllPeriphCtrlSpec>;
#[doc = "PLL Peripheral Control Register"]
pub mod pll_periph_ctrl;
#[doc = "CPU_CLK_SRC (rw) register accessor: CPU Clock Source Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_clk_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_clk_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_clk_src`] module"]
#[doc(alias = "CPU_CLK_SRC")]
pub type CpuClkSrc = crate::Reg<cpu_clk_src::CpuClkSrcSpec>;
#[doc = "CPU Clock Source Register"]
pub mod cpu_clk_src;
#[doc = "AHB_APB_HCLKC_CFG (rw) register accessor: AHB/APB/HCLKC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_apb_hclkc_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_apb_hclkc_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_apb_hclkc_cfg`] module"]
#[doc(alias = "AHB_APB_HCLKC_CFG")]
pub type AhbApbHclkcCfg = crate::Reg<ahb_apb_hclkc_cfg::AhbApbHclkcCfgSpec>;
#[doc = "AHB/APB/HCLKC Configuration Register"]
pub mod ahb_apb_hclkc_cfg;
#[doc = "BUS_CLK_GATING0 (rw) register accessor: Bus Clock Gating Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_clk_gating0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_clk_gating0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_clk_gating0`] module"]
#[doc(alias = "BUS_CLK_GATING0")]
pub type BusClkGating0 = crate::Reg<bus_clk_gating0::BusClkGating0Spec>;
#[doc = "Bus Clock Gating Register 0"]
pub mod bus_clk_gating0;
#[doc = "BUS_CLK_GATING1 (rw) register accessor: Bus Clock Gating Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_clk_gating1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_clk_gating1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_clk_gating1`] module"]
#[doc(alias = "BUS_CLK_GATING1")]
pub type BusClkGating1 = crate::Reg<bus_clk_gating1::BusClkGating1Spec>;
#[doc = "Bus Clock Gating Register 1"]
pub mod bus_clk_gating1;
#[doc = "BUS_CLK_GATING2 (rw) register accessor: Bus Clock Gating Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_clk_gating2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_clk_gating2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_clk_gating2`] module"]
#[doc(alias = "BUS_CLK_GATING2")]
pub type BusClkGating2 = crate::Reg<bus_clk_gating2::BusClkGating2Spec>;
#[doc = "Bus Clock Gating Register 2"]
pub mod bus_clk_gating2;
#[doc = "SDMMC0_CLK (rw) register accessor: SDMMC0 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdmmc0_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc0_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc0_clk`] module"]
#[doc(alias = "SDMMC0_CLK")]
pub type Sdmmc0Clk = crate::Reg<sdmmc0_clk::Sdmmc0ClkSpec>;
#[doc = "SDMMC0 Clock Register"]
pub mod sdmmc0_clk;
#[doc = "SDMMC1_CLK (rw) register accessor: SDMMC1 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdmmc1_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc1_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmmc1_clk`] module"]
#[doc(alias = "SDMMC1_CLK")]
pub type Sdmmc1Clk = crate::Reg<sdmmc1_clk::Sdmmc1ClkSpec>;
#[doc = "SDMMC1 Clock Register"]
pub mod sdmmc1_clk;
#[doc = "DAUDIO_CLK (rw) register accessor: DAUDIO Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`daudio_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daudio_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daudio_clk`] module"]
#[doc(alias = "DAUDIO_CLK")]
pub type DaudioClk = crate::Reg<daudio_clk::DaudioClkSpec>;
#[doc = "DAUDIO Clock Register"]
pub mod daudio_clk;
#[doc = "OWA_CLK (rw) register accessor: OWA Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`owa_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`owa_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@owa_clk`] module"]
#[doc(alias = "OWA_CLK")]
pub type OwaClk = crate::Reg<owa_clk::OwaClkSpec>;
#[doc = "OWA Clock Register"]
pub mod owa_clk;
#[doc = "CIR_CLK (rw) register accessor: CIR Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cir_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cir_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cir_clk`] module"]
#[doc(alias = "CIR_CLK")]
pub type CirClk = crate::Reg<cir_clk::CirClkSpec>;
#[doc = "CIR Clock Register"]
pub mod cir_clk;
#[doc = "USBPHY_CLK (rw) register accessor: USBPHY Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbphy_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphy_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbphy_clk`] module"]
#[doc(alias = "USBPHY_CLK")]
pub type UsbphyClk = crate::Reg<usbphy_clk::UsbphyClkSpec>;
#[doc = "USBPHY Clock Register"]
pub mod usbphy_clk;
#[doc = "DRAM_GATING (rw) register accessor: DRAM GATING Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dram_gating::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dram_gating::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dram_gating`] module"]
#[doc(alias = "DRAM_GATING")]
pub type DramGating = crate::Reg<dram_gating::DramGatingSpec>;
#[doc = "DRAM GATING Register"]
pub mod dram_gating;
#[doc = "BE_CLK (rw) register accessor: BE Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`be_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`be_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@be_clk`] module"]
#[doc(alias = "BE_CLK")]
pub type BeClk = crate::Reg<be_clk::BeClkSpec>;
#[doc = "BE Clock Register"]
pub mod be_clk;
#[doc = "FE_CLK (rw) register accessor: FE Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fe_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fe_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fe_clk`] module"]
#[doc(alias = "FE_CLK")]
pub type FeClk = crate::Reg<fe_clk::FeClkSpec>;
#[doc = "FE Clock Register"]
pub mod fe_clk;
#[doc = "TCON_CLK (rw) register accessor: TCON Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcon_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcon_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcon_clk`] module"]
#[doc(alias = "TCON_CLK")]
pub type TconClk = crate::Reg<tcon_clk::TconClkSpec>;
#[doc = "TCON Clock Register"]
pub mod tcon_clk;
#[doc = "DI_CLK (rw) register accessor: De-interlacer Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`di_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`di_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@di_clk`] module"]
#[doc(alias = "DI_CLK")]
pub type DiClk = crate::Reg<di_clk::DiClkSpec>;
#[doc = "De-interlacer Clock Register"]
pub mod di_clk;
#[doc = "TVE_CLK (rw) register accessor: TVE Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tve_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tve_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tve_clk`] module"]
#[doc(alias = "TVE_CLK")]
pub type TveClk = crate::Reg<tve_clk::TveClkSpec>;
#[doc = "TVE Clock Register"]
pub mod tve_clk;
#[doc = "TVD_CLK (rw) register accessor: TVD Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tvd_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tvd_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tvd_clk`] module"]
#[doc(alias = "TVD_CLK")]
pub type TvdClk = crate::Reg<tvd_clk::TvdClkSpec>;
#[doc = "TVD Clock Register"]
pub mod tvd_clk;
#[doc = "CSI_CLK (rw) register accessor: CSI Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csi_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi_clk`] module"]
#[doc(alias = "CSI_CLK")]
pub type CsiClk = crate::Reg<csi_clk::CsiClkSpec>;
#[doc = "CSI Clock Register"]
pub mod csi_clk;
#[doc = "VE_CLK (rw) register accessor: VE Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ve_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ve_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ve_clk`] module"]
#[doc(alias = "VE_CLK")]
pub type VeClk = crate::Reg<ve_clk::VeClkSpec>;
#[doc = "VE Clock Register"]
pub mod ve_clk;
#[doc = "AUDIO_CODEC_CLK (rw) register accessor: Audio Codec Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_codec_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_codec_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_codec_clk`] module"]
#[doc(alias = "AUDIO_CODEC_CLK")]
pub type AudioCodecClk = crate::Reg<audio_codec_clk::AudioCodecClkSpec>;
#[doc = "Audio Codec Clock Register"]
pub mod audio_codec_clk;
#[doc = "AVS_CLK (rw) register accessor: AVS Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`avs_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`avs_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@avs_clk`] module"]
#[doc(alias = "AVS_CLK")]
pub type AvsClk = crate::Reg<avs_clk::AvsClkSpec>;
#[doc = "AVS Clock Register"]
pub mod avs_clk;
#[doc = "PLL_STABLE_TIME0 (rw) register accessor: PLL Stable Time Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_stable_time0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_stable_time0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_stable_time0`] module"]
#[doc(alias = "PLL_STABLE_TIME0")]
pub type PllStableTime0 = crate::Reg<pll_stable_time0::PllStableTime0Spec>;
#[doc = "PLL Stable Time Register 0"]
pub mod pll_stable_time0;
#[doc = "PLL_STABLE_TIME1 (rw) register accessor: PLL Stable Time Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_stable_time1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_stable_time1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_stable_time1`] module"]
#[doc(alias = "PLL_STABLE_TIME1")]
pub type PllStableTime1 = crate::Reg<pll_stable_time1::PllStableTime1Spec>;
#[doc = "PLL Stable Time Register 1"]
pub mod pll_stable_time1;
#[doc = "PLL_CPU_BIAS (rw) register accessor: PLL CPU Bias Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_cpu_bias::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cpu_bias::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_cpu_bias`] module"]
#[doc(alias = "PLL_CPU_BIAS")]
pub type PllCpuBias = crate::Reg<pll_cpu_bias::PllCpuBiasSpec>;
#[doc = "PLL CPU Bias Register"]
pub mod pll_cpu_bias;
#[doc = "PLL_AUDIO_BIAS (rw) register accessor: PLL Audio Bias Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_audio_bias::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_audio_bias::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_audio_bias`] module"]
#[doc(alias = "PLL_AUDIO_BIAS")]
pub type PllAudioBias = crate::Reg<pll_audio_bias::PllAudioBiasSpec>;
#[doc = "PLL Audio Bias Register"]
pub mod pll_audio_bias;
#[doc = "PLL_VIDEO_BIAS (rw) register accessor: PLL Video Bias Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_video_bias::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_video_bias::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_video_bias`] module"]
#[doc(alias = "PLL_VIDEO_BIAS")]
pub type PllVideoBias = crate::Reg<pll_video_bias::PllVideoBiasSpec>;
#[doc = "PLL Video Bias Register"]
pub mod pll_video_bias;
#[doc = "PLL_VE_BIAS (rw) register accessor: PLL VE Bias Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_ve_bias::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_ve_bias::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_ve_bias`] module"]
#[doc(alias = "PLL_VE_BIAS")]
pub type PllVeBias = crate::Reg<pll_ve_bias::PllVeBiasSpec>;
#[doc = "PLL VE Bias Register"]
pub mod pll_ve_bias;
#[doc = "PLL_DDR_BIAS (rw) register accessor: PLL DDR Bias Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_ddr_bias::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_ddr_bias::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_ddr_bias`] module"]
#[doc(alias = "PLL_DDR_BIAS")]
pub type PllDdrBias = crate::Reg<pll_ddr_bias::PllDdrBiasSpec>;
#[doc = "PLL DDR Bias Register"]
pub mod pll_ddr_bias;
#[doc = "PLL_PERIPH_BIAS (rw) register accessor: PLL Peripheral Bias Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_periph_bias::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_periph_bias::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_periph_bias`] module"]
#[doc(alias = "PLL_PERIPH_BIAS")]
pub type PllPeriphBias = crate::Reg<pll_periph_bias::PllPeriphBiasSpec>;
#[doc = "PLL Peripheral Bias Register"]
pub mod pll_periph_bias;
#[doc = "PLL_CPU_TUN (rw) register accessor: PLL CPU Tuning Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_cpu_tun::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cpu_tun::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_cpu_tun`] module"]
#[doc(alias = "PLL_CPU_TUN")]
pub type PllCpuTun = crate::Reg<pll_cpu_tun::PllCpuTunSpec>;
#[doc = "PLL CPU Tuning Register"]
pub mod pll_cpu_tun;
#[doc = "PLL_DDR_TUN (rw) register accessor: PLL DDR Tuning Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_ddr_tun::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_ddr_tun::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_ddr_tun`] module"]
#[doc(alias = "PLL_DDR_TUN")]
pub type PllDdrTun = crate::Reg<pll_ddr_tun::PllDdrTunSpec>;
#[doc = "PLL DDR Tuning Register"]
pub mod pll_ddr_tun;
#[doc = "PLL_AUDIO_PAT_CTRL (rw) register accessor: PLL Audio Pattern Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_audio_pat_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_audio_pat_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_audio_pat_ctrl`] module"]
#[doc(alias = "PLL_AUDIO_PAT_CTRL")]
pub type PllAudioPatCtrl = crate::Reg<pll_audio_pat_ctrl::PllAudioPatCtrlSpec>;
#[doc = "PLL Audio Pattern Control Register"]
pub mod pll_audio_pat_ctrl;
#[doc = "PLL_VIDEO_PAT_CTRL (rw) register accessor: PLL Video Pattern Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_video_pat_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_video_pat_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_video_pat_ctrl`] module"]
#[doc(alias = "PLL_VIDEO_PAT_CTRL")]
pub type PllVideoPatCtrl = crate::Reg<pll_video_pat_ctrl::PllVideoPatCtrlSpec>;
#[doc = "PLL Video Pattern Control Register"]
pub mod pll_video_pat_ctrl;
#[doc = "PLL_DDR_PAT_CTRL (rw) register accessor: PLL DDR Pattern Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_ddr_pat_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_ddr_pat_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_ddr_pat_ctrl`] module"]
#[doc(alias = "PLL_DDR_PAT_CTRL")]
pub type PllDdrPatCtrl = crate::Reg<pll_ddr_pat_ctrl::PllDdrPatCtrlSpec>;
#[doc = "PLL DDR Pattern Control Register"]
pub mod pll_ddr_pat_ctrl;
#[doc = "BUS_SOFT_RST0 (rw) register accessor: Bus Software Reset Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_soft_rst0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_soft_rst0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_soft_rst0`] module"]
#[doc(alias = "BUS_SOFT_RST0")]
pub type BusSoftRst0 = crate::Reg<bus_soft_rst0::BusSoftRst0Spec>;
#[doc = "Bus Software Reset Register 0"]
pub mod bus_soft_rst0;
#[doc = "BUS_SOFT_RST1 (rw) register accessor: Bus Software Reset Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_soft_rst1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_soft_rst1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_soft_rst1`] module"]
#[doc(alias = "BUS_SOFT_RST1")]
pub type BusSoftRst1 = crate::Reg<bus_soft_rst1::BusSoftRst1Spec>;
#[doc = "Bus Software Reset Register 1"]
pub mod bus_soft_rst1;
#[doc = "BUS_SOFT_RST2 (rw) register accessor: Bus Software Reset Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_soft_rst2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_soft_rst2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_soft_rst2`] module"]
#[doc(alias = "BUS_SOFT_RST2")]
pub type BusSoftRst2 = crate::Reg<bus_soft_rst2::BusSoftRst2Spec>;
#[doc = "Bus Software Reset Register 2"]
pub mod bus_soft_rst2;
