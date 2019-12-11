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
   ///📬 node address
    #[structopt(short = "n", long = "node",  default_value = "/com/lar/nodes/Doppelmotor1")]
    node: String,
    ///📍 uart select [0..1]
    #[structopt(short = "u", long = "uart",  default_value = "1")]
    uart: u8,
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
        fs::write(path.join("node"),self.node.as_bytes())?;
        fs::write(path.join("uart"),format!("{}",self.uart).as_bytes())?;
        fs::write(path.join("bitrate"),format!("{}",self.bitrate).as_bytes())?;
        fs::write(path.join("counter"),format!("{}",self.counter).as_bytes())?;
        fs::write(path.join("interval"),format!("{}",self.interval).as_bytes())?;
        Ok(())
    }
}

/// 📢 subcommands
#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(name = "average", about = "📢 subcommand to calculate average value")]
    Average(Average),
    #[structopt(name = "setup", about = "📢 subcommand to setup sensor uart setting")]
    Setup(Setting),
    #[structopt(name = "clean", about = "📢 subcommand to clean pid")]
    Clean,


}


///Edinburgh sensor command argument
#[derive(Debug, StructOpt)]
#[structopt(name = "ndir", about = "  🧰edinburgh sensor interface interface usage.")]
pub struct Args {
    ///🔌 hardware connection address
    #[structopt(short = "a", long = "address",  default_value = "tcp:host=192.168.66.59,port=6666")]
    address: String,
    ///🗁 sensor working directory
    #[structopt(short = "d", long = "dir",  default_value = ".")]
    dir: PathBuf,
    ///📢 subcommand to run.
    #[structopt(subcommand, about = "📢 subcommand to serve controller or start pipeline directly")]
    cmd:Cmd,
}

/// 🔧 Activate debug mode
impl Args {
    /// Access the directory name.
    #[inline]
    pub fn command(&self) -> &Cmd {
       &self.cmd
    }
    pub fn pid(&self) -> io::Result<PathBuf> {
        Ok(self.dir.join("pid"))
    }
    pub fn address(&self) -> &str {
        &self.address
    }
    pub fn directory(&self) -> &Path {
        &self.dir
    }
}
