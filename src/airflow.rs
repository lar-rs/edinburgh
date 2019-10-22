use std::fmt;
use serde::{Deserialize, Serialize};

pub use uom::si::f32::{Ratio, VolumeRate};

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

// use crate::sys;

/// Hardware airflow sensor.
pub struct AirflowSensor {
    pub(crate) unit: String,
    pub(crate) label: Option<String>,
    pub(crate) input: VolumeRate,
    pub(crate) output: VolumeRate,
    pub(crate) high: Option<VolumeRate>,
    pub(crate) critical: Option<VolumeRate>,
}

impl AirflowSensor {
    /// Returns sensor unit name.
    pub fn unit(&self) -> &str {
        &self.unit
    }

    /// Returns sensor label.
    pub fn label(&self) -> Option<&str> {
        self.label.as_ref().map(|s| s.as_str())
    }

    /// Returns volume rate on input reported by sensor.
    pub fn input(&self) -> VolumeRate {
        self.input
    }
    /// Returns volume rate on input reported by sensor.
    pub fn output(&self) -> VolumeRate {
        self.output
    }

    /// Returns high trip point for sensor if available.
    pub fn high(&self) -> Option<VolumeRate> {
        self.high
    }

    /// Returns critical trip point for sensor if available.
    pub fn critical(&self) -> Option<VolumeRate> {
        self.critical
    }
}

