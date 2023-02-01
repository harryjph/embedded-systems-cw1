use super::super::Result;
use super::util::*;
use crate::sensors::vl53l0x::VL53L0X;
use i2cdev::core::I2CDevice;

impl<D: I2CDevice> VL53L0X<D> {
    fn set_signal_rate_limit(&mut self, limit: f32) -> Result<bool> {
        if limit < 0.0 || limit > 511.99 {
            Ok(false)
        } else {
            // Q9.7 fixed point format (9 integer bits, 7 fractional bits)
            self.write_register_u16(
                Register::FINAL_RANGE_CONFIG_MIN_COUNT_RATE_RTN_LIMIT,
                (limit * ((1 << 7) as f32)) as u16,
            )?;
            Ok(true)
        }
    }

    fn get_spad_info(&mut self) -> Result<(u8, u8)> {
        self.write_register(0x80, 0x01)?;
        self.write_register(0xFF, 0x01)?;
        self.write_register(0x00, 0x00)?;

        self.write_register(0xFF, 0x06)?;
        let tmp = self.read_register(0x83)?;
        self.write_register(0x83, tmp | 0x04)?;
        self.write_register(0xFF, 0x07)?;
        self.write_register(0x81, 0x01)?;

        self.write_register(0x80, 0x01)?;

        self.write_register(0x94, 0x6b)?;
        self.write_register(0x83, 0x00)?;

        self.wait_for(|s| Ok(s.read_register(0x83)? != 0))?;

        self.write_register(0x83, 0x01)?;
        let tmp = self.read_register(0x92)?;

        let count: u8 = tmp & 0x7f;
        let type_is_aperture: u8 = (tmp >> 7) & 0x01;

        self.write_register(0x81, 0x00)?;
        self.write_register(0xFF, 0x06)?;
        let tmp = self.read_register(0x83)?;
        self.write_register(0x83, tmp & !0x04)?;
        self.write_register(0xFF, 0x01)?;
        self.write_register(0x00, 0x01)?;

        self.write_register(0xFF, 0x00)?;
        self.write_register(0x80, 0x00)?;

        Ok((count, type_is_aperture))
    }

    fn perform_single_ref_calibration(&mut self, vhv_init_byte: u8) -> Result<()> {
        self.write_register(Register::SYSRANGE_START, 0x01 | vhv_init_byte)?;
        self.wait_for(|s| Ok((s.read_register(Register::RESULT_INTERRUPT_STATUS)? & 0x07) != 0))?;
        self.write_register(Register::SYSTEM_INTERRUPT_CLEAR, 0x01)?;
        self.write_register(Register::SYSRANGE_START, 0x00)?;
        Ok(())
    }

    pub fn init_hardware(&mut self) -> Result<()> {
        // Sensor uses 1V8 mode for I/O by default; switch to 2V8 mode if necessary
        if self.io_mode2v8 {
            // set bit 0
            let ext_sup_hv = self.read_register(Register::VHV_CONFIG_PAD_SCL_SDA__EXTSUP_HV)?;
            self.write_register(
                Register::VHV_CONFIG_PAD_SCL_SDA__EXTSUP_HV,
                ext_sup_hv | 0x01,
            )?;
        }

        // "Set I2C standard mode"
        self.write_register(0x88, 0x00)?;
        self.write_register(0x80, 0x01)?;
        self.write_register(0xFF, 0x01)?;
        self.write_register(0x00, 0x00)?;
        self.stop_variable = self.read_register(0x91)?;
        self.write_register(0x00, 0x01)?;
        self.write_register(0xFF, 0x00)?;
        self.write_register(0x80, 0x00)?;

        // disable SIGNAL_RATE_MSRC (bit 1) and SIGNAL_RATE_PRE_RANGE (bit 4) limit checks
        let config = self.read_register(Register::MSRC_CONFIG_CONTROL)?;
        self.write_register(Register::MSRC_CONFIG_CONTROL, config | 0x12)?;

        // set final range signal rate limit to 0.25 MCPS (million counts per second)
        self.set_signal_rate_limit(0.25)?;

        self.write_register(Register::SYSTEM_SEQUENCE_CONFIG, 0xFF)?;

        let (spad_count, spad_type_is_aperture) = self.get_spad_info()?;

        // The SPAD map (RefGoodSpadMap) is read by VL53L0X_get_info_from_device() in the API,
        // but the same data seems to be more easily readable from GLOBAL_CONFIG_SPAD_ENABLES_REF_0 through _6, so read it from there
        let mut ref_spad_map = [0u8; 6];
        self.read_register_burst(
            Register::GLOBAL_CONFIG_SPAD_ENABLES_REF_0,
            &mut ref_spad_map,
        )?;

        self.write_register(0xFF, 0x01)?;
        self.write_register(Register::DYNAMIC_SPAD_REF_EN_START_OFFSET, 0x00)?;
        self.write_register(Register::DYNAMIC_SPAD_NUM_REQUESTED_REF_SPAD, 0x2C)?;
        self.write_register(0xFF, 0x00)?;
        self.write_register(Register::GLOBAL_CONFIG_REF_EN_START_SELECT, 0xB4)?;

        // 12 is the first aperture spad
        let first_spad_to_enable = if spad_type_is_aperture != 0 { 12 } else { 0 };
        let mut spads_enabled: u8 = 0;

        for i in 0..48 {
            if i < first_spad_to_enable || spads_enabled == spad_count {
                // This bit is lower than the first one that should be enabled, or (reference_spad_count) bits have already been enabled, so zero this bit
                ref_spad_map[i / 8] &= !(1 << (i % 8));
            } else if (ref_spad_map[i / 8] >> (i % 8)) & 0x1 > 0 {
                spads_enabled = spads_enabled + 1;
            }
        }

        self.write_register_burst(Register::GLOBAL_CONFIG_SPAD_ENABLES_REF_0, &ref_spad_map)?;

        // DefaultTuningSettings from vl53l0x_tuning.h

        self.write_register(0xFF, 0x01)?;
        self.write_register(0x00, 0x00)?;

        self.write_register(0xFF, 0x00)?;
        self.write_register(0x09, 0x00)?;
        self.write_register(0x10, 0x00)?;
        self.write_register(0x11, 0x00)?;

        self.write_register(0x24, 0x01)?;
        self.write_register(0x25, 0xFF)?;
        self.write_register(0x75, 0x00)?;

        self.write_register(0xFF, 0x01)?;
        self.write_register(0x4E, 0x2C)?;
        self.write_register(0x48, 0x00)?;
        self.write_register(0x30, 0x20)?;

        self.write_register(0xFF, 0x00)?;
        self.write_register(0x30, 0x09)?;
        self.write_register(0x54, 0x00)?;
        self.write_register(0x31, 0x04)?;
        self.write_register(0x32, 0x03)?;
        self.write_register(0x40, 0x83)?;
        self.write_register(0x46, 0x25)?;
        self.write_register(0x60, 0x00)?;
        self.write_register(0x27, 0x00)?;
        self.write_register(0x50, 0x06)?;
        self.write_register(0x51, 0x00)?;
        self.write_register(0x52, 0x96)?;
        self.write_register(0x56, 0x08)?;
        self.write_register(0x57, 0x30)?;
        self.write_register(0x61, 0x00)?;
        self.write_register(0x62, 0x00)?;
        self.write_register(0x64, 0x00)?;
        self.write_register(0x65, 0x00)?;
        self.write_register(0x66, 0xA0)?;

        self.write_register(0xFF, 0x01)?;
        self.write_register(0x22, 0x32)?;
        self.write_register(0x47, 0x14)?;
        self.write_register(0x49, 0xFF)?;
        self.write_register(0x4A, 0x00)?;

        self.write_register(0xFF, 0x00)?;
        self.write_register(0x7A, 0x0A)?;
        self.write_register(0x7B, 0x00)?;
        self.write_register(0x78, 0x21)?;

        self.write_register(0xFF, 0x01)?;
        self.write_register(0x23, 0x34)?;
        self.write_register(0x42, 0x00)?;
        self.write_register(0x44, 0xFF)?;
        self.write_register(0x45, 0x26)?;
        self.write_register(0x46, 0x05)?;
        self.write_register(0x40, 0x40)?;
        self.write_register(0x0E, 0x06)?;
        self.write_register(0x20, 0x1A)?;
        self.write_register(0x43, 0x40)?;

        self.write_register(0xFF, 0x00)?;
        self.write_register(0x34, 0x03)?;
        self.write_register(0x35, 0x44)?;

        self.write_register(0xFF, 0x01)?;
        self.write_register(0x31, 0x04)?;
        self.write_register(0x4B, 0x09)?;
        self.write_register(0x4C, 0x05)?;
        self.write_register(0x4D, 0x04)?;

        self.write_register(0xFF, 0x00)?;
        self.write_register(0x44, 0x00)?;
        self.write_register(0x45, 0x20)?;
        self.write_register(0x47, 0x08)?;
        self.write_register(0x48, 0x28)?;
        self.write_register(0x67, 0x00)?;
        self.write_register(0x70, 0x04)?;
        self.write_register(0x71, 0x01)?;
        self.write_register(0x72, 0xFE)?;
        self.write_register(0x76, 0x00)?;
        self.write_register(0x77, 0x00)?;

        self.write_register(0xFF, 0x01)?;
        self.write_register(0x0D, 0x01)?;

        self.write_register(0xFF, 0x00)?;
        self.write_register(0x80, 0x01)?;
        self.write_register(0x01, 0xF8)?;

        self.write_register(0xFF, 0x01)?;
        self.write_register(0x8E, 0x01)?;
        self.write_register(0x00, 0x01)?;
        self.write_register(0xFF, 0x00)?;
        self.write_register(0x80, 0x00)?;

        self.write_register(Register::SYSTEM_INTERRUPT_CONFIG_GPIO, 0x04)?;
        let high = self.read_register(Register::GPIO_HV_MUX_ACTIVE_HIGH)?;
        self.write_register(Register::GPIO_HV_MUX_ACTIVE_HIGH, high & !0x10)?;
        self.write_register(Register::SYSTEM_INTERRUPT_CLEAR, 0x01)?;

        self.measurement_timing_budget_microseconds = self.get_measurement_timing_budget()?;
        self.write_register(Register::SYSTEM_SEQUENCE_CONFIG, 0xE8)?;
        let mtbm = self.measurement_timing_budget_microseconds;
        self.set_measurement_timing_budget(mtbm)?;

        self.write_register(Register::SYSTEM_SEQUENCE_CONFIG, 0x01)?;
        self.perform_single_ref_calibration(0x40)?;

        self.write_register(Register::SYSTEM_SEQUENCE_CONFIG, 0x02)?;
        self.perform_single_ref_calibration(0x00)?;

        self.write_register(Register::SYSTEM_SEQUENCE_CONFIG, 0xE8)?;
        Ok(())
    }

    fn get_vcsel_pulse_period(&mut self, ty: VcselPeriodType) -> Result<u8> {
        match ty {
            VcselPeriodType::VcselPeriodPreRange => Ok(decode_vcsel_period(
                self.read_register(Register::PRE_RANGE_CONFIG_VCSEL_PERIOD)?,
            )),
            VcselPeriodType::VcselPeriodFinalRange => Ok(decode_vcsel_period(
                self.read_register(Register::FINAL_RANGE_CONFIG_VCSEL_PERIOD)?,
            )),
        }
    }

    fn get_sequence_step_enables(&mut self) -> Result<SeqStepEnables> {
        let sequence_config = self.read_register(Register::SYSTEM_SEQUENCE_CONFIG)?;
        Ok(SeqStepEnables {
            tcc: ((sequence_config >> 4) & 0x1) == 1,
            dss: ((sequence_config >> 3) & 0x1) == 1,
            msrc: ((sequence_config >> 2) & 0x1) == 1,
            pre_range: ((sequence_config >> 6) & 0x1) == 1,
            final_range: ((sequence_config >> 7) & 0x1) == 1,
        })
    }

    fn get_sequence_step_timeouts(&mut self, enables: &SeqStepEnables) -> Result<SeqStepTimeouts> {
        let pre_range_mclks =
            decode_timeout(self.read_register_u16(Register::PRE_RANGE_CONFIG_TIMEOUT_MACROP_HI)?);
        let mut final_range_mclks =
            decode_timeout(self.read_register_u16(Register::FINAL_RANGE_CONFIG_TIMEOUT_MACROP_HI)?);
        if enables.pre_range {
            final_range_mclks -= pre_range_mclks;
        };
        let pre_range_vcselperiod_pclks =
            self.get_vcsel_pulse_period(VcselPeriodType::VcselPeriodPreRange)?;
        let msrc_dss_tcc_mclks = self.read_register(Register::MSRC_CONFIG_TIMEOUT_MACROP)? + 1;
        let final_range_vcsel_period_pclks =
            self.get_vcsel_pulse_period(VcselPeriodType::VcselPeriodFinalRange)?;
        Ok(SeqStepTimeouts {
            msrc_dss_tcc_microseconds: timeout_mclks_to_microseconds(
                msrc_dss_tcc_mclks as u16,
                pre_range_vcselperiod_pclks,
            ),
            pre_range_mclks: pre_range_mclks,
            pre_range_microseconds: timeout_mclks_to_microseconds(
                pre_range_mclks,
                pre_range_vcselperiod_pclks,
            ),
            final_range_vcsel_period_pclks,
            final_range_microseconds: timeout_mclks_to_microseconds(
                final_range_mclks,
                final_range_vcsel_period_pclks,
            ),
        })
    }

    fn get_measurement_timing_budget(&mut self) -> Result<u32> {
        let start_overhead: u32 = 1910;
        let end_overhead: u32 = 960;
        let msrc_overhead: u32 = 660;
        let tcc_overhead: u32 = 590;
        let dss_overhead: u32 = 690;
        let pre_range_overhead: u32 = 660;
        let final_range_overhead: u32 = 550;

        let enables = self.get_sequence_step_enables()?;
        let timeouts = self.get_sequence_step_timeouts(&enables)?;

        // "Start and end overhead times always present"
        let mut budget_microseconds = start_overhead + end_overhead;
        if enables.tcc {
            budget_microseconds += timeouts.msrc_dss_tcc_microseconds + tcc_overhead;
        }
        if enables.dss {
            budget_microseconds += 2 * (timeouts.msrc_dss_tcc_microseconds + dss_overhead);
        } else if enables.msrc {
            budget_microseconds += timeouts.msrc_dss_tcc_microseconds + msrc_overhead;
        }
        if enables.pre_range {
            budget_microseconds += timeouts.pre_range_microseconds + pre_range_overhead;
        }
        if enables.final_range {
            budget_microseconds += timeouts.final_range_microseconds + final_range_overhead;
        }

        Ok(budget_microseconds)
    }

    fn set_measurement_timing_budget(&mut self, budget_microseconds: u32) -> Result<bool> {
        // note that these are different than values in get_
        let start_overhead: u32 = 1320;
        let end_overhead: u32 = 960;
        let msrc_overhead: u32 = 660;
        let tcc_overhead: u32 = 590;
        let dss_overhead: u32 = 690;
        let pre_range_overhead: u32 = 660;
        let final_range_overhead: u32 = 550;
        let min_timing_budget: u32 = 20000;

        if budget_microseconds < min_timing_budget {
            return Ok(false);
        }

        let enables = self.get_sequence_step_enables()?;
        let timeouts = self.get_sequence_step_timeouts(&enables)?;

        let mut use_budget_microseconds: u32 = (start_overhead + end_overhead) as u32;
        if enables.tcc {
            use_budget_microseconds += timeouts.msrc_dss_tcc_microseconds + tcc_overhead;
        }
        if enables.dss {
            use_budget_microseconds += 2 * timeouts.msrc_dss_tcc_microseconds + dss_overhead;
        } else if enables.msrc {
            use_budget_microseconds += timeouts.msrc_dss_tcc_microseconds + msrc_overhead;
        }
        if enables.pre_range {
            use_budget_microseconds += timeouts.pre_range_microseconds + pre_range_overhead;
        }
        if enables.final_range {
            use_budget_microseconds += final_range_overhead;
        }

        if use_budget_microseconds > budget_microseconds {
            // Requested timeout too small
            return Ok(false);
        }

        let final_range_timeout_microseconds: u32 = budget_microseconds - use_budget_microseconds;

        let mut final_range_timeout_mclks: u16 = timeout_microseconds_to_mclks(
            final_range_timeout_microseconds,
            timeouts.final_range_vcsel_period_pclks,
        ) as u16;

        if enables.pre_range {
            final_range_timeout_mclks += timeouts.pre_range_mclks;
        }

        self.write_register_u16(
            Register::FINAL_RANGE_CONFIG_TIMEOUT_MACROP_HI,
            encode_timeout(final_range_timeout_mclks),
        )?;

        self.measurement_timing_budget_microseconds = budget_microseconds;
        Ok(true)
    }
}

struct SeqStepEnables {
    pub tcc: bool,
    pub dss: bool,
    pub msrc: bool,
    pub pre_range: bool,
    pub final_range: bool,
}

struct SeqStepTimeouts {
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

impl From<Register> for u8 {
    fn from(value: Register) -> Self {
        value as u8
    }
}

#[derive(Debug, Copy, Clone)]
enum VcselPeriodType {
    VcselPeriodPreRange = 0,
    VcselPeriodFinalRange = 1,
}
