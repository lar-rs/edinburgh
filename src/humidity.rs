pub use uom::si::f32::{Ratio};


// use crate::sys;


/// Hardware airflow sensor.
pub struct HumiditySernsor {
    pub(crate) unit: String,
    pub(crate) label: String,
    pub(crate) current: f32,
    // pub(crate) high: Option<Ratio>,
    // pub(crate) critical: Option<Ratio>,
}

impl HumiditySernsor {
    /// Returns sensor unit name.
    pub fn unit(&self) -> &str {
        &self.unit
    }

    /// Returns sensor label.
    pub fn label(&self) -> &str {
        &self.unit
    }

    /// Returns current presure reported by sensor.
    pub fn current(&self) -> f32 {
        self.current
    }

    /// Returns high trip point for sensor if available.
    // pub fn high(&self) -> Option<Ratio> {
        // self.high
    // }

    /// Returns critical trip point for sensor if available.
    // pub fn critical(&self) -> Option<Ratio> {
        // self.critical
    // }
}



