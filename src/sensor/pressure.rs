/// Pressure sensor
/// Anschlus:  `Analog:IN04`
/// Model:     `presurei877`
///
///
// use super::node::Node;
// use failure::Fallible;

use async_std::fs;
use async_std::io;
use async_std::prelude::*;
use serde_json::from_slice;
use serde_json::to_vec;
use chrono::prelude::*;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use super::*;
use std::path::PathBuf;
// use crate::error::*;

lazy_static::lazy_static! {
    static ref INTERVAL:u64 = 2000;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub val: f32,
    pub brocken: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Pressure {
    pub updated: u64,
    pub value: f32,
    pub brocken: bool,
    pub config: Config,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    pub warn_level: f32,
    pub crit_level: f32,
    pub interval: u64,
    pub scale: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            warn_level:  300.0,
            crit_level:  600.0,
            interval:   0,
            scale: 1.0,
        }
    }
}

impl Pressure {
    // pub fn from_analog16(value: u16) -> Pressure {
    //     let voltage =  value as f32 / 4096.0 * 5.0;
    //     Self::from_voltage(voltage)
    // }

    // pub fn from_voltage(voltage: f32) ->  Pressure{
    //     let brocken = voltage< 1.0 * 4.0 / 5.0;
    //     let val = ((voltage - 1.0)  / (5.0 - 1.0))*1000.0;
    //     Pressure{
    //         updated: Utc::now().timestamp_millis() as u64,
    //         value:   val,
    //         brocken: brocken,
    //     }
    // }
}

pub fn workdir() -> PathBuf {
    let path = super::workdir().join("/pressure/");
    path
}



pub async fn signal(config:&Config) -> io::Result<f32> {
    let path = workdir().join("signal");
    let mut file = fs::File::open(path.as_path()).await?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await?;
    let pressure:Pressure = from_slice(buf.as_slice())?;
    Ok(0.0 as f32)
}

// pub async fn check(sensor:Sensor) -> io::Result<bool> {

// }

pub async fn pressure() -> Result<Sensor> {
    let path = workdir();
    let sensor = sensor(path.as_path()).await?;
    Ok(sensor)
}

