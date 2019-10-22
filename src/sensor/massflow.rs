/// Temperatur sensor
/// Anschlus:  `Analog:IN04`
/// Model:     `presurei877`
///
// use serde_derive::{Deserialize, Serialize};
use analyzer::{
    Airflow,
    Humidity,
    Pressure,
    // flow::*,

};
use crate::error::*;
use super::io;

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




pub async fn get_pressure() -> Result<Pressure> {
    let value = io::get_analog1_input04().await?;
    Ok(Pressure::from_analog16(value))
}






//
