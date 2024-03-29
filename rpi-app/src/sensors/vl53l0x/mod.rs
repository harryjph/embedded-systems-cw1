//! A VL53L0X driver based on https://github.com/copterust/vl53l0x

use super::{ProximitySensor, Result};
use crate::sensors::vl53l0x::device_setup::Register;
use crate::util::stringify_error;
use anyhow::Error;
use async_trait::async_trait;
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;
use std::path::Path;

mod device_setup;
mod io;
mod util;

pub struct VL53L0X<D> {
    device: D,
    pub revision_id: u8,
    io_mode2v8: bool,
    stop_variable: u8,
    measurement_timing_budget_microseconds: u32,
}

impl VL53L0X<LinuxI2CDevice> {
    pub fn new_from_descriptor<P: AsRef<Path>>(path: P, slave_address: u16) -> Result<Self> {
        Ok(VL53L0X::new(LinuxI2CDevice::new(path, slave_address)?)?)
    }
}

impl<D: I2CDevice> VL53L0X<D> {
    pub fn new(device: D) -> Result<Self> {
        let mut driver = VL53L0X {
            device,
            revision_id: 0x00,
            io_mode2v8: true,
            stop_variable: 0,
            measurement_timing_budget_microseconds: 0,
        };

        let who_am_i = driver.read_register(Register::WHO_AM_I)?;
        if who_am_i == 0xEE {
            driver.init_hardware()?;
            Ok(driver)
        } else {
            Err(Error::msg(format!("Invalid device: {who_am_i}")))
        }
    }

    fn take_reading(&mut self) -> Result<f32> {
        // Send a measure command
        self.write_register(0x80, 0x01)?;
        self.write_register(0xFF, 0x01)?;
        self.write_register(0x00, 0x00)?;
        let sv = self.stop_variable;
        self.write_register(0x91, sv)?;
        self.write_register(0x00, 0x01)?;
        self.write_register(0xFF, 0x00)?;
        self.write_register(0x80, 0x00)?;

        self.write_register(Register::SYSRANGE_START, 0x01)?;
        self.wait_for(|s| Ok((s.read_register(Register::SYSRANGE_START)? & 0x01) == 0))?;

        // Read the result
        self.wait_for(|s| Ok((s.read_register(Register::RESULT_INTERRUPT_STATUS)? & 0x07) != 0))?;
        let range_err = self.read_register_u16(Register::RESULT_RANGE_STATUS_plus_10);
        // Clear this before checking error
        self.write_register(Register::SYSTEM_INTERRUPT_CLEAR, 0x01)?;
        Ok(range_err? as f32 / 1000.0)
    }
}

#[async_trait]
impl<D: I2CDevice + Send> ProximitySensor for VL53L0X<D> {
    async fn read_proximity(&mut self) -> Result<f32> {
        let reading = self.take_reading()?;
        if reading > 0.0 && reading <= 1.0 {
            Ok(reading)
        } else {
            Ok(f32::NAN)
        }
    }
}
