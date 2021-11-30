extern crate structopt;
use rand::Rng;
use structopt::StructOpt;
use tacklebox::tcp::TcpSession;
use tacklebox::udp::UdpSession;
use tacklebox::Protocol;
use tacklebox::Sender;

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(help = "Tranport protocol used")]
    pub protocol: Protocol,
    #[structopt(short = "f", long = "from", help = "Socket to send datagrams from")]
    pub local: Option<String>,
    #[structopt(help = "Socket to send datagrams to")]
    pub remote: String,
    #[structopt(help = "Payload to send to remote host")]
    pub payload: String,
    #[structopt(short = "n", help = "Amount of datagrams to collect")]
    pub amount: Option<u16>,
    #[structopt(short, long, help = "Time to spend waiting for individual datagrams")]
    pub time: Option<u32>,
}

pub fn run(options: &mut Options) {
    let amount = *options.amount.get_or_insert(5);
    let wait_time = *options.time.get_or_insert(5);

    let local_writer = match &options.local {
        Some(local) => local.clone(),
        None => random_local(),
    };

    match options.protocol {
        Protocol::Udp => {
            let mut writer = UdpSession::new(&local_writer[..]);
            writer.set_remote(&options.remote[..]);

            let mut local_reader = String::from("127.0.0.1:");
            local_reader.push_str(&writer.remote_port.unwrap().to_string()[..]);
            let reader = UdpSession::new(&local_reader[..]);

            for _ in 0..amount {
                writer
                    .send(options.payload.as_bytes())
                    .expect("send failed!");
                let (packet, _bytes) = reader.receive(wait_time).expect("receive failed!");
                let packet_data = String::from_utf8_lossy(&packet.data);
                println!("{}", packet_data);
            }
        }
        Protocol::Tcp => {
            let mut writer = TcpSession::new(&local_writer[..]);
            let connection = writer.connect_to(&options.remote[..]);
            if connection.is_err() {
                let connect_err = connection.unwrap_err();
                println!("\nConnection not established : {}\n", connect_err);
                return;
            }

            let mut local_reader = String::from("127.0.0.1:");
            local_reader.push_str(&writer.remote_port.unwrap().to_string()[..]);
            let mut reader = TcpSession::new(&local_reader[..]);

            for _ in 0..amount {
                writer
                    .send(options.payload.as_bytes())
                    .expect("send failed!");
                let (packet, _bytes) = reader.receive(wait_time).expect("receive failed!");
                let packet_data = String::from_utf8_lossy(&packet.data);
                println!("{}", packet_data);
            }
        }
    }
}

fn random_local() -> String {
    let mut local = String::from("127.0.0.1:");
    let mut rng = rand::thread_rng();
    let port: u16 = rng.gen_range(2000, 65353);
    local.push_str(&port.to_string()[..]);
    local
}
