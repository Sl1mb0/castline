extern crate structopt;
use structopt::StructOpt;
use thiserror::Error;

pub mod udp;
// pub mod tcp;

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(help = "Tranport protocol used")]
    pub protocol: Protocol,
    #[structopt(help = "Socket to send datagrams from")]
    pub local: String,
    #[structopt(help = "Socket to send datagrams to")]
    pub remote: Option<String>,
    #[structopt(short = "n", help = "Amount of datagrams to collect")]
    pub amount: Option<u16>,
    #[structopt(short, long, help = "Time to spend waiting for individual datagrams")]
    pub time: Option<u32>,
}

#[derive(Error, Debug)]
pub enum ParseErr {
    #[error("Invalid protocol")]
    Protocol,
}

#[derive(Debug)]
pub enum Protocol {
    Udp,
    Tcp,
}

use std::str::{self, FromStr};
impl FromStr for Protocol {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "udp" => Ok(Protocol::Udp),
            "tcp" => Ok(Protocol::Tcp),
            _ => Err(ParseErr::Protocol),
        }
    }
}

use std::fmt;
impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.to_string()[..])
    }
}

pub fn run(options: &Options) {
    match options.protocol {
        Protocol::Udp => {
            let mut udp_metadata = udp::Metadata::new(&options.local[..]);
            set_amount_and_time_udp(options, &mut udp_metadata);
            udp_metadata.collect();
        }
        Protocol::Tcp => {
            println!("tcp")
        }
    }
}

fn set_amount_and_time_udp(options: &Options, udp_metadata: &mut udp::Metadata) {
    if let Some(amount) = options.amount {
        udp_metadata.set_amount(amount);
    }
    if let Some(time) = options.time {
        udp_metadata.set_time(time);
    }
}

fn set_amount_and_time_tcp(options: &Options) {
    todo!()
}
