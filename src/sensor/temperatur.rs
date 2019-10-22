/// Temperatur sensor
/// Anschlus:  `Analog:TEMP`
///
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TemperaturSetting {
    pub monitorin_interval:   u64,
    // pub injection_threshold:  f32,
}



impl Default for TemperaturSetting {
    fn default() -> Self {
        Self {
            monitorin_interval:   0,
        }
    }
}



/// Presure value model
///
/// fsr - full scale range
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Temperatur {
    pub value: f32,
    pub broken: bool,
}

impl Temperatur {
    pub fn from_analog16(value : u16) -> Temperatur {
        let signal =  value as f32 / 10.0;
        let broken = value>1000;

        Temperatur{
            value: signal,
            broken: broken,
        }
    }
}


// pub async fn setup() -> Result<(),MioError> {
    // Ok(())
// }
//
// pub async fn Temperatur_value() -> Result<Temperatur, MioError> {
    // let analog_value  = io::analog_input16(0x4).await?;
    // Ok(Temperatur::from_analog16(analog_value))
// }
//
