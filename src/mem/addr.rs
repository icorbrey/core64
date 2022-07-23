const VI_BASE: usize = 0x0440_0000;

pub const VI_STATUS_REG: *mut u32 = (VI_BASE + 0x00) as *mut u32;
pub const VI_DRAM_ADDR_REG: *mut u32 = (VI_BASE + 0x04) as *mut u32;
pub const VI_H_WIDTH_REG: *mut u32 = (VI_BASE + 0x08) as *mut u32;
pub const VI_V_INTR_REG: *mut u32 = (VI_BASE + 0x0C) as *mut u32;
pub const VI_V_CURRENT_LINE_REG: *mut u32 = (VI_BASE + 0x10) as *mut u32;
pub const VI_TIMING_REG: *mut u32 = (VI_BASE + 0x14) as *mut u32;
pub const VI_V_SYNC_REG: *mut u32 = (VI_BASE + 0x18) as *mut u32;
pub const VI_H_SYNC_REG: *mut u32 = (VI_BASE + 0x1C) as *mut u32;
pub const VI_H_SYNC_LEAP_REG: *mut u32 = (VI_BASE + 0x20) as *mut u32;
pub const VI_H_VIDEO_REG: *mut u32 = (VI_BASE + 0x24) as *mut u32;
pub const VI_V_VIDEO_REG: *mut u32 = (VI_BASE + 0x28) as *mut u32;
pub const VI_V_BURST_REG: *mut u32 = (VI_BASE + 0x2C) as *mut u32;
pub const VI_X_SCALE_REG: *mut u32 = (VI_BASE + 0x30) as *mut u32;
pub const VI_Y_SCALE_REG: *mut u32 = (VI_BASE + 0x34) as *mut u32;

#[allow(dead_code)]
mod unused {
    pub const RDRAM_0: usize = 0x0000_0000;
    pub const RDRAM_1: usize = 0x0020_0000;
    pub const RDRAM_2: usize = 0x0040_0000;

    const RDRAM_BASE: usize = 0x03F0_0000;

    pub const RDRAM_DEVICE_TYPE_REG: *mut u32 = (RDRAM_BASE + 0x00) as *mut u32;
    pub const RDRAM_DEVICE_ID_REG: *mut u32 = (RDRAM_BASE + 0x04) as *mut u32;
    pub const RDRAM_DELAY_REG: *mut u32 = (RDRAM_BASE + 0x08) as *mut u32;
    pub const RDRAM_MODE_REG: *mut u32 = (RDRAM_BASE + 0x0C) as *mut u32;
    pub const RDRAM_REF_INTERVAL_REG: *mut u32 = (RDRAM_BASE + 0x10) as *mut u32;
    pub const RDRAM_REF_ROW_REG: *mut u32 = (RDRAM_BASE + 0x14) as *mut u32;
    pub const RDRAM_RAS_INTERVAL_REG: *mut u32 = (RDRAM_BASE + 0x18) as *mut u32;
    pub const RDRAM_MIN_INTRVAL_REG: *mut u32 = (RDRAM_BASE + 0x1C) as *mut u32;
    pub const RDRAM_ADDR_SELECT_REG: *mut u32 = (RDRAM_BASE + 0x20) as *mut u32;
    pub const RDRAM_DEVICE_MANUF_REG: *mut u32 = (RDRAM_BASE + 0x24) as *mut u32;

    pub const SP_DMEM: usize = 0x0400_0000;
    pub const SP_IMEM: usize = 0x0400_1000;

    const SP_BASE_0: usize = 0x0404_0000;

    pub const SP_MEM_ADDR_REG: *mut u32 = (SP_BASE_0 + 0x00) as *mut u32;
    pub const SP_DRAM_ADDR_REG: *mut u32 = (SP_BASE_0 + 0x04) as *mut u32;
    pub const SP_RD_LEN_REG: *mut u32 = (SP_BASE_0 + 0x08) as *mut u32;
    pub const SP_WR_LEN_REG: *mut u32 = (SP_BASE_0 + 0x0C) as *mut u32;
    pub const SP_STATUS_REG: *mut u32 = (SP_BASE_0 + 0x10) as *mut u32;
    pub const SP_DMA_FULL_REG: *mut u32 = (SP_BASE_0 + 0x14) as *mut u32;
    pub const SP_DMA_BUSY_REG: *mut u32 = (SP_BASE_0 + 0x18) as *mut u32;
    pub const SP_SEMAPHORE_REG: *mut u32 = (SP_BASE_0 + 0x1C) as *mut u32;

    const SP_BASE_1: usize = 0x0408_0000;

    pub const SP_PC_REG: *mut u32 = (SP_BASE_1 + 0x00) as *mut u32;
    pub const SP_IBIST_REG: *mut u32 = (SP_BASE_1 + 0x04) as *mut u32;

    const DPC_BASE: usize = 0x0410_0000;

    pub const DPC_START_REG: *mut u32 = (DPC_BASE + 0x00) as *mut u32;
    pub const DPC_END_REG: *mut u32 = (DPC_BASE + 0x04) as *mut u32;
    pub const DPC_CURRENT_REG: *mut u32 = (DPC_BASE + 0x08) as *mut u32;
    pub const DPC_STATUS_REG: *mut u32 = (DPC_BASE + 0x0C) as *mut u32;
    pub const DPC_CLOCK_REG: *mut u32 = (DPC_BASE + 0x10) as *mut u32;
    pub const DPC_BUFBUSY_REG: *mut u32 = (DPC_BASE + 0x14) as *mut u32;
    pub const DPC_PIPEBUSY_REG: *mut u32 = (DPC_BASE + 0x18) as *mut u32;
    pub const DPC_TMEM_REG: *mut u32 = (DPC_BASE + 0x1C) as *mut u32;

    const DPS_BASE: usize = 0x0420_0000;

    pub const DPS_TBIST_REG: *mut u32 = (DPS_BASE + 0x00) as *mut u32;
    pub const DPS_TEST_MODE_REG: *mut u32 = (DPS_BASE + 0x04) as *mut u32;
    pub const DPS_BUFTEST_ADDR_REG: *mut u32 = (DPS_BASE + 0x08) as *mut u32;
    pub const DPS_BUFTEST_DATA_REG: *mut u32 = (DPS_BASE + 0x0C) as *mut u32;

    const MI_BASE: usize = 0x0430_0000;

    pub const MI_INIT_TEST_REG: *mut u32 = (MI_BASE + 0x00) as *mut u32;
    pub const MI_VERSION_REG: *mut u32 = (MI_BASE + 0x04) as *mut u32;
    pub const MI_INTR_REG: *mut u32 = (MI_BASE + 0x08) as *mut u32;
    pub const MI_INTER_MASK_REG: *mut u32 = (MI_BASE + 0x0C) as *mut u32;

    const AI_BASE: usize = 0x0450_0000;

    pub const AI_DRAM_ADDR_REG: *mut u32 = (AI_BASE + 0x00) as *mut u32;
    pub const AI_LEN_REG: *mut u32 = (AI_BASE + 0x04) as *mut u32;
    pub const AI_CONTROL_REG: *mut u32 = (AI_BASE + 0x08) as *mut u32;
    pub const AI_STATUS_REG: *mut u32 = (AI_BASE + 0x0C) as *mut u32;
    pub const AI_DACRATE_REG: *mut u32 = (AI_BASE + 0x10) as *mut u32;
    pub const AI_BITRATE_REG: *mut u32 = (AI_BASE + 0x14) as *mut u32;

    const PI_BASE: usize = 0x0460_0000;

    pub const PI_DRAM_ADDR_REG: *mut u32 = (PI_BASE + 0x00) as *mut u32;
    pub const PI_CART_ADDR_REG: *mut u32 = (PI_BASE + 0x04) as *mut u32;
    pub const PI_RD_LEN_REG: *mut u32 = (PI_BASE + 0x08) as *mut u32;
    pub const PI_WR_LEN_REG: *mut u32 = (PI_BASE + 0x0C) as *mut u32;
    pub const PI_STATUS_REG: *mut u32 = (PI_BASE + 0x10) as *mut u32;
    pub const PI_BSD_DOM1_LAT_REG: *mut u32 = (PI_BASE + 0x14) as *mut u32;
    pub const PI_BSD_DOM1_PWD_REG: *mut u32 = (PI_BASE + 0x18) as *mut u32;
    pub const PI_BSD_DOM1_PGS_REG: *mut u32 = (PI_BASE + 0x1C) as *mut u32;
    pub const PI_BSD_DOM1_RLS_REG: *mut u32 = (PI_BASE + 0x20) as *mut u32;
    pub const PI_BSD_DOM2_LAT_REG: *mut u32 = (PI_BASE + 0x24) as *mut u32;
    pub const PI_BSD_DOM2_PWD_REG: *mut u32 = (PI_BASE + 0x28) as *mut u32;
    pub const PI_BSD_DOM2_PGS_REG: *mut u32 = (PI_BASE + 0x2C) as *mut u32;
    pub const PI_BSD_DOM2_RLS_REG: *mut u32 = (PI_BASE + 0x30) as *mut u32;

    const RI_BASE: usize = 0x0470_0000;

    pub const RI_MODE_REG: *mut u32 = (RI_BASE + 0x00) as *mut u32;
    pub const RI_CONFIG_REG: *mut u32 = (RI_BASE + 0x04) as *mut u32;
    pub const RI_CURRENT_LOAD_REG: *mut u32 = (RI_BASE + 0x08) as *mut u32;
    pub const RI_SELECT_REG: *mut u32 = (RI_BASE + 0x0C) as *mut u32;
    pub const RI_REFRESH_REG: *mut u32 = (RI_BASE + 0x10) as *mut u32;
    pub const RI_LATENCY_REG: *mut u32 = (RI_BASE + 0x14) as *mut u32;
    pub const RI_RERROR_REG: *mut u32 = (RI_BASE + 0x18) as *mut u32;
    pub const RI_WERROR_REG: *mut u32 = (RI_BASE + 0x1C) as *mut u32;

    const SI_BASE: usize = 0x0480_0000;

    pub const SI_DRAM_ADDR_REG: *mut u32 = (SI_BASE + 0x00) as *mut u32;
    pub const SI_PIF_ADDR_RD64B_REG: *mut u32 = (SI_BASE + 0x04) as *mut u32;
    pub const SI_PIF_ADDR_WR64B_REG: *mut u32 = (SI_BASE + 0x10) as *mut u32;
    pub const SI_STATUS_REG: *mut u32 = (SI_BASE + 0x18) as *mut u32;
}
