use std::fmt;
// pub use uom::si::f32::{Ratio, Presure};
use async_std::prelude::*;
use async_std::stream;


pub type Presure = f32;


/// Hardware presure sensor.
pub struct PresureSensor {
    pub(crate) unit: String,
    pub(crate) label: Option<String>,
    pub(crate) current:Presure ,
    pub(crate) high: Option<Presure>,
    pub(crate) critical: Option<Presure>,
}

impl PresureSensor{
    /// Returns sensor unit name.
    pub fn unit(&self) -> &str {
        &self.unit
    }

    /// Returns sensor label.
    pub fn label(&self) -> Option<&str> {
        self.label.as_ref().map(|s| s.as_str())
    }

    /// Returns current presure reported by sensor.
    pub fn current(&self) -> Presure {
        self.current
    }

    /// Returns high trip point for sensor if available.
    pub fn high(&self) -> Option<Presure> {
        self.high
    }

    /// Returns critical trip point for sensor if available.
    pub fn critical(&self) -> Option<Presure> {
        self.critical
    }
}

