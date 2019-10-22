use serial::SystemPort;
use std::ffi::OsStr;
use std::io::{Write, Read};
use crate::error::Result;
use std::time::{Duration, Instant};
use lazy_static::lazy_static;
use std::str::FromStr;
use regex::Regex;
use chrono::{DateTime, Utc};



pub type Scale = f64;

pub struct Edinburgh {
    port:       SystemPort,
    duration:   Duration,
    scale:      Scale,
    timestamp:  u64,
    signal:     Vec<Signal>
}

/// Supported measure ranges
pub enum Range {
    Rang500   = 500,
    Range2000 = 2000,
    Range5000 = 5000,
}

const TEST_DATA: &'static str  =  "N 0.0414 0.0000 0.0000 0.00 0.0000 22942 992.6";

const READ_WAIT: Duration = Duration::from_millis(500);
const READ_TIMEOUT: Duration = Duration::from_millis(1500);

const PORT_SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate:    serial::Baud9600,
    char_size:    serial::Bits8,
    parity:       serial::ParityNone,
    stop_bits:    serial::Stop1,
    flow_control: serial::FlowNone,
};

lazy_static! {
    static ref RE: Regex = {
        Regex::new(r"N (?P<fsr>\d{1}.\d{4}) \d{1}.\d{4} \d{1}.\d{4} \d{1}.\d{2} \d{1}.\d{4} (?P<dig>\d{5}) (?P<ppm>\d{1}.\d{4}) \d{1}").unwrap()
    };
}


pub struct Signal{
    fsr: f64,
    ppm: f64,
    dig: u64,
}

impl std::fmt::Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fsr:{} dig:{} ppm:{}", self.fsr, self.ppm, self.dig)
    }
}



/// 🧰  Edibnurgh sensor
/// Edinburg rs232 CO₂ sensor
///
/// ## Usage
///
/// ```rust,no_run
/// use edinburgh;
///
/// fn main() {
///     let mut edin = Edinburgh::open("/dev/ttyUSB0").unwrap();
///     println!("CO₂ readout: {} ppm", edinburgh.read().unwrap());
/// }
/// ```
///
impl Edinburgh {
    /// Connect to the mh-z19 at the specified serial port
    /// Open edinburgh port
    pub fn open<T: AsRef<OsStr> + ?Sized>(port: &T) -> Result<Self> {
        let edin = Edinburgh{
            port: serial::open(port)?,
            duration:READ_WAIT,
            scale: 0.0,
            signal: Vec::new(),
            timestamp: Utc::now().timestamp() as u64,

        };
        Ok(edin)
    }

    /// Read the CO2 value from the meter as ppm
    ///
    /// ## Blocking
    ///
    /// This command will wait for 100ms between sending the read command and getting the response
    /// during this the thread is blocked
    ///
    /// If the crc check of the response fails the method will retry up to 8 times
    pub fn read(&mut self) -> Result<Vec<Signal>> {
        // let command = MHZ19::generate_command(Command::Read, 0, 0);
        // let mut buffer : Vec<u8> = Vec::new();
        // let mut crc_err_count = 0;
        let start = Instant::now();

        // self.buffer.clear();
        loop {
            // self.port.write(&command)?;
            let mut buffer : Vec<u8> = Vec::new();
            std::thread::sleep(self.duration);
            self.port.read(&mut buffer)?;
            let mut pre = String::from_utf8(buffer).unwrap();
            // buffer.append();
            // let st = buffer;
            // println!("match: {}",RE.is_match());
            let signal = RE.captures_iter(&pre).filter_map(|cap| {
                let groups = (cap.name("fsr"), cap.name("dig"), cap.name("ppm"));
                match groups {
                    (Some(fsr), Some(dig), Some(ppm)) => Some(Signal{
                        fsr:f64::from_str(fsr.as_str()).unwrap(),
                        ppm:f64::from_str(ppm.as_str()).unwrap(),
                        dig:u64::from_str(dig.as_str()).unwrap(),
                    }),
                    _ => None,
                }
            });
            // Ok(signals)
            // if start.elapsed() > READ_TIMEOUT {
                // return Err(format_err!("Edinburg decoder match Regex fail"));

            // }
        }
        // Err(format_err!("r match Regex fail"))
    }
    /// ≞ signal ppm
    pub fn signal(&self) -> Vec<Signal> {
        // self.signal.clone()
    }
}


    //   // match RE.captures(src) {
            // Some(caps) => {
                // let fsr_str = caps.name("fsr").unwrap().as_str();
                // println!("fsrstr:{}",fsr_str);
                // println!("caps[0]:as_str {}", &caps[0]);
                // let a:f64 = f64::from_str(fsr_str)?;
                // self.buffer.clear();
                // println!("a:{} caps[1]: {}",a, &caps[1]);
                // println!("caps[2]: {}", &caps[2]);
                // self.fsr = a * self.scale;
                // Ok(true)
            // },
            // None => {
                // self.buffer.clear();
                // Err(format_err!("Edinburg parce captures fail"))
            // }
        // }
// }
