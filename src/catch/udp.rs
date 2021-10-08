extern crate etherparse;
use etherparse::UdpHeader;
use std::net::UdpSocket;

#[derive(Default, Debug)]
struct UdpDatagram {
    pub header: UdpHeader,
    pub data: Vec<u8>,
}

#[derive(Debug)]
struct UdpHandler {
    // Socket to collect from
    socket: UdpSocket,
    // Amount of datagrams to collect
    amount: u16,
    pub datagrams: Vec<UdpDatagram>,
}

impl UdpHandler {
    fn new(amount: u16, socket: UdpSocket) -> UdpHandler {
        UdpHandler {
            socket,
            amount,
            datagrams: Default::default(),
        }
    }

    // TODO handle errors
    fn send<'a>(&self, data: &'a [u8]) {
        self.socket.send(data).unwrap();
    }

    // TODO handle errors
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
    handler: UdpHandler,
    // dummy data that's sent to `remote_address`
    // each time `send()` is called; is `None`
    // unless `set_payload()` is called.
    payload: Option<&'a [u8]>,
}

impl<'a> UdpMetadata<'a> {
    // TODO handle errors
    fn new(local: &'a str, remote: &'a str) -> Self {
        let socket = UdpSocket::bind(local).unwrap();
        socket.connect(remote).unwrap();

        UdpMetadata {
            local_address: &*local,
            remote_address: &*remote,
            handler: UdpHandler::new(1, socket),
            payload: None,
        }
    }

    // TODO handle errors
    fn send(&self) {
        if let Some(payload) = self.payload {
            self.handler.send(&payload)
        }
    }

    fn set_payload(&mut self, data: &'a [u8]) {
        self.payload = Some(&*data)
    }

    // TODO handle errors
    fn collect(&mut self) {
        self.handler.collect();
    }
}

#[cfg(test)]
#[test]
fn random_socket() {
    let rtx = UdpSocket::bind("127.0.0.1:3400").expect("failed to bind to address!");
    let tx = UdpSocket::bind("127.0.0.1:8080").expect("failed to bind to address!");

    rtx.connect("127.0.0.1:8080")
        .expect("connect function failed!");
    tx.connect("127.0.0.1:3400")
        .expect("connect function failed!");

    let dummy_data = [1u8; 569];
    tx.send(&dummy_data).expect("couldn't send message!");
    tx.send(&dummy_data).expect("couldn't send message!");

    let mut udp_handler = UdpHandler::new(2, rtx);
    udp_handler.collect();

    assert!(udp_handler.datagrams.len() == 2);
    assert!(udp_handler.datagrams[0].data.len() == 249);
    assert!(udp_handler.datagrams[1].data.len() == 249);
}
