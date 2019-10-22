pub use uom::si::f32::{Ratio, ThermodynamicTemperature};
pub use uom::si::f64::Time;
// pub use uom::si::u64::{Frequency, Information};
pub use uom::si::{
    frequency, information, information_rate, ratio, thermodynamic_temperature, time,
};

pub mod pressure;
pub mod airflow;
pub mod humidity;
pub mod edinburgh;
pub mod aide;

// use json_serde::from_bytes;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::{from_slice,to_vec};

use chrono::prelude::*;

use std::time::Duration;
use async_std::os::unix::net::UnixStream;
use async_std::fs;
use async_std::io;
use async_std::prelude::*;
// use async_std::task;
use async_std::stream;
use async_std::stream::Stream;
use std::pin::Pin;
use chrono::Utc;
use crate::error::Result;
// use super::Mio;


// type UartData = impl Future<Output = Result<String>>;
// use std::future::Future;
// use std::pin::Pin;
// use settings::ron::Config;
// use csv;
use std::{
    path::{PathBuf,Path},
    fmt,
};

// use lazy_static::lazy_static;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum State {
    Brocket,
    Signal,
    Average,
    Integration,
}

#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct Signal {
/// time in mili seconds
    pub time:   u64,
/// Value
    pub value:  f64,
/// State
    pub state: State,
}



impl Signal {
    pub fn simulation() -> Signal {
        Signal {
            time:Utc::now().timestamp_millis() as u64,
            value: 0.32,// random value 0..1
            state: State::Signal,
        }
    }
    pub fn brocket() -> Signal {
       Signal {
            time:Utc::now().timestamp_millis() as u64,
            value: 0.0,// random value 0..1
            state: State::Brocket,
        }
    }
}


impl Default for Signal {
    fn default() -> Self {
        Self {
            time:  0,
            value: 0.0,
            state: State::Brocket,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Scale  {
    slope: f64,
}

impl Scale {
    pub fn new(slope:f64) -> Scale {
        Scale{
            slope: slope,
        }
    }
}

/// Sensor model
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Model {
    Simulation,
    Edinburgh(edinburgh::Edinburgh),
    Aide(aide::Aide),
    No,
    Zirox
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Model::Simulation => write!(f, "simulation"),
            Model::Edinburgh(edin) => write!(f, "edinburgh"),
            Model::Aide(aide) => write!(f, "aide"),
            Model::No => write!(f, "aide"),
            Model::Zirox => write!(f, "aide"),
            _ => write!(f, "not supported"),
        }
    }
}


#[derive(Serialize,Clone, Deserialize)]
pub struct Sensor {
    path:   PathBuf,
    unit:   String,
    label:  String,
    scale:  Scale,
    current: f64,
}

impl Sensor {
    pub fn simulation(path:&Path) -> Sensor {
        Sensor{
            path:path.to_path_buf(),
            unit: String::from("fsr"),
            label: String::from("simulation"),
            scale: Scale::new(1.0),
            current: 0.0,
        }
    }
    pub fn unit(&self) -> &str {
        &self.unit
    }
    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn scale(&self) -> &Scale {
        &self.scale
    }
    pub fn current(&self) -> f64 {
        self.current
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config{
    pub name: String,
    pub interval: u64,
    pub scale: Vec<Scale>,

}

/// Return sensors driver directory
///
pub fn workdir() -> PathBuf {
    super::workdir().join("sensors")
}

pub async fn sensor(path: &Path) -> Result<Sensor> {
    let label = path.join("label");
    let unit  = path.join("unit");
    let scale = path.join("scale");
    // let curent = path.join("curent");
    let label = fs::read_to_string(&label);
    let unit = fs::read_to_string(&unit);
    let scale:Scale = from_slice(fs::read(&scale).await?.as_slice())?;
    let value = current_value(path);
    Ok(Sensor {
        path: path.to_path_buf(),
        label: label.await?,
        unit: unit.await?,
        current: value.await?,
        scale: scale,
    })
}

/// Returns stream which yields [ndirs sensors].
///
/// ## Compatibility
///
/// At the moment, this function works only with Linux.
/// For other platforms it returns an empty stream.
///
/// [ndir sensors]: ./struct.Sensor.html
pub async fn sensors() -> impl Stream<Item = Sensor> {
    let path = super::workdir().join("simulation");
    let s = stream::repeat(Sensor::simulation(path.as_path())).take(1);
    s
}

pub async fn current_value<T:DeserializeOwned>(path : &Path) -> io::Result<T> {
    let mut stream = UnixStream::connect(path.join("signal")).await?;
    // stream.reade
    let mut response = Vec::new();
    stream.read_to_end(&mut response).await?;
    let value: T = from_slice(response.as_slice())?;
    // stream.write_all(to_vec(&cmd)?.as_slice()).await?;
    Ok(value)
}


pub async fn read_config<T:DeserializeOwned>(sensor:&Sensor) -> io::Result<T>{
    let path = sensor.path.join("config");
    let mut file = fs::File::open(path.as_path()).await?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await?;
    let config: T = serde_json::from_slice(buf.as_slice())?;
    Ok(config)
}

pub async fn write_config<T:Serialize>(sensor:&Sensor,config:&T) -> io::Result<()> {
    let path = sensor.path.join("config");
    let mut file = fs::File::create(path.as_path()).await?;
    let config = to_vec(config).unwrap();
    file.write_all(config.as_slice()).await?;
    file.sync_data().await?;
    Ok(())
}


impl fmt::Debug for Sensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Sensor")
            .field("unit", &self.unit())
            .field("label", &self.label())
            .field("current", &self.current())
            // .field("high", &self.high())
            // .field("critical", &self.critical())
            .finish()
    }
}
impl fmt::Display for Sensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       f.debug_struct("Sensor")
            .field("label", &self.label())
            .field("unit", &self.unit())
            .field("current", &self.current())
            // .field("high", &self.high())
            // .field("critical", &self.critical())
            .finish()
    }
}

// async fn reader(sensor: &Sensor ) -> Result<()> {
//     let mut stream = UnixStream::connect("/tmp/socket").await?;
//     let (reader, mut writer) = (&stream, &stream); // 1

//     let mut lines_from_server = BufReader::new(reader).lines().fuse(); // 2
//     let mut lines_from_stdin = BufReader::new(stdin()).lines().fuse(); // 2
//     loop {
//         select! { // 3
//             line = lines_from_server.next() => match line {
//                 Some(line) => {
//                     let line = line?;
//                     println!("{}", line);
//                 },
//                 None => break,
//             },
//             line = lines_from_stdin.next() => match line {
//                 Some(line) => {
//                     let line = line?;
//                     writer.write_all(line.as_bytes()).await?;
//                     writer.write_all(b"\n").await?;
//                 }
//                 None => break,
//             }
//         }
//     }
//     Ok(())
// }



// pub async fn current(stream: Pin<&mut UnixStream>) -> io::Result<Signal> {
//    io::timeout(Duration::from_millis(500), async {
        // let mut response = Vec::new();
        // stream.read_to_end(&mut response).await?;
        // let sig: Signal = from_slice(response.as_slice())?;
        // Ok(sig)
    // }).await
// }
// pub type SignalStream = Pin<&mut dyn Stream<Item = Result<Signal, io::Error>>>;

// pub async fn signal(sensor: &Sensor) -> io::Result<impl Stream<Item = io::Result<Signal>>> {
    // let path= sensor.path.join("signal");
    // let stream = Pin::new(&mut UnixStream::connect("/tmp/sensor").await?);
    // let mut sig = stream::once(current(stream));
    // Ok(sig)
// }


pub async fn record(sensor: Sensor) -> io::Result<()> {
    // let config :
    //  io::timeout(Duration::from_secs(5), async {
        // stream.read_to_end(&mut buf).await?
        // Ok(buf)
    // })
    // let _path = sensor.mio.directory().join("record");
    // let mut file = File::open(&path).await?;
    // let mut buffer:Vec<u8> = Vec::new();
    // let _n = file.read(&mut buffer).await?;
    // let config: Config = from_bytes(buffer.as_slice())?;
    Ok(())
}

//
// pub async fn read_config(sensor: &Sensor) -> Result<Config> {
    // let path = sensor.node.path().join("config");
    // let mut file = File::open(&path).await?;
    // let mut buffer:Vec<u8> = Vec::new();
    // let _n = file.read(&mut buffer).await?;
    // let config: Config = from_bytes(buffer.as_slice())?;
    // Ok(config)
// }

