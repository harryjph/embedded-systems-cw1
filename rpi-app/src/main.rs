use std::error::Error;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;
use tokio::time::sleep;
use crate::nodeapi::Client;
use crate::nodeapi::grpc_generated::EnvironmentData;
use crate::sensors::si7021::SI7021;
use crate::sensors::{HumiditySensor, ProximitySensor, TemperatureSensor};
use crate::sensors::vl53l0x::VL53L0X;

mod nodeapi;
mod sensors;
mod util;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut sensor = SI7021::new_from_descriptor("/dev/i2c-1", 0x40)?;

    let mut prox_sensor = VL53L0X::new_from_descriptor("/dev/i2c-1", 0x29)?;
    loop {
        println!("Distance: {}mm", prox_sensor.read_proximity().await?);
        sleep(Duration::from_millis(100)).await;
    }

    let mut client = Client::new("http://localhost:81").await?;

    let (client_readings_in, client_readings_out) = mpsc::channel(1);
    tokio::spawn(async move {
        client.report_environment(client_readings_out).await.unwrap();
    });

    let mut interval_timer = time::interval(Duration::from_secs(1));
    loop {
        let reading = EnvironmentData { temperature: sensor.read_temperature().await?, relative_humidity: sensor.read_humidity().await? };
        println!("Reading: {reading:?}");
        client_readings_in.send(reading).await?;
        interval_timer.tick().await;
    }
}
