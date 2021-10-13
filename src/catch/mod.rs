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
    pub local_socket: String,
    #[structopt(help = "Socket to send datagrams to")]
    pub remote_socket: Option<String>,
    #[structopt(short = "n", help = "Amount of datagrams to collect")]
    pub amount: Option<u16>,
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

pub fn run(_options: &Options) {
    match _options.protocol {
        Protocol::Udp => {
            let mut udp_metadata = udp::UdpMetadata::new(&_options.local_socket[..]);

            if let Some(amount) = _options.amount {
                udp_metadata.set_amount(amount);
            }
            udp_metadata.collect();
        }
        Protocol::Tcp => {
            println!("tcp")
        }
    }
}
