use serial::SystemPort;
pub mod error;
pub mod config;

pub type Result<T> = nb::Result<T, error::Error>;

use core::time::{Duration};
use lazy_static::lazy_static;
use config::Config;
use nb;
use regex::Regex;

pub fn hello()  {
    use ansi_term::Colour;

    println!(r#" {:} "#,Colour::Blue.paint("  ███████╗██████╗ ██╗███╗   ██╗██████╗ ██╗   ██╗██████╗  ██████╗ ██╗  ██╗   "));
    println!(r#" {:} "#,Colour::Blue.paint("  ██╔════╝██╔══██╗██║████╗  ██║██╔══██╗██║   ██║██╔══██╗██╔════╝ ██║  ██║   "));
    println!(r#" {:} "#,Colour::Blue.paint("  █████╗  ██║  ██║██║██╔██╗ ██║██████╔╝██║   ██║██████╔╝██║  ███╗███████║   "));
    println!(r#" {:} "#,Colour::Blue.paint("  ██╔══╝  ██║  ██║██║██║╚██╗██║██╔══██╗██║   ██║██╔══██╗██║   ██║██╔══██║   "));
    println!(r#" {:} "#,Colour::Blue.paint("  ███████╗██████╔╝██║██║ ╚████║██████╔╝╚██████╔╝██║  ██║╚██████╔╝██║  ██║   "));
    println!(r#" {:} "#,Colour::Blue.paint("  ╚══════╝╚═════╝ ╚═╝╚═╝  ╚═══╝╚═════╝  ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝   "));
    println!(r#" {:} "#,Colour::Blue.paint("   NDIR sensor driver  "));
    // println!(r#" "#);
    // println!(r#" "#);
    // println!(" {:}  ",Colour::Blue.paint(format!("Number of logical cores is {}",num_cpus::get())));
    // println!(r#"  ENVIRONMENTAL MONITORING  "#);
}

/// Supported measure ranges
pub enum Range {
    Rang500   = 500,
    Range2000 = 2000,
    Range5000 = 5000,
}


pub type Scale = f64;


const TEST_DATA: &'static str  =  "N 0.0414 0.0000 0.0000 0.00 0.0000 22942 992.6";
const READ_WAIT: Duration = Duration::from_millis(500);
const READ_TIMEOUT: Duration = Duration::from_millis(1500);


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

/// 🧰 edibnurgh sensor
/// ## Usage
///
/// ```rust,no_run
/// use edinburgh;
///
/// fn main() {
///     let mut edin = Edinburgh::open("/dev/ttyUSB0").unwrap();
///     println!("CO₂ readout: {} ppm", edinburgh.ppm().unwrap());
/// }
/// ```
///

pub struct Edinburgh {
    // port: SystemPort,
    duration:   u64,
    scale:      Scale,
    timestamp:  u64,
    signal:     Vec<Signal>,
}


// connect return unix socket
// pub fn connect() -> io::Result<()> {
    // Ok(())
// }


impl Edinburgh {
    fn open (config: Config) -> Result<Edinburgh> {
        let edin = Edinburgh{
            duration: 1,
            scale: 1.0,
            timestamp: 0,
            signal: Vec::new(),
        };
        Ok(edin)
    }
    fn fsr(&mut self) -> Result<f64> {
        Ok(0.0)
    }

    fn ppm(&mut self) -> Result<f64> {

        Ok(0.0)
    }
}

