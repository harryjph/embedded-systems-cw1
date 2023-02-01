use byteorder::{BigEndian, ByteOrder};
use i2cdev::core::I2CDevice;
use super::*;

impl<D: I2CDevice> VL53L0X<D> {
    pub fn read_register<R: Into<u8>>(&mut self, reg: R) -> Result<u8> {
        let mut data = [0];
        self.device.write(&[reg.into()]).map_err(stringify_error)?;
        self.device.read(&mut data).map_err(stringify_error)?;
        Ok(data[0])
    }

    pub fn read_register_burst<R: Into<u8>>(&mut self, reg: R, buffer: &mut [u8]) -> Result<()> {
        self.device.write(&[(reg.into())]).map_err(stringify_error)?;
        self.device.read(buffer).map_err(stringify_error)
    }

    pub fn read_register_u16<R: Into<u8>>(&mut self, reg: R) -> Result<u16> {
        let mut buffer: [u8; 2] = [0, 0];
        self.read_register_burst(reg, &mut buffer)?;
        Ok(BigEndian::read_u16(&buffer))
    }

    pub fn write_register<R: Into<u8>>(&mut self, reg: R, byte: u8) -> Result<()> {
        self.device.write(&[reg.into(), byte]).map_err(stringify_error)
    }

    pub fn write_register_burst<R: Into<u8>>(&mut self, reg: R, bytes: &[u8]) -> Result<()> {
        self.device.write(&[&[reg.into()], bytes].concat()).map_err(stringify_error)
    }

    pub fn write_register_u16<R: Into<u8>>(&mut self, reg: R, data: u16) -> Result<()> {
        let mut buffer = [0u8; 2];
        BigEndian::write_u16(&mut buffer, data);
        self.write_register_burst(reg, &buffer)
    }
}