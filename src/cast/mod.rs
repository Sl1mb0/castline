extern crate structopt;
use structopt::StructOpt;
use tacklebox::tcp::TcpMetadata;
use tacklebox::udp::UdpMetadata;
use tacklebox::Protocol;

//use std::io::Error as IoErr;
//use std::io::ErrorKind as IoErrKind;

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(help = "Tranport protocol used")]
    pub protocol: Protocol,
    #[structopt(help = "Socket to send datagrams from")]
    pub local: String,
    #[structopt(short = "n", help = "Amount of datagrams to collect")]
    pub amount: Option<u16>,
    #[structopt(short, long, help = "Time to spend waiting for individual datagrams")]
    pub time: Option<u32>,
}

pub fn run(options: &Options) {
    let (_amount, _wait_time) = set_amount_and_time(&options);
    match options.protocol {
        Protocol::Udp => {
            let _udp_metadata = UdpMetadata::new(&options.local[..]);
            todo!()
        }
        Protocol::Tcp => {
            let _tcp_metadata = TcpMetadata::new(&options.local[..]);
            todo!()
        }
    }
}

fn set_amount_and_time(options: &Options) -> (u16, u32) {
    let (mut amount, mut time): (u16, u32) = (5, 5);
    if let Some(a) = options.amount {
        amount = a;
    }
    if let Some(t) = options.time {
        time = t;
    }
    (amount, time)
}
