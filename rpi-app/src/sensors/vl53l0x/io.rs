use i2cdev::core::I2CDevice;
use super::*;

impl<D: I2CDevice> VL53L0X<D> {
    pub fn read_byte(&mut self, reg: u8) -> Result<u8> {
        let mut data = [0];
        self.device.write(&[reg]).map_err(stringify_error)?;
        self.device.read(&mut data).map_err(stringify_error)?;
        Ok(data[0])
    }

    pub fn read_register(&mut self, reg: Register) -> Result<u8> {
        self.read_byte(reg as u8)
    }

    pub fn read_registers(&mut self, reg: Register, buffer: &mut [u8]) -> Result<()> {
        self.device.write(&[(reg as u8)]).map_err(stringify_error)?;
        self.device.read(buffer).map_err(stringify_error)
    }

    pub fn read_u16(&mut self, reg: Register) -> Result<u16> {
        let mut buffer: [u8; 2] = [0, 0];
        self.read_registers(reg, &mut buffer)?;
        Ok(((buffer[0] as u16) << 8) + buffer[1] as u16) // TODO byteorder
    }

    pub fn write_byte(&mut self, reg: u8, byte: u8) -> Result<()> {
        self.device.write(&[reg, byte]).map_err(stringify_error)
    }

    pub fn write_register(&mut self, reg: Register, byte: u8) -> Result<()> {
        self.write_byte(reg as u8, byte)
    }

    pub fn write_register_burst(&mut self, reg: Register, bytes: &[u8]) -> Result<()> {
        self.device.write(&[&[reg as u8], bytes].concat()).map_err(stringify_error)
    }

    pub fn write_u16(&mut self, reg: Register, word: u16) -> Result<()> {
        let msb = (word >> 8) as u8; // TODO byteorder
        let lsb = (word & 0xFF) as u8;
        self.device.write(&[reg as u8, msb, lsb]).map_err(stringify_error)
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