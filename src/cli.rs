//! Edinburgh sensor

use std::io;
use std::fs;
// use std::io;
use std::path::{PathBuf,Path};
use structopt::StructOpt;


/// ✇ average signal
#[derive(Debug, StructOpt)]
pub struct Average{
    /// ⏱  interval in seconds
    #[structopt(name = "count", long = "count", default_value = "20")]
    count: usize,
}

/// ✇ integration signal
#[derive(Debug, StructOpt)]
pub struct Integral{
    /// ⏱  interval in seconds
    #[structopt(name = "count", long = "count", default_value = "20")]
    count: usize,
}

/// ✇ average signal
#[derive(Debug, StructOpt)]
pub struct Setting {
   
    //⏱ interval in seconds
    #[structopt(short = "i", long = "interval",  default_value = "500")]
    interval: usize,
    //🔌 uart port bitrate setting
    #[structopt(short = "b", long = "bitrate",  default_value = "57600")]
    bitrate: usize,
    ///🔧 average counter
    #[structopt(short = "c", long = "counter",  default_value = "20")]
    counter: usize,

}

impl Setting {
    pub fn save(&self, path: &Path) -> io::Result<()> {
        if ! path.is_dir(){
            fs::create_dir_all(path)?;
        }
        fs::write(path.join("counter"),format!("{}",self.counter).as_bytes())?;
        fs::write(path.join("interval"),format!("{}",self.interval).as_bytes())?;
        Ok(())
    }
}

///📢 Commands
#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(name = "pipe", about = "📢read sensor")]
    Pipe,
    #[structopt(name = "average", about = "📢subcommand to calculate average value")]
    Average(Average),
    #[structopt(name = "setup", about = "📢subcommand to setup sensor uart setting")]
    Setup(Setting),
}