//! UDP echo server.
//!
//! To send messages do:
//! ```sh
//! $ nc -u localhost 8080
//! ```

// use futures::prelude::*;
// use runtime::net::UdpSocket;
use log::info;
use failure::{Fallible,format_err};
use regex::Regex;
use std::str::FromStr;


use lazy_static::lazy_static;

// lazy_static! {
//     static ref SCALE: f64 = 1.0;
//     static ref EDINBURGH_RE: Regex = {
//         Regex::new(r"N (\d{1}).(\d{4}) 0.0000 0.0000 0.00 0.0000 22942 992.1 0\r\n").unwrap()
//     };
// }


//Example 'N 0.0414 0.0000 0.0000 0.00 0.0000 22942 992.6 0\\r\\nN 0.0414 0.0000 0.0000 0.00 0.0000 22943 992.6 0\\r\\n'
//  2 'N 0.0409 0.0000 0.0000 0.00 0.0000 22936 992.5 0\\r\\n'

// use std::ops::Range;

pub struct Edinburgh {
    buffer     : Vec<u8>,
    updated    : u64,
    scale      : f64,
    fsr        : f64,
    bautrate   : u32,
    ppm        : u32,
    
}


impl Edinburgh {
    pub fn new() -> Edinburgh {
        Edinburgh {
            buffer: Vec::new(),
            updated: 0,
            fsr:    0.0,
            scale: 1.0,
            bautrate: 56700, // entspricht 7 fur montornode 2
            ppm: 500,
        }
    }
    pub fn decode(&mut self, src: &[u8]) -> Fallible<bool> {
        lazy_static! {
            static ref SCALE: f64 = 1.0;
            static ref RE: Regex = {
                Regex::new(r"N (?P<fsr>\d{1}.\d{4}) \d{1}.\d{4} \d{1}.\d{4} \d{1}.\d{2} \d{1}.\d{4} \d{5} \d{3}.\d{1} \d{1}").unwrap()
            };
        }
        self.buffer.extend_from_slice(src);
        let text = std::str::from_utf8(&self.buffer).unwrap();
        if !RE.is_match(text) {
            if self.buffer.len() > 120 {
                self.buffer.clear();
                return Err(format_err!("Edinburg decoder match Regex fail"));
            }
            return Ok(false)
        }
        println!("match: {}",RE.is_match(text));

        match RE.captures(text) {
            Some(caps) => {
                let fsr_str = caps.name("fsr").unwrap().as_str();
                // println!("fsrstr:{}",fsr_str);
                // println!("caps[0]: {}", &caps[0]);
                let a:f64 = f64::from_str(fsr_str)?;
                self.buffer.clear();
                // println!("a:{} caps[1]: {}",a, &caps[1]);
                // println!("caps[2]: {}", &caps[2]);
                self.fsr = a * self.scale;
                Ok(true)
            },
            None => {
                self.buffer.clear();
                Err(format_err!("Edinburg parce captures fail"))
            }
        }
    }
}





// pub fn decode(data:Vec<u8>) -> Fallible<f64> {

// }


// pub async fn run_socket() -> Fallible<()> {
    // let mut socket = UdpSocket::bind("127.0.0.1:31000")?;
    // let mut buf = vec![0u8; 1024];

    // println!("Listening on {}", socket.local_addr()?);
//
    // loop {
        // let (recv, peer) = socket.recv_from(&mut buf).await?;
        // info!("tesive:")
        // let sent = socket.send_to(&buf[..recv], &peer).await?;
        // info!("Sent {} out of {} bytes to {}", sent, recv, peer);
    // }
// }



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode() {
        let data = "N 0.0414 0.0000 0.0000 0.00 0.0000 22942 992.6";
        let mut sen = Edinburgh::new();
        assert_eq!(true,sen.decode(data.as_bytes()).unwrap())
        // let _ = decode(data.into()).unwrap();
    }
}


//

