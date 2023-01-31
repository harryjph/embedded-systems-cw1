use std::path::Path;
use async_trait::async_trait;
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;
use super::{Result, ProximitySensor};

pub struct VL53L0X<D> {
    device: D,
}

impl VL53L0X<LinuxI2CDevice> {
    pub fn new_from_descriptor<P: AsRef<Path>>(path: P, slave_address: u16) -> Result<Self> {
        Ok(VL53L0X::new(LinuxI2CDevice::new(path, slave_address)?)?)
    }
}

impl <D: I2CDevice> VL53L0X<D> {
    pub fn new(device: D) -> Result<Self> {
        todo!()
    }
}

#[async_trait]
impl <D: I2CDevice + Send> ProximitySensor for VL53L0X<D> {
    async fn read_proximity(&mut self) -> Result<f32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_sensor_driver() {
    }
}
