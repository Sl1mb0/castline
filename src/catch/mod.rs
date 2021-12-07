extern crate structopt;
use structopt::StructOpt;
use tacklebox::tcp::{TcpDatagram, TcpSession};
use tacklebox::udp::{UdpDatagram, UdpSession};
use tacklebox::Protocol;

use std::io::ErrorKind as IoErrKind;
use std::time::Instant;

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

pub fn run(options: &mut Options) {
    let amount = *options.amount.get_or_insert(5);
    let wait_time = *options.time.get_or_insert(5);

    match options.protocol {
        Protocol::Udp => {
            let session = UdpSession::new(&options.local[..]);
            let mut packets: Vec<UdpDatagram> = Vec::new();

            let mut total_bytes = 0;
            let now = Instant::now();
            for _ in 0..amount {
                match session.receive(wait_time) {
                    Ok((packet, bytes)) => {
                        packets.push(packet);
                        total_bytes += bytes;
                    }
                    Err(ref e) if e.kind() != IoErrKind::TimedOut => {
                        println!("Unexpected Error : {}\n", *e);
                        return;
                    }
                    Err(_) => {}
                }
            }
            let total_time = now.elapsed().as_secs();
            let received = packets.len() as f32 / amount as f32;

            let avg_time = total_time as f32 / received;

            print_header();
            print_stats(avg_time, total_bytes, received);
        }
        Protocol::Tcp => {
            let mut session = TcpSession::new(&options.local[..]);
            let mut packets: Vec<TcpDatagram> = Vec::new();

            match session.wait_for_connection(wait_time) {
                Ok(s) => println!("TCP connection esstablished in : {} seconds", s),
                Err(e) => {
                    println!("\nTCP connection not established : {}\n", e);
                    return;
                }
            };

            let mut total_bytes = 0;
            let now = Instant::now();
            for _ in 0..amount {
                let (packet, bytes) = session.receive(wait_time).unwrap();
                packets.push(packet);
                total_bytes += bytes;
            }
            let read_time = now.elapsed().as_secs() as u32;
            let received = packets.len() as f32 / amount as f32;
            let avg_time = read_time as f32 / received;

            print_header();
            print_stats(avg_time, total_bytes, received);
        }
    }
}

fn print_header() {
    println!();

    println!(
        "{time:<width$}{bytes:<width$}{received:<width$}",
        time = "AVG TIME",
        bytes = "TOTAL BYTES",
        received = "PACKETS RECEIVED",
        width = 20
    );
}

fn print_stats(avg_time: f32, total_bytes: u32, received: f32) {
    let mut received = received.to_string();
    received.push_str("%");

    let mut avg_time = avg_time.to_string();
    avg_time.push_str("s");

    println!(
        "{time:<width$}{bytes:<width$}{received:<width$}",
        time = &avg_time[..],
        bytes = total_bytes,
        received = received,
        width = 20
    );

    println!();
}
