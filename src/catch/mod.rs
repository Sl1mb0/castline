extern crate structopt;
use structopt::StructOpt;
use tacklebox::tcp::TcpMetadata;
use tacklebox::udp::UdpMetadata;
use tacklebox::Protocol;

use std::io::Error as IoErr;
use std::io::ErrorKind as IoErrKind;

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
    let (_amount, wait_time) = set_amount_and_time(&options);
    match options.protocol {
        Protocol::Udp => {
            let udp_metadata = UdpMetadata::new(&options.local[..]);

            let now = std::time::Instant::now();
            let datagrams = udp_metadata.collect_datagrams(_amount, wait_time).unwrap();
            let time = now.elapsed().as_secs();

            let mut total_bytes = 0;
            for datagram in &datagrams {
                total_bytes += datagram.0.data.len() as u32;
            }

            let received = datagrams.len() as f32 / _amount as f32;

            print_total_stats(time, total_bytes, received);
        }
        Protocol::Tcp => {
            let mut tcp_metadata = TcpMetadata::new(&options.local[..]);

            let _block = IoErr::from(IoErrKind::WouldBlock);

            let now = std::time::Instant::now();
            if let Err(_block) = tcp_metadata.wait_for_connection(wait_time) {}
            let time = now.elapsed().as_secs();

            print_total_stats(time, 0, 0.0);
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
