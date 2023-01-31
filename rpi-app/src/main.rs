use std::error::Error;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;
use crate::nodeapi::Client;
use crate::nodeapi::grpc_generated::EnvironmentData;
use crate::sensors::si7021::SI7021;
use crate::sensors::{HumiditySensor, TemperatureSensor};

mod nodeapi;
mod sensors;
mod util;
mod config;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut sensor = SI7021::new_from_descriptor("/dev/i2c-1", 0x40)?;
    let mut client = Client::new("http://192.168.1.48:1051").await?;

    
    let (tx, mut rx) = mpsc::channel(1);
    let (client_readings_in, client_readings_out) = mpsc::channel(1);
    tokio::spawn(async move {
        client.assign_id(tx).await.unwrap();
        client.report_environment(client_readings_out).await.unwrap();
    });

    println!("HELLOOO!!! {:?}", rx.recv().await);
    let mut interval_timer = time::interval(Duration::from_secs(1));
    loop {
        
        let reading = EnvironmentData { temperature: sensor.read_temperature().await?, relative_humidity: sensor.read_humidity().await? };
        println!("Reading: {reading:?}");
        client_readings_in.send(reading).await?;
        interval_timer.tick().await;
    }
}
