use anyhow::Error;
use async_trait::async_trait;
use std::result;

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
    /// Returns `NaN` if the reading was invalid
    async fn read_proximity(&mut self) -> Result<f32>;

    /// Reads the proximity in meters, taking an average of several readings
    /// Returns `NaN` if any reading was invalid
    async fn average_proximity(&mut self, num_readings: usize) -> Result<f32> {
        let mut sum = 0.0f32;
        for _ in 0..num_readings {
            let reading = self.read_proximity().await?;
            if !reading.is_nan() {
                sum += reading;
            } else {
                return Ok(f32::NAN);
            }
        }
        Ok(sum / num_readings as f32)
    }
}
