/// Humidity sensor
/// Anschlus Analog:IN04
///
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use async_std::fs;
use async_std::io;
use async_std::prelude::*;
use serde_json::from_slice;
use lazy_static::lazy_static;

use std::path::PathBuf;


#[derive(Serialize,Default, Deserialize, Clone, Debug)]
pub struct Config {
    pub scale:                f32,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Humidity {
    pub updated:   u64,
    pub broken :   bool,
    pub value  :   f32,
}



impl Default for Humidity {
    fn default() -> Self {
        Self {
            updated: Utc::now().timestamp_millis() as u64,
            value: 0.0,
            broken: true,
        }
    }
}

impl Humidity {
    pub fn from_analog16(value: u16) -> Humidity {
        let signal =  value as f32 / 4096.0 * 5.0;
        Humidity::from_voltage(signal)
    }
    pub fn from_voltage(voltage:f32) -> Humidity {
        let broken = voltage < 0.8 * 4.0 / 5.0;
        let value  = ((voltage - 0.8)  / (3.6 - 0.8))*100.0;
        Humidity {
            updated:   Utc::now().timestamp_millis() as u64,
            value:     value,
            broken:    broken,
        }
    }
}

pub fn workdir() -> PathBuf {
    let path = super::workdir().join("/humidity/");
    // if !path.exists() {
        // fs::DirBuilder::new()
            // .recursive(true)
            // .create(path.as_path())
            // .await?;
        // info!("{:} new creat", Paint::cyan("MIO:airflow"));
    //
    path
}

pub async fn read_config() -> io::Result<Config>{
    let path = workdir().join("config");
    let mut file = fs::File::open(path.as_path()).await?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await?;
    let config: Config = from_slice(buf.as_slice())?;
    Ok(config)
}

pub async fn write_config(config:&Config) -> io::Result<()> {
    let path = workdir().join("config");
    let mut file = fs::File::create(path.as_path()).await?;

    // airflow.write(path)?;
    Ok(())
}


pub async fn signal(config:&Config) -> io::Result<Humidity> {
    let path = workdir().join("signal");
    let mut file = fs::File::open(path.as_path()).await?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await?;
    let humidity:Humidity = from_slice(buf.as_slice())?;
    Ok(humidity)
}
