use std::thread::sleep;
use std::time::Duration;
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;
use byteorder::{BigEndian, ByteOrder};

fn main() {
    println!("Hello, world!");

    let mut dev = LinuxI2CDevice::new("/dev/i2c-1", 0x40).unwrap();
    dev.write(&[0xF3]).unwrap();
    sleep(Duration::from_secs(1));
    let mut data = [0u8; 2];
    dev.read(&mut data).unwrap();
    let data_parsed = BigEndian::read_u16(&data);
    println!("Result: {data_parsed}");
}
