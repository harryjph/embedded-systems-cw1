use std::result;
use anyhow::Error;
use async_trait::async_trait;

pub mod si7021;
pub mod vl53l0x;

pub type Result<T> = result::Result<T, Error>;

#[async_trait]
pub trait TemperatureSensor {
    /// Reads the temperature in degrees celsius
    async fn read_temperature(&mut self) -> Result<f32>;
}

#[async_trait]
pub trait HumiditySensor {
    /// Reads the relative humidity as a percentage
    async fn read_humidity(&mut self) -> Result<f32>;
}

#[async_trait]
pub trait ProximitySensor {
    /// Reads the proximity in meters
    async fn read_proximity(&mut self) -> Result<f32>;
}
