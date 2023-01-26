use crate::sensors::si7021::SI7021;
use crate::sensors::{HumiditySensor, TemperatureSensor};

mod nodeapi;
mod sensors;
mod util;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut dev = SI7021::new_from_descriptor("/dev/i2c-1", 0x40).unwrap();
    println!("Result: {}C, {}%", dev.read_temperature().await.unwrap(), dev.read_humidity().await.unwrap());
}
