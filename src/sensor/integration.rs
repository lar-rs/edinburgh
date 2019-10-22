//! Integration parameters,records,calculation
//! * Justification - time to emit last zero value.
//!
use serde::{Deserialize, Serialize};
use super::{Sensor}

use chrono::prelude::*;

use std::time::Duration;
use async_std::os::unix::net::UnixStream;
use async_std::fs;
use async_std::io;
use async_std::prelude::*;
// use async_std::task;
use async_std::stream;
use async_std::stream::Stream;
use std::pin::Pin;
/// Integration
///
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    pub justification: u32,
    pub start_treshold: f64,
    pub stop_treshold: f64,
    pub min_start: u32,
    pub min_stop: u32,
    pub max_stop: u32,
}


impl Default for Config {
    fn default() -> Self {
        Self {
            justification: 15,
            start_treshold: 0.002,
            stop_treshold: 0.003,
            min_start: 10,
            min_stop: 60,
            max_stop: 210,
        }
    }
}



pub async fn read(sensor::Sensor) -> io::Result<Config>{
    let path = sensor.path.join("integration");

}


pub async fn record () -> io::Result<()> {
    Ok(())
}
