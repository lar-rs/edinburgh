// use std::fmt;
// pub use uom::si::f32::{Ratio,MassRate,ElectricCurrent};
use super::setting;
use super::signal;
use async_std::prelude::*;
use async_std::fs;
use async_std::path::{ Path, PathBuf};
use async_std::stream;

use serde_json::from_str;
use serde::{Deserialize, Serialize};

// use crate::sys;
// use lazy_static::lazy_static;

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub enum State {
//     Brocket,
//     Signal,
//     Average,
//     Integration,
// }

// #[derive(Serialize,Deserialize, Clone, Debug)]
// pub struct Signal {
// /// time in mili seconds
//     pub timestamp:   u64,
// /// Value
//     pub value:  f64,
// }

// impl Signal {
//     pub fn random() -> Signal {
//         Signal{
//             time:Utc::now().timestamp_millis() as u64,

//         }
//     }
// }



/// Hardware airflow sensor.
pub struct NdirSensor {
    pub unit:     String,
    pub label:    String,
    pub address:  String,
    pub path:     PathBuf,
    pub current:  f32,
}

impl NdirSensor {
    /// Returns sensor unit name.
    pub fn unit(&self) -> &str {
        self.unit.as_ref().map(|s| s.as_str())
    }

    /// Returns sensor label.
    pub fn label(&self) -> Option<&str> {
        &self.label
    }

    /// Returns current presure reported by sensor.
    pub fn current(&self) -> f32 {
        self.current
    }

    pub fn signal(&mut self) -> impl Stream<Item = Result<f32>> {
        let path = self.path.join("current");
        let s = stream::repeat_with(|| async {
            let mut rng = rand::thread_rng();

            // let signal  = fs::read_to_string(&path.join("current")).await?;
            // let current:Signal = from_str(&signal)?;
            // current;
            Ok(rng.gen::<f32>())

         });
        pin_utils::pin_mut!(s);
        curr
    }
}

pub async fn model<T: AsRef<Path>>(path: T) -> Result<String> {
    let model = fs::read_to_string(path.join("ndir")).await?;
    Ok(model)
}

pub async fn read<T: AsRef<Path>>(path: T) -> Result<NdirSensor> {
    let model    = model(path).await?;
    let unit     = setting::unit(path).await?;
    let label    = setting::label(path).await?;
    let adderess = setting::address(path).await?;
    let current  = signal::current(path).await?;
    Ok(NdirSensor{unit,label,address,path,current})
}


// pub async fn set_addres(pa)

pub async fn simulation(path: PathBuf) {
}

// pub fn fsr(sensor: NdirSensor) -> impl Stream<Item = Result<f32>>
