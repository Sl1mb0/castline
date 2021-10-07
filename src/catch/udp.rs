extern crate etherparse;
use etherparse::UdpHeader;
use std::net::UdpSocket;

#[derive(Default, Debug)]
struct UdpDatagram {
    pub header: UdpHeader,
    pub data: Vec<u8>,
}

#[derive(Debug)]
struct UdpCollector {
    // Socket to collect from
    socket: UdpSocket,
    // Amount of datagrams to collect
    amount: u16,
    pub datagrams: Vec<UdpDatagram>,
}

impl UdpCollector {
    fn new(amount: u16, socket: UdpSocket) -> UdpCollector {
        UdpCollector {
            socket,
            amount,
            datagrams: Default::default(),
        }
    }

    fn collect(&mut self) {
        for _ in 0..self.amount {
            // max size of a udp datagram.
            let big_buf: &mut [u8] = &mut [0u8; 65536];
            let _received = self
                .socket
                .recv(&mut big_buf[..])
                .expect("`UdpSocket::recv()` failed!");
            let (header, data) = UdpHeader::read_from_slice(&big_buf)
                .expect("`UdpHeader::read_from_slice()` failed!");
            let mut data = data.to_vec();
            // `header.length` includes size of header;
            // which is 8 bytes.
            data.resize(header.length as usize - 8, 0);
            self.datagrams.push(UdpDatagram { header, data });
        }
    }
}

struct UdpMetadata<'a> {
    local_address: &'a str,
    remote_address: &'a str,
    datagrams: UdpCollector,
}

impl<'a> UdpMetadata<'a> {
    fn new(local: &str, remote: &str) -> Self {
        todo!()
    }

    fn collect(&mut self) {
        self.datagrams.collect();
    }
}

#[cfg(test)]
#[test]
fn random_socket() {
    let rtx = UdpSocket::bind("127.0.0.1:3400").expect("failed to bind to address!");
    rtx.connect("127.0.0.1:8080")
        .expect("connect function failed!");

    let tx = UdpSocket::bind("127.0.0.1:8080").expect("failed to bind to address!");
    tx.connect("127.0.0.1:3400")
        .expect("connect function failed!");

    let dummy_data = [1u8; 569];
    tx.send(&dummy_data).expect("couldn't send message!");

    let mut udp_collector = UdpCollector::new(1, rtx);
    udp_collector.collect();

    assert!(udp_collector.datagrams.len() == 1);
    // This took some playing around to get, but it
    // holds true over multiple tests, which is certainly interesting.
    // Is this the "minimum" packet size for my mac?
    // TODO ask teacher..
    assert!(udp_collector.datagrams[0].data.len() == 249);
}
