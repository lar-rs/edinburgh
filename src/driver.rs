
use mio::Result;

use embedded_hal as hal;
// extern crate num_traits;
use serde::{Serialize,Deserialize};

// use byteorder::{BigEndian, ByteOrder};
// use hal::blocking::delay::{DelayMs, DelayUs};
// use hal::blocking::serial::{Read, Write, WriteRead};
// use hal::serial;
use hal::serial::{Read};
use lazy_static::lazy_static;
use regex::Regex;


// Testdate edinburgh
// const TEST_DATA: &'static str  =  "N 0.0414 0.0000 0.0000 0.00 0.0000 22942 992.6";
// Regex::new(r"N (?P<fsr>\d{1}.\d{4}) \d{1}.\d{4} \d{1}.\d{4} \d{1}.\d{2} \d{1}.\d{4} (?P<dig>\d{5}) (?P<ppm>\d{1}.\d{4}) \d{1}").unwrap()*
// -- Listenmakros für Aufzählungen
// #define BAUD_RATES(g,f,d) \
// 	g(9600,   d) \
// 	f(1200,   d) \
// 	f(2400,   d) \
// 	f(4800,   d) \
// 	f(9600,   d) \
// 	f(19200,  d) \
// 	f(38400,  d) \
// 	f(57600,  d) \
// 	f(115200, no)
#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct Measurement{
    fsr: f32,
    ppm: f32,
    dig: u32,
}

// /// All possible errors in this crate
// #[derive(Debug)]
// pub enum Error {
//     /// Serial communication error
//     Serial,
//     /// CRC checksum validation failed
//     Crc,
//     /// User tried to measure the air quality without starting the
//     /// initialization phase.
//     NotInitialized,
//      /// User tried to measure the air quality without starting the
//     /// initialization phase.
//     NoData,
// }



impl std::fmt::Display for Measurement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fsr:{} dig:{} ppm:{}", self.fsr, self.ppm, self.dig)
    }
}

pub struct Driver<S> {
    /// The concrete I²C device implementation.
    s: S,
}


impl<S, E> Driver<S>
where
    S: Read<String, Error = E>,
{
    /// Create a new instance of the Edinburgh driver.
    pub fn new(s: S) -> Self {
        Driver{
            s:s,
        }
    }

    /// Get measurement.
    ///
    /// Before calling this method, the air quality measurements must have been
    /// initialized using the [`init()`](struct.Edinburgh.html#method.init) method.
    /// Otherwise an [`Error::NotInitialized`](enum.Error.html#variant.NotInitialized)
    /// will be returned.
    pub fn measure(&mut self) -> nb::Result<Measurement,E> {
        // Send command to sensor
        let mut buffer = String::from("");
        buffer.push_str(self.s.read()?.as_str());
        println!("uart string {}",buffer);
        let values:Vec<_> = buffer.split(r#"\r\n"#).collect();
        for v in values {
            // println!("data: {}",v);
            if let Some(measure) = self.decode(v) {
                return Ok(measure)
            }else {
                log::warn!("{} wrong format",&v);
            }
        }
        // Max duration according to datasheet (Table 10)
        Ok(Measurement{
            fsr:0.0,
            ppm:0.0,
            dig:0,
        })
 
    }
    pub fn decode(&self , input: &str) -> Option<Measurement> {
        lazy_static! {
                                                // N 0.0384                0.0000        0.0000      0.00        0.0000       25764             997.2           0
            static ref RE: Regex = Regex::new(r"N (?P<fsr>\d{1}.\d{4}) \d{1}.\d{4} \d{1}.\d{4} \d{1}.\d{2} \d{1}.\d{4} (?P<dig>\d{5}) (?P<ppm>\d{1,5}.\d{1}) \d{1}").unwrap();
        }
        RE.captures(input).and_then(|cap| {
            let fsr = cap.name("fsr").map(|fsr| fsr.as_str().parse::<f32>().unwrap_or(0.0)).unwrap();
            let dig = cap.name("dig").map(|dig| dig.as_str().parse::<u32>().unwrap_or(0)).unwrap();
            let ppm = cap.name("ppm").map(|ppm| ppm.as_str().parse::<f32>().unwrap_or(0.0)).unwrap();
            Some(Measurement{fsr,ppm,dig})
        })
    }
}