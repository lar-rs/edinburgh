//! Edinburgh sensor

#[macro_use]
extern crate lazy_static;
use std::fs;
// use std::fs::File;
// use std::io::LineWriter;
use std::io;
use std::io::prelude::*;
use std::path::{PathBuf,Path};
// use serde::{Serialize,Deserialize};
// use std::process;
// use std::stream;
// use std::prelude::*;
use std::time;
// use can::error::CanError;
// use regex::{Regex,RegexSetBuilder};
// use lazy_static::lazy_static;
// use std::time::Duration;
// use std::str::FromStr;
// use std::time::SystemTime;
// use async_std::println;
use edinburgh::{
    cli::*,
    driver::*,
    // config::Config,
};

// use atty::Stream;
// use std::time::Duration;
use crossbeam::channel::{bounded, tick, Receiver, select};

use structopt::StructOpt;

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}



///Edinburgh sensor command argument
#[derive(Debug, StructOpt)]
#[structopt(name = "ndir", about = "🧰edinburgh sensor interface interface usage.")]
pub struct Args {
    ///🗁 sensor working directory
    #[structopt(short = "d", long = "dir",  default_value = ".")]
    dir: PathBuf,
    ///🔌 hardware connection address
    #[structopt(short = "a", long = "address",  default_value = "tcp:host=192.168.66.59,port=6666")]
    address: String,
    ///📬 node address
    #[structopt(short = "n", long = "node",  default_value = "/com/lar/nodes/Doppelmotor1")]
    node: String,
    ///📍 uart select [0..1]
    #[structopt(short = "u", long = "uart",  default_value = "1")]
    uart: u8,
    //⏱ interval in seconds
    #[structopt(short = "i", long = "interval",  default_value = "500")]
    interval: u64,
}

/// 🔧 Activate debug mode
impl Args {
    /// Access the directory name.
    #[inline]
    pub fn directory(&self) -> &Path {
        &self.dir
    }
}

#[paw::main]
fn main(args: Args) -> io::Result<()> {
    // println!("{}",Paint::blue(can::banner::NAME));
    femme::start(log::LevelFilter::Trace).unwrap();
    // let node = fs::read_to_string(args.directory().join("node"))?;
    // let uart =  fs::read_to_string(args.directory().join("uart"))?.parse::<u8>().unwrap_or(1);

    let c = can::io::connection(&args.address)?;
    let uart = can::io::Uart::new(&c,args.node.clone(),"com.lar.nodes.Doppelmotor3".to_owned(),args.uart);
    uart.set_bitrate(57600).expect("Set bautrate error ");
    println!("was los");
    // let mut counter = fs::read_to_string(args.directory().join("counter"))?.parse::<usize>().unwrap_or(20);
    // match args.driver() {
        // Driver::DBus{address,node,uart} => {
            // uart
        // }
    // };
   
    let mut drv = Driver::new(uart);
    let ctrl_c_events = ctrl_channel().expect("create ctrl c signal failed");
    // let driver = mio::driver::start(args.directory())?;
    // let config: Config = confy::load("edinburg")?;
    // println!("{:#?}", config);
    let interval = time::Duration::from_millis(args.interval);
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = stdout.lock(); // acquire a lock on it
    let ticks = tick(interval);

    loop {
        select! {
            recv(ticks) -> _ => {
               let meas=  drv.measure().unwrap();
                writeln!(handle, "{}", meas)?; // add `?` if you care about errors here
            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Abort!");
                break;
            }
        }
    }

    // femme::start(log::LevelFilter::Trace).unwrap();
    // let pid = args.pid()?;
    // ctrlc::set_handler(move || {
    //     if pid.is_file(){
    //         log::info!("pid file removed");
    //         fs::remove_file(&pid).expect("error - remove pid file");
    //     }
    //     process::abort();
    // }).expect("Error setting Ctrl-C handler");
    // 
    //     Cmd::Average(opt) => {
    //         let pid = args.pid()?;
    //         if pid.is_file() {
    //             log::warn!("sensor is busy pid {}",fs::read_to_string(args.pid()?)?);
    //             return Ok(())
    //         }
    //         fs::write(&pid,&format!("{}",process::id()))?;
    //         if let Err(e) = average(&args.directory(),){
    //             eprintln!("{}", e)
    //         }
    //         fs::remove_file(&pid)?;
    //         // can::io::average_signal().await?;
    //     },
    //     Cmd::Setup(setup) => {
    //         setup.save(&args.directory())?;
    //         //
    //     },
    //     Cmd::Clean => {
    //         let pid =args.pid()?;
    //         if pid.is_file(){
    //             log::info!("remove pid file");
    //             let pstr = fs::read_to_string(&pid)?;
    //             fs::remove_file(&pid)?;
    //             process::Command::new("kill") .arg("-9").arg(pstr.as_str()).spawn().expect("kill edinburg process error");
    //         }
    //     }
    // }

    Ok(())
}


