extern crate etherparse;
use etherparse::UdpHeader;
use std::io;
use std::net::UdpSocket;
use std::time::Instant;

#[derive(Default, Debug, Clone)]
pub struct UdpDatagram {
    pub header: UdpHeader,
    pub data: Vec<u8>,
}

#[derive(Debug)]
struct UdpHandler {
    // Socket to collect from
    socket: UdpSocket,
    // Time spent waiting for individual
    // datagrams to appear
    pub wait_time: u32,
    // Datagrams and the amount of time
    // it took to read them.
    pub datagrams: Vec<(UdpDatagram, u32)>,
}

use io::Error as IoErr;
impl UdpHandler {
    fn new(socket: UdpSocket) -> UdpHandler {
        UdpHandler {
            socket,
            wait_time: 5,
            datagrams: Default::default(),
        }
    }

    fn send<'a>(&self, data: &'a [u8]) -> Result<usize, IoErr> {
        let bytes = self.socket.send(data)?;
        Ok(bytes)
    }

    fn connect<'a>(&self, address: &'a str) -> Result<(), IoErr> {
        self.socket.connect(address)?;
        Ok(())
    }

    #[inline]
    fn collect(&mut self, amount: u16) -> Result<(), IoErr> {
        let buf: &mut [u8] = &mut [0u8; 65536];

        self.socket.set_nonblocking(true).unwrap();
        for _ in 0..amount {
            let now = Instant::now();
            'timed: loop {
                match self.socket.recv(&mut buf[..]) {
                    Ok(bytes) => {
                        let read_time = now.elapsed().as_millis() as u32;
                        let (header, data) = UdpHeader::read_from_slice(&buf)
                            .expect("`UdpHeader::read_from_slice()` failed!");

                        let mut data = data.to_vec();
                        data.resize(bytes - 8, 0);
                        self.datagrams
                            .push((UdpDatagram { header, data }, read_time));
                        break 'timed;
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        if now.elapsed().as_secs() >= self.wait_time as u64 {
                            break 'timed;
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(())
    }
}

pub struct UdpMetadata<'a> {
    local: &'a str,
    remote: Option<&'a str>,
    handler: UdpHandler,
    amount: u16,
    payload: Option<&'a [u8]>,
}

impl<'a> UdpMetadata<'a> {
    pub fn new(socket: &'a str) -> Self {
        let udp_socket = UdpSocket::bind(socket).unwrap();
        UdpMetadata {
            local: &*socket,
            remote: None,
            handler: UdpHandler::new(udp_socket),
            amount: 5,
            payload: None,
        }
    }

    pub fn connect(&mut self, address: &'a str) {
        self.remote = Some(&*address);
        self.handler.connect(address).unwrap();
    }

    pub fn send(&self) {
        if let Some(payload) = self.payload {
            self.handler.send(&payload).unwrap();
        }
    }

    pub fn set_payload(&mut self, data: &'a [u8]) {
        self.payload = Some(&*data)
    }

    pub fn set_amount(&mut self, amount: u16) {
        self.amount = amount;
    }

    pub fn set_time(&mut self, time: u32) {
        self.handler.wait_time = time;
    }

    pub fn collect(&mut self) -> Vec<(UdpDatagram, u32)> {
        self.handler.collect(self.amount).unwrap();
        Vec::from(&self.handler.datagrams[..])
    }
}

#[cfg(test)]
#[test]
fn socket() {
    let rtx = UdpSocket::bind("127.0.0.1:3400").expect("failed to bind to address!");
    let tx = UdpSocket::bind("127.0.0.1:8080").expect("failed to bind to address!");

    rtx.connect("127.0.0.1:8080")
        .expect("connect function failed!");
    tx.connect("127.0.0.1:3400")
        .expect("connect function failed!");

    let dummy_data = [1u8; 569];
    tx.send(&dummy_data).expect("couldn't send message!");
    tx.send(&dummy_data).expect("couldn't send message!");

    let mut udp_handler = UdpHandler::new(rtx);
    udp_handler.collect(2);

    assert!(udp_handler.datagrams.len() == 2);
    assert!(udp_handler.datagrams[0].0.data.len() == 561);
    assert!(udp_handler.datagrams[1].0.data.len() == 561);
}

#[test]
fn timed() {
    let rtx = UdpSocket::bind("127.0.0.1:3445").expect("failed to bind to address!");

    rtx.connect("127.0.0.1:8045")
        .expect("connect function failed!");

    let mut udp_handler = UdpHandler::new(rtx);
    udp_handler.collect(1);

    assert!(udp_handler.datagrams.len() == 0);
}
