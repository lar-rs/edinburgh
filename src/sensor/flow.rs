/// Temperatur sensor
/// Anschlus:  `Analog:IN04`
/// Model:     `presurei877`
///
use serde_derive::{Deserialize, Serialize};
use analyzer::{
    Airflow,
    Humidity,
    Pressure,
    flow::*,

};
use crate::error::*;
use super::io;
use rand::prelude::*;

use lazy_static::lazy_static;
// use std::sync::RwLock;
lazy_static! {
    static ref VAL:f32 = 0.0;
    // static ref SETTINGS:Setting = Setting::default();
    // static ref AIN5v: Box<impl Future<Output=Result<f32>>> =  simulate_5v();

    // static ref SENSOR : RwLock<Humidity> = {
        // RwLock::new(Humidity::from_analog16(0))
    // };
    // static ref DATA : RwLock<Vec<f32>> = {
        // RwLock::new(Vec::new())
    // };
}

async fn simulate_5v() -> Result<f32> {
    let mut rng = thread_rng();
    let r5v:f32 = rng.gen_range(1.0, 5.0);
    Ok(r5v)
}

pub async fn calibrate_airflow() -> Result<()> {

    Ok(())
}
pub async fn get_airflow_input() -> Result<Airflow> {
    let value = io::get_analog1_input01().await?;
    Ok(Airflow::from_analog16(value))
}
pub async fn get_airflow_output() -> Result<Airflow> {
    let value = io::get_analog1_input02().await?;
    Ok(Airflow::from_analog16(value))
}
pub async fn get_humidity() -> Result<Humidity> {
    let value = io::get_analog1_input03().await?;
    Ok(Humidity::from_analog16(value))
}
pub async fn get_pressure() -> Result<Pressure> {
    let value = io::get_analog1_input04().await?;
    Ok(Pressure::from_analog16(value))
}






//
