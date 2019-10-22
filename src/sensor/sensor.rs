use serde::{Deserialize, Serialize};
// use std::future::Future;
use std::fmt;
// use std::pin::Pin;
use settings::ron::Config;
// use csv;
// use std::{
    // fs,
    // path::PathBuf,
// };
use lazy_static::lazy_static;
// use std::sync::RwLock;
lazy_static! {
    static ref MIN:f32 = 0.0;
    static ref MAX:f32 = 0.0;

    // static ref SETTINGS:Setting = Setting::default();
    // static ref AIN5v: Box<impl Future<Output=Result<f32>>> =  simulate_5v();

    // static ref SENSOR : RwLock<Humidity> = {
        // RwLock::new(Humidity::from_analog16(0))
    // };
    // static ref DATA : RwLock<Vec<f32>> = {
        // RwLock::new(Vec::new())
    // };
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Record {
/// time in mili seconds
    pub time:   u64,
/// Pereodic
    pub value:  u64,
/// state
    pub state: u8,
}



impl Default for Record {
    fn default() -> Self {
        Self {
            start:  0,
            period: 300,
            data:  Vec::new(),
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Signal {
    pub value: f64,
    pub timestamp: u64,
}

impl Default for Signal{
    fn default() -> Self {
        Self {
            value:   0,
            brocken:true,
        }
    }
}

pub enum Cmd {
    Status,
    Tick(u64),
    Recod(u64),
}

pub enum Resive {
    Status,
    Signal()
}


pub type Range = std::ops::Range<f64>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AverageSignal {
//sart time
    pub start:   u64,
// Pereodic
    pub meas_id:  u64,
// Sensor data
    pub value  :  f64,
}


impl Default for AverageSignal{
    fn default() -> Self {
        Self {
            start:   0,
            meas_id: 0,
            value:   0.0,
        }
    }
}

impl AverageSignal {
    fn new(meas_id:u64) -> AverageSignal {
         Self {
            start:   0,
            meas_id: 0,
            value:   0.0,
        }
    }

}

/// Sensor model
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Model {
    Unimplemented,
    Simulation,
    Edinburgh,
    Aide,
    No,
    Zirox
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Model::Simulation => write!(f, "simulation"),
            Model::Edinburgh => write!(f, "edinburgh"),
            Model::Aide => write!(f, "aide"),
            Model::No => write!(f, "aide"),
            Model::Zirox => write!(f, "aide"),
            _ => write!(f, "not supported"),
        }
    }
}

impl Default for Model {
    pub fn default() -> Model {
        Model::Unimplemented
    }
}




impl From<String> for Model {
    #[inline]
    fn from(s: String) -> Model {
        match *s {
            "simulation" => Model::Simulation,
            "edinburgh" => Model::Edinburgh,
             _  => Model::Unimplemented,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sensor {
    pub value: f64,
    pub brocked: bool,
    pub model: Model,

}


impl Default for Sensor {
    fn default() -> Self {
        Sensor::new(1)
    }
}

impl Sensor {
    pub fn  new() -> Sensor  {
        Sensor {
            interval : 0.0,
            model: Model::Simulation,
            range: Range { start: 0.0, end: 1.0 }
        }
    }
}

