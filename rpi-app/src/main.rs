use crate::sensors::si7021::SI7021;
use crate::sensors::{HumiditySensor, TemperatureSensor};

mod sensors;

fn main() {
    println!("Hello, world!");
    let mut dev = SI7021::new_from_descriptor("/dev/i2c-1", 0x40).unwrap();
    println!("Result: {}C, {}%", dev.read_temperature().unwrap(), dev.read_humidity().unwrap());
}
