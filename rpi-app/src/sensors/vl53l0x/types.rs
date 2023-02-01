pub struct SeqStepEnables {
    pub tcc: bool,
    pub dss: bool,
    pub msrc: bool,
    pub pre_range: bool,
    pub final_range: bool,
}

pub struct SeqStepTimeouts {
    pub final_range_vcsel_period_pclks: u8,
    pub pre_range_mclks: u16,
    pub msrc_dss_tcc_microseconds: u32,
    pub pre_range_microseconds: u32,
    pub final_range_microseconds: u32,
}

#[allow(non_camel_case_types)]
pub enum Register {
    SYSRANGE_START = 0x00,
    WHO_AM_I = 0xC0,
    VHV_CONFIG_PAD_SCL_SDA__EXTSUP_HV = 0x89,
    MSRC_CONFIG_CONTROL = 0x60,
    SYSTEM_SEQUENCE_CONFIG = 0x01,
    FINAL_RANGE_CONFIG_MIN_COUNT_RATE_RTN_LIMIT = 0x44,
    GLOBAL_CONFIG_SPAD_ENABLES_REF_0 = 0xB0,
    DYNAMIC_SPAD_REF_EN_START_OFFSET = 0x4F,
    DYNAMIC_SPAD_NUM_REQUESTED_REF_SPAD = 0x4E,
    GLOBAL_CONFIG_REF_EN_START_SELECT = 0xB6,
    SYSTEM_INTERRUPT_CONFIG_GPIO = 0x0A,
    GPIO_HV_MUX_ACTIVE_HIGH = 0x84,
    SYSTEM_INTERRUPT_CLEAR = 0x0B,
    RESULT_INTERRUPT_STATUS = 0x13,
    RESULT_RANGE_STATUS_plus_10 = 0x1e,
    FINAL_RANGE_CONFIG_VCSEL_PERIOD = 0x70,
    PRE_RANGE_CONFIG_VCSEL_PERIOD = 0x50,
    PRE_RANGE_CONFIG_TIMEOUT_MACROP_HI = 0x51,
    FINAL_RANGE_CONFIG_TIMEOUT_MACROP_HI = 0x71,
    MSRC_CONFIG_TIMEOUT_MACROP = 0x46,
}

#[derive(Debug, Copy, Clone)]
pub enum VcselPeriodType {
    VcselPeriodPreRange = 0,
    VcselPeriodFinalRange = 1,
}
