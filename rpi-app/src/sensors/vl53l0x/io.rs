use i2cdev::core::I2CDevice;
use super::*;

impl<D: I2CDevice> VL53L0X<D> {
    pub fn write_read(&mut self, bytes: &[u8], buffer: &mut [u8]) -> Result<()> {
        self.device.write(bytes).map_err(stringify_error)?;
        self.device.read(buffer).map_err(stringify_error)
    }

    pub fn read_byte(&mut self, reg: u8) -> Result<u8> {
        let mut data = [0];
        self.device.write(&[reg]).map_err(stringify_error)?;
        self.device.read(&mut data).map_err(stringify_error)?;
        Ok(data[0])
    }

    pub fn read_register(&mut self, reg: Register) -> Result<u8> {
        self.read_byte(reg as u8)
    }

    pub fn read_6bytes(&mut self, reg: Register) -> Result<[u8; 6]> {
        let mut ret: [u8; 6] = Default::default();
        self.read_registers(reg, &mut ret)?;

        Ok(ret)
    }

    pub fn read_registers(
        &mut self,
        reg: Register,
        buffer: &mut [u8],
    ) -> Result<()> {
        self.write_read(
            &[(reg as u8)],
            buffer,
        )?;

        Ok(())
    }

    pub fn read_16bit(&mut self, reg: Register) -> Result<u16> {
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

    pub fn write_6bytes(&mut self, reg: Register, bytes: [u8; 6]) -> Result<()> {
        self.device.write(&[reg as u8, bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]]).map_err(stringify_error)
    }

    pub fn write_16bit(&mut self, reg: Register, word: u16) -> Result<()> {
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