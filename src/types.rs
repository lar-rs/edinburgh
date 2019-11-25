#[allow(unused_imports)] // Required for no_std
use num_traits::float::FloatCore;

/// A measurement result from the sensor.
#[derive(Debug, PartialEq, Clone)]
pub struct Measurement {
    /// CO₂ equivalent (parts per million, ppm)
	pub co2eq_ppm: u16,
    /// Total Volatile Organic Compounds (parts per billion, ppb)
    pub tvoc_ppb: u16,

    pub fsr: f32
}

/// A raw signals result from the sensor.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RawSignals {
    /// H2 signal
	pub h2: u16,
    /// Ethanol signal
	pub ethanol: u16,
}

/// The baseline values.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Baseline {
    /// CO₂eq baseline
	pub co2eq: u16,
    /// TVOC baseline
	pub tvoc: u16,
}



/// The product types compatible with this driver.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ProductType {
    /// Edinburgh500
    Edinburgh500,
    /// Edinburgh1%
    Edinburgh1ppm,
    /// Edinburgh3%
    Edinburgh3ppm,
    /// Unknown product type
    Unknown(u8),
}

impl ProductType {
    /// Parse the product type.
    pub fn parse(val: u8) -> Self {
        match val {
            0 => ProductType::Edinburgh500,
            1 => ProductType::Edinburgh500,
            2 => ProductType::Edinburgh500,
            _ => ProductType::Unknown(val),
        } 
    }
}

/// The feature set returned by the sensor.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureSet {
    /// The product type (see [`ProductType`](enum.ProductType.html))
    pub product_type: ProductType,
    /// The product version
    pub product_version: u8,
}

impl FeatureSet {
    /// Parse the two bytes returned by the device.
    pub fn parse(msb: u8, lsb: u8) -> Self {
        FeatureSet {
            product_type: ProductType::parse(msb >> 4),
            product_version: lsb,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32;

    use super::*;
}
