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
                let _received = self.socket.recv(&mut big_buf[..]).expect("`UdpSocket::recv()` failed!");
                let (header, data) = UdpHeader::read_from_slice(&big_buf)
                    .expect("`UdpHeader::read_from_slice()` failed!");
                let mut data = data.to_vec();
                // `header.length` includes size of header;
                // which is 8 bytes.
                data.resize(header.length as usize-8, 0);
                self.datagrams.push(UdpDatagram{header, data});
            }
    }
}

struct UdpMetadata<'a> {
    ip_address: &'a str,
    port: &'a str,
    datagram_collector: UdpCollector,
}

impl<'a> UdpMetadata<'a> {
    fn new(_ip: &str, _port: &str) -> Self {
        todo!()
    }

    fn collect(&mut self) {
        self.datagram_collector.collect();
    }
}

fn run_catch() {
    todo!()
}

#[cfg(test)]

#[test]
fn random_socket() {
    todo!()
}
