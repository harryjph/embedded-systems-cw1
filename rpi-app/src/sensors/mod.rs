use std::error::Error;
use std::result;

pub mod si7021;

pub type Result<T> = result::Result<T, Box<dyn Error>>;

pub trait TemperatureSensor {
    fn read_temperature(&mut self) -> Result<f32>;
}

pub trait HumiditySensor {
    fn read_humidity(&mut self) -> Result<f32>;
}
