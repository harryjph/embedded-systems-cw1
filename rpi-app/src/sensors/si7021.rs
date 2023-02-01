use std::path::Path;
use std::result;
use std::time::Duration;
use async_trait::async_trait;
use byteorder::{BigEndian, ByteOrder};
use i2cdev::core::I2CDevice;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use tokio::time::sleep;
use crate::util::stringify_error;
use super::{Result, HumiditySensor, TemperatureSensor};

const MEASURE_TEMPERATURE_NO_HOLD: u8 = 0xF3;
const MEASURE_HUMIDITY_NO_HOLD: u8 = 0xF5;
const READ_DELAY_MS: u64 = 100;

pub struct SI7021<D> {
    device: D,
}

impl SI7021<LinuxI2CDevice> {
    pub fn new_from_descriptor<P: AsRef<Path>>(path: P, slave_address: u16) -> result::Result<Self, LinuxI2CError> {
        Ok(SI7021::new(LinuxI2CDevice::new(path, slave_address)?))
    }
}

impl <D: I2CDevice> SI7021<D> {
    pub fn new(device: D) -> Self {
        SI7021 { device }
    }

    /// Reads a register but waits for READ_DELAY_MS between requesting the read and actually reading the data.
    async fn read_register_delayed(&mut self, command: u8) -> Result<u16> {
        self.device.write(&[command]).map_err(stringify_error)?;
        sleep(Duration::from_millis(READ_DELAY_MS)).await;
        let mut data = [0u8; 2];
        self.device.read(&mut data).map_err(stringify_error)?;
        Ok(BigEndian::read_u16(&data))
    }
}

#[async_trait]
impl <D: I2CDevice + Send> TemperatureSensor for SI7021<D> {
    async fn read_temperature(&mut self) -> Result<f32> {
        Ok(175.72 * self.read_register_delayed(MEASURE_TEMPERATURE_NO_HOLD).await? as f32 / 65536.0 - 46.85)
    }
}

#[async_trait]
impl <D: I2CDevice + Send> HumiditySensor for SI7021<D> {
    async fn read_humidity(&mut self) -> Result<f32> {
        Ok(125.0 * self.read_register_delayed(MEASURE_HUMIDITY_NO_HOLD).await? as f32 / 65536.0 - 6.0)
    }
}

#[cfg(test)]
mod tests {
    use i2cdev::mock::MockI2CDevice;
    use super::{MEASURE_HUMIDITY_NO_HOLD, MEASURE_TEMPERATURE_NO_HOLD, SI7021};
    use super::super::{HumiditySensor, TemperatureSensor};

    #[tokio::test]
    async fn test_sensor_driver() {
        let mut device = MockI2CDevice::new();
        device.regmap.write_regs(MEASURE_TEMPERATURE_NO_HOLD as usize, &[0x68, 0xAD]);
        device.regmap.write_regs(MEASURE_HUMIDITY_NO_HOLD as usize, &[0x68, 0x73]);
        let mut driver = SI7021::new(device);
        assert_eq!(25.000114, driver.read_temperature().await.unwrap());
        assert_eq!(45.000595, driver.read_humidity().await.unwrap());
    }
}
