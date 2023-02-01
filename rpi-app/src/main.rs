use std::io;
use std::io::ErrorKind;
use std::process::exit;
use std::error::Error;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;
use crate::nodeapi::Client;
use crate::nodeapi::grpc_generated::EnvironmentData;
use crate::sensors::si7021::SI7021;
use crate::sensors::{HumiditySensor, TemperatureSensor};
use crate::sensors::vl53l0x::VL53L0X;
use crate::config::Config;

mod nodeapi;
mod sensors;
mod util;
mod config;


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut config = load_config();
    
    let mut environment_sensor = SI7021::new_from_descriptor("/dev/i2c-1", 0x40)?;
    let mut proximity_sensor = VL53L0X::new_from_descriptor("/dev/i2c-1", 0x29)?;
    
    let mut client = Client::new(config.url.clone()).await?;
    if let None = config.id {
        config.id = Some(client.assign_id().await?);
        config.write_default()?;
    }
    
    let (client_readings_in, client_readings_out) = mpsc::channel(1);
    tokio::spawn(async move {
        client.report_environment(client_readings_out).await.unwrap();
    });

    let mut interval_timer = time::interval(Duration::from_secs(1));
    loop {
        let reading = EnvironmentData {
            temperature: environment_sensor.read_temperature().await?,
            relative_humidity: environment_sensor.read_humidity().await?
        };
        println!("Reading: {reading:?}");
        client_readings_in.send(reading).await?;
        interval_timer.tick().await;
    }
}

fn load_config() -> Config {
    match Config::load_default() {
        Ok(config) => return config,
        Err(e) => {
            match e.downcast::<io::Error>() {
                Ok(e) => {
                    if e.kind() == ErrorKind::NotFound {
                        eprintln!("No config found. Example Config:");
                        eprintln!("{}", toml::to_string_pretty(&Config::default()).unwrap());
                        exit(1);
                    } else {
                        eprintln!("Failed to load config: {e}")
                    }
                },
                Err(e) => eprintln!("Failed to load config: {e}")
            }
        },
    }

    exit(2);
}
