use crate::config::Config;
use crate::nodeapi::grpc_generated::SensorData;
use crate::nodeapi::Client;
use crate::sensors::si7021::SI7021;
use crate::sensors::vl53l0x::VL53L0X;
use crate::sensors::{HumiditySensor, ProximitySensor, TemperatureSensor};
use std::io;
use std::io::ErrorKind;
use std::process::exit;
use std::time::Duration;
use anyhow::Error;
use tokio::sync::mpsc;
use tokio::time;
use tokio::time::sleep;
use crate::util::{DescribeError, GetJoinHandleResult};

mod config;
mod nodeapi;
mod sensors;
mod util;

const NUM_PROXIMITY_READINGS: usize = 5;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut config = load_config();

    let mut temperature_sensor = SI7021::new_from_descriptor("/dev/i2c-1", 0x40)
        .expect("Failed to connect to temperature sensor");

    let mut proximity_sensor = VL53L0X::new_from_descriptor("/dev/i2c-1", 0x29)
        .expect("Failed to connect to proximity sensor");

    loop {
        if let Err(e) = run_app(&mut config, &mut temperature_sensor, &mut proximity_sensor).await {
            eprintln!("Error: {e:?}");
            sleep(Duration::from_secs(1)).await;
        }
    }
}

async fn run_app<T, P>(
    config: &mut Config,
    temperature_sensor: &mut T,
    proximity_sensor: &mut P,
) -> Result<(), Error> where T: TemperatureSensor + HumiditySensor, P: ProximitySensor + Send {
    let mut client = Client::new(config.url.clone())
        .await
        .describe_error("Could not connect to client")?;

    let id;
    if let Some(config_id) = config.id {
        id = config_id;
    } else {
        id = client.assign_id().await.describe_error("Failed to assign ID")?;
        config.id = Some(id);
        config.write_default().describe_error("Failed to write config")?;
    }

    let (client_readings_in, client_readings_out) = mpsc::channel(1);
    let mut sensor_stream = tokio::spawn(async move {
        client
            .stream_sensor_data(client_readings_out)
            .await
    });

    let mut interval_timer = time::interval(Duration::from_secs(1));
    loop {
        // Check if the sensor stream died
        match sensor_stream.get_result().await {
            Some(Ok(Ok(()))) => return Err(Error::msg("Sensor stream stopped without error")),
            Some(Ok(Err(e))) => return Err(Error::new(e).context("Sensor stream stopped")),
            Some(Err(join_error)) => return Err(Error::new(join_error).context("Error checking on sensor stream")),
            None => {},
        }
        interval_timer.tick().await;
        let reading = SensorData {
            id,
            distance: proximity_sensor
                .average_proximity(NUM_PROXIMITY_READINGS)
                .await
                .describe_error("Could not take proximity reading")?,
            temperature: temperature_sensor
                .read_temperature()
                .await
                .describe_error("Could not take temperature reading")?,
            relative_humidity: temperature_sensor
                .read_humidity()
                .await
                .describe_error("Could not take humidity reading")?,
        };
        println!("Reading: {reading:?}");
        if let Err(_) = client_readings_in
            .send(reading)
            .await {
            eprintln!("Warning: Failed to send value. The sensor stream probably shut down")
        }
    }
}

fn load_config() -> Config {
    match Config::load_default() {
        Ok(config) => return config,
        Err(e) => match e.downcast::<io::Error>() {
            Ok(e) => {
                if e.kind() == ErrorKind::NotFound {
                    eprintln!("No config found. Example Config:");
                    eprintln!("{}", toml::to_string_pretty(&Config::default()).unwrap());
                    exit(1);
                } else {
                    eprintln!("Failed to load config: {e}")
                }
            }
            Err(e) => eprintln!("Failed to load config: {e}"),
        },
    }

    exit(2);
}
