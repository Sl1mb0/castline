use thiserror::Error;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Error as IoErr;
use std::str::{self, FromStr};

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

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.to_string()[..])
    }
}

pub trait Sender<'a> {
    fn send(&mut self, data: &'a [u8]) -> Result<usize, IoErr>;
}

pub mod tcp;
pub mod udp;
