extern crate structopt;
use structopt::StructOpt;
use tacklebox::random_local;
use tacklebox::tcp::TcpSession;
use tacklebox::Sender;

use std::convert::TryInto;

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(short = "f", long = "from", help = "Socket to send datagrams from")]
    pub local: Option<String>,
    #[structopt(help = "Socket to send datagrams to")]
    pub remote: String,
    #[structopt(short = "n", help = "Amount of datagrams to collect")]
    pub amount: Option<u16>,
    #[structopt(short, long, help = "Size of datagrams being sent")]
    pub packet_size: Option<u32>,
    #[structopt(short, long, help = "Time to spend waiting for individual datagrams")]
    pub time: Option<u32>,
}

pub fn run(options: &mut Options) {
    let amount = *options.amount.get_or_insert(5);
    let size = *options.packet_size.get_or_insert(5);
    let wait_time = *options.time.get_or_insert(5);

    let local_writer = match &options.local {
        Some(local) => local.clone(),
        None => random_local(),
    };

    let mut writer = TcpSession::new(&local_writer[..]);
    let connection = writer.connect_to(&options.remote[..]);
    if connection.is_err() {
        let connect_err = connection.unwrap_err();
        println!("\nConnection not established : {}", connect_err);
    }

    let mut local_reader = String::from("127.0.0.1:");
    local_reader.push_str(&writer.remote_port.unwrap().to_string()[..]);
    let mut reader = TcpSession::new(&local_reader[..]);

    let payloads = vec![vec![1; size.try_into().unwrap()]; amount.into()];
    for payload in &payloads {
        writer.send(&payload).expect("send failed!");
    }

    let mut received = 0;
    for _ in 0..amount {
        if reader.receive(wait_time).is_ok() {
            received += 1;
        }
    }

    print_header();
    print_stats(amount, size, received);
}

fn print_header() {
    println!();

    println!(
        "{amount:<width$}{size:<width$}{received:<width$}",
        amount = "AMOUNT SENT",
        size = "PACKET SIZE",
        received = "PACKETS RECEIVED",
        width = 20
    );
}

fn print_stats(amount: u16, size: u32, received: usize) {
    println!(
        "{amount:<width$}{size:<width$}{received:<width$}",
        amount = amount,
        size = size,
        received = received,
        width = 20
    );
}
