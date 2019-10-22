/// Airlow sensor
/// IN  Anschlus `Analog:IN01`
/// OUT Anschlus `Analog:IN02`
///


// use rand::prelude::*;
use async_std::fs;
use async_std::io;
use async_std::prelude::*;
use serde_json::{from_slice,to_vec};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;


// use lazy_static::lazy_static;
// use std::sync::RwLock;

// lazy_static! {
    // static ref IN : RwLock<Airflow> = {
        // RwLock::new(Airflow::from_analog16(0))
    // };
    // static ref OUT : RwLock<Airflow> = {
        // RwLock::new(Airflow::from_analog16(0))
    // };
// }


// static max:f64 = 0.5;
// static
// static max:f64 = 0.4;

/// für 0..60:   0.230197;
static A6:f32 = 0.003836617;
// für 0..60:  -3.616438;
static A5:f32 = -0.06027397;
// für 0..60:  22.36370;
static A4:f32 = 0.3727283;
// für 0..60: -68.58285;
static A3:f32 = -1.1430475;
// für 0..60: 110.3052;
static A2:f32 = 1.83842;
// für 0..60: -84.19201;
static A1:f32 = -1.4032;
// für 0..60:  23.49542;
static A0:f32 = 0.39159;


#[derive(Serialize,Default, Deserialize, Clone, Debug)]
pub struct Config {
    pub correlation:          f32,
    pub soll_value:           f32,
    pub input_deviation:      f32,
    pub output_deviation:     f32,
    pub monitorin_interval:   u64,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Airflow {
    updated: u64,
    input:   f32,
    output:  f32,
    brocket: u8,
}

impl Airflow {
    pub fn from_voltage(mut self,input:f32,output:f32) -> Airflow{
        let broket  = if input > 1.0 {
            1
        } else if output < 1.0 {
            2
        } else {
            0
        };
        let airout:f32  = 60.0 * ((((((A6*output +A5)*output +A4) *output +A3) *output + A2) * output  + A1) *output  + A0);
        let airin:f32   = 60.0 * ((((((A6*input +A5)*input +A4) *input +A3) *input + A2) * input + A1) *input + A0);
        Airflow {
            updated: Utc::now().timestamp_millis() as u64,
            input:   airin,
            output:  airout,
            brocket:  broket,
        }
    }
    pub fn from_analog16(mut self,input:u16,output:u16) -> Airflow{
        let voltagein  = input as f32 / 4095.0 * 5.0;
        let voltageout = output as f32 / 4095.0 * 5.0;
        self.from_voltage(voltagein,voltageout)
    }
}



pub fn workdir() -> PathBuf {
    let path = super::workdir().join("airflow/");
    // if !path.exists() {
        // fs::DirBuilder::new()
            // .recursive(true)
            // .create(path.as_path())
            // .await?;
        // info!("{:} new creat", Paint::cyan("MIO:airflow"));
    // }
    path
}

// pub async fn airflow() -> io::Result<super::Sensor> {
    // Ok()
// }


// pub async fn signal(sensor:&Sensor) -> io::Result<Airflow> {
    // let path = workdir().join("input");
    // let mut file = fs::File::open(path.as_path()).await?;
    // let mut buf = Vec::new();
    // file.read_to_end(&mut buf).await?;
    // let airflow:Airflow = from_slice(buf.as_slice())?;
    // Ok(airflow)
// }




// pub async fn create()
// pub async fn warning_input() -> Option<Range> {
    // None
// }
// pub async fn critical_range_input() -> Option<Range> {
    // None
// }

// pub async fn setup_range_input(warn: Option<Range>, critical : Option<Range>) {

// }



// pub async fn airflow_input_value(input_read: ReadAnalogu16) -> Result<Airflow,MioError>{
//     let analog_value = input_read.await?;
//     Ok(Airflow::from_analog16(analog_value))
// }
// pub async fn airflow_output_value(output_read:ReadAnalogu16) -> Result<Airflow,MioError>{
//     let analog_value = output_read.await?;
//     Ok(Airflow::from_analog16(analog_value))
// }
