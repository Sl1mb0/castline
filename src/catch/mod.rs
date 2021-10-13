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
    #[structopt(short, help = "Verbose output")]
    pub verbose: bool,
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

            let now = std::time::Instant::now();
            let datagrams = udp_metadata.collect();
            let time = now.elapsed().as_secs();

            let mut total_bytes = 0;
            for datagram in &datagrams {
                // length of payload + length of header
                total_bytes += datagram.0.data.len() as u32 + 8;
            }

            let received = datagrams.len() as f32 / options.amount.unwrap() as f32;

            if !options.verbose {
                print_total_stats(time, total_bytes, received);
            } else {
                print_datagram_stats(datagrams);
            }
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

fn set_amount_and_time_tcp(_options: &Options) {
    todo!()
}

fn print_total_stats(total_time: u64, total_bytes: u32, received: f32) {
    println!();

    println!(
        "{time:>width$}{bytes:>width$}{received:>width$}",
        time = "time",
        bytes = "bytes",
        received = "received",
        width = 10
    );

    println!(
        "{line: >space_width$}{line:=>header_width$}",
        line = "",
        space_width = 6,
        header_width = 24
    );

    let mut received = received.to_string();
    received.push_str("%");

    let mut total_time = total_time.to_string();
    total_time.push_str("s");

    println!(
        "{time:>width$}{bytes:>width$}{received:>width$}",
        time = &total_time[..],
        bytes = total_bytes,
        received = received,
        width = 10
    );

    println!();
}

fn print_datagram_stats(datagrams: Vec<(udp::UdpDatagram, u32)>) {
    println!();

    println!(
        "{src:<width$}{dst:<width$}{len:<width$}{chksm:<width$}{time:<width$}{bytes:<width$}",
        src = "src",
        dst = "dst",
        len = "len",
        chksm = "chksm",
        time = "time",
        bytes = "bytes",
        width = 12
    );

    println!("{line:=<width$}", line = "", width = 65);

    for datagram in &datagrams {
        print_udp(&datagram);
    }
}

fn print_udp(_datagram: &(udp::UdpDatagram, u32)) {
    todo!()
}
