extern crate structopt;
use structopt::StructOpt;
use tacklebox::tcp::TcpMetadata;
use tacklebox::udp::UdpMetadata;
use tacklebox::Protocol;

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
    let _amount = *options.amount.get_or_insert(5);
    let _wait_time = *options.time.get_or_insert(5);

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
