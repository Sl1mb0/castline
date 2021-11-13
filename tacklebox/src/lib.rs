use thiserror::Error;
use std::fmt;
use std::str::{self, FromStr};

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
