
#![doc(html_root_url = "https://docs.rs/heim-sensors/0.0.3")]

pub mod setting;
pub mod signal;
pub mod temperatures;
pub mod pressure;
pub mod airflow;
pub mod ndir;


// mod simulation;

// use self::simulation as sys;
// pub enum SendorAddr {
    // Net(SocketAddr),
    // Unix(PathBuf),
// }

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// pub struct SensorAddr {
    // pub address: String,
    // url: String
// }


