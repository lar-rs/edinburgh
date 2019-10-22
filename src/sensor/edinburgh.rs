//! Edinburg access
//!
//! The methods exposed by this library are centered around
//! the `Pin` struct and map pretty directly the API exposed
//! by the kernel in syfs (https://www.kernel.org/doc/Documentation/gpio/sysfs.txt).
//!
//! # Examples
//!
//! Typical usage for systems where one wants to ensure that
//! the pins in use are unexported upon completion looks like
//! the following:
//!
//! ```no_run
//! extern crate sysfs_gpio;
//!
//! use sysfs_gpio::{Direction, Pin};
//! use std::thread::sleep;
//! use std::time::Duration;
//!
//! fn main() {
//!     let my_led = Pin::new(127); // number depends on chip, etc.
//!     my_led.with_exported(|| {
//!         loop {
//!             my_led.set_value(0).unwrap();
//!             sleep(Duration::from_millis(200));
//!             my_led.set_value(1).unwrap();
//!             sleep(Duration::from_millis(200));
//!         }
//!     }).unwrap();
//! }
//! ```

use serde::{Deserialize, Serialize};

// use crate::error::*;
use super::Scale;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Model {
    Edin500,
    Edin1000,
    Edin3000,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Edinburgh {
    pub model: Model,
    scale: Scale,
}

impl Default for Edinburgh {
    fn default() -> Edinburgh {
        Edinburgh {
            model: Model::Edin500,
            scale: Scale::new(1.0),
        }
    }
}



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config{
    pub name: String,
    pub interval: u64,
    pub scale: Vec<Scale>,

}





// pub async fn decode_uart_data(read: impl Future<Output= Result<Vec<u8>>) -> Result<f64> {

//     Ok(0.0))
// }


// pub async fn setup_ndir1(min:f64,max:f64,read: impl Future<Output = Result<f64>>) -> Result<()> {
// pub async fn setup_ndir1(min:f64,max:f64) -> Result<()> {
    // Ok(())
// }
// pub async fn setup_ndir2(min:f64,max:f64) -> Result<()> {
    // Ok(())
// }


// pub async fn get_ndir1_value() -> Result<f64> {
    // Ok(0.0)
// }
// pub async fn get_ndir2_value() -> Result<f64> {
    // Ok(0.0)
// }
