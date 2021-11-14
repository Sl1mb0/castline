use thiserror::Error;

use std::io::Error as IoErr;
use std::fmt;
use std::str::{self, FromStr};
use std::io::Write;

pub mod tcp;
pub mod udp;

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

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.to_string()[..])
    }
}

trait Sender<'a> {
    fn send(&mut self, data: &'a [u8], amount: u16) -> Result<usize, IoErr>;
}

impl<'a> Sender<'a> for tcp::TcpMetadata<'a> {
    fn send(&mut self, data: &'a [u8], amount: u16) -> Result<usize, IoErr> {
        let mut bytes: usize = 0;
        for _ in 0..amount {
            if let Some(_socket) = &self.socket {
                bytes += self.socket.as_ref().unwrap().write(data)?;
            }
        }
        Ok(bytes)
    }
}

impl<'a> Sender<'a> for udp::UdpMetadata<'a> {
    fn send(&mut self, payload: &'a [u8], amount: u16) -> Result<usize, IoErr> {
        let mut bytes = 0;
        for _ in 0..amount {
            bytes += self.socket.send(payload)?;
        }
        Ok(bytes)
    }
}
