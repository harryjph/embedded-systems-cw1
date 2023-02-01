use i2cdev::core::I2CDevice;
use super::*;

impl<D: I2CDevice> VL53L0X<D> {
    pub fn read_register<R: Into<u8>>(&mut self, reg: R) -> Result<u8> {
        let mut data = [0];
        self.device.write(&[reg.into()]).map_err(stringify_error)?;
        self.device.read(&mut data).map_err(stringify_error)?;
        Ok(data[0])
    }

    pub fn read_registers<R: Into<u8>>(&mut self, reg: R, buffer: &mut [u8]) -> Result<()> {
        self.device.write(&[(reg.into())]).map_err(stringify_error)?;
        self.device.read(buffer).map_err(stringify_error)
    }

    pub fn read_register_u16<R: Into<u8>>(&mut self, reg: R) -> Result<u16> {
        let mut buffer: [u8; 2] = [0, 0];
        self.read_registers(reg, &mut buffer)?;
        Ok(((buffer[0] as u16) << 8) + buffer[1] as u16) // TODO byteorder
    }

    pub fn write_register<R: Into<u8>>(&mut self, reg: R, byte: u8) -> Result<()> {
        self.device.write(&[reg.into(), byte]).map_err(stringify_error)
    }

    pub fn write_register_burst<R: Into<u8>>(&mut self, reg: R, bytes: &[u8]) -> Result<()> {
        self.device.write(&[&[reg.into()], bytes].concat()).map_err(stringify_error)
    }

    pub fn write_register_u16<R: Into<u8>>(&mut self, reg: R, word: u16) -> Result<()> {
        let msb = (word >> 8) as u8; // TODO byteorder, use write_register_burst
        let lsb = (word & 0xFF) as u8;
        self.device.write(&[reg.into(), msb, lsb]).map_err(stringify_error)
    }

    /// Waits for a condition to be achieved, or times out if that takes too long.
    /// The condition is achieved when the predicate returns true
    pub fn wait_for<F: Fn(&mut Self) -> Result<bool>>(&mut self, predicate: F) -> Result<()> {
        for _ in 0..10000 {
            if predicate(self)? {
                return Ok(())
            }
        }
        Err("Timeout".into())
    }
}