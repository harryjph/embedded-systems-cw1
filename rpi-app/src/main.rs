use crate::config::Config;
use crate::nodeapi::grpc_generated::DistanceData;
use crate::nodeapi::Client;
use crate::sensors::vl53l0x::VL53L0X;
use anyhow::Error;
use std::io;
use std::io::ErrorKind;
use std::process::exit;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;

mod config;
mod nodeapi;
mod sensors;
mod util;

const NUM_PROXIMITY_READINGS: usize = 5;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut config = load_config();

    let mut proximity_sensor = VL53L0X::new_from_descriptor("/dev/i2c-1", 0x29)
        .expect("Failed to connect to proximity sensor");

    let mut client = Client::new(config.url.clone())
        .await
        .expect("Failed to connect to server");

    if let None = config.id {
        config.id = Some(client.assign_id().await.expect("Failed to assign ID"));
        config.write_default().expect("Failed to write config");
    }

    let (client_readings_in, client_readings_out) = mpsc::channel(1);
    tokio::spawn(async move {
        client.report_distance(client_readings_out).await.unwrap();
    });

    let mut interval_timer = time::interval(Duration::from_secs(5));
    loop {
        let reading = DistanceData {
            id: config.id.unwrap(),
            distance: sensors::average_proximity(&mut proximity_sensor, NUM_PROXIMITY_READINGS)
                .await
                .expect("Could not take proximity reading"),
        };
        println!("Reading: {reading:?}");
        client_readings_in
            .send(reading)
            .await
            .expect("Failed to send reading");
        interval_timer.tick().await;
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
