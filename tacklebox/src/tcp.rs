extern crate etherparse;
use etherparse::TcpHeader;
use std::io::{Error, ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Instant;

#[derive(Default, Debug, Clone)]
pub struct TcpDatagram {
    pub header: TcpHeader,
    pub data: Vec<u8>,
}

#[derive(Debug)]
struct TcpHandler {
    listener: TcpListener,
    socket: Option<TcpStream>,
    pub wait_time: u32,
    pub datagrams: Vec<(TcpDatagram, u32)>,
}

use Error as IoErr;
use ErrorKind as IoErrKind;
impl TcpHandler {
    fn new(listener: TcpListener) -> TcpHandler {
        TcpHandler {
            listener,
            socket: None,
            wait_time: 5,
            datagrams: Default::default(),
        }
    }

    fn set_wait_time(&mut self, wait_time: u32) {
        self.wait_time = wait_time;
    }

    fn send<'a>(&mut self, data: &'a [u8]) -> Result<usize, IoErr> {
        let mut bytes: usize = 0;
        if let Some(_socket) = &self.socket {
            bytes = self.socket.as_ref().unwrap().write(data)?;
        }
        Ok(bytes)
    }

    fn wait_for_connection(&mut self) -> Result<(), IoErr> {
        let time = Instant::now();
        self.listener.set_nonblocking(true)?;
        while time.elapsed().as_secs() < self.wait_time.into() {
            match self.listener.accept() {
                Ok((new_socket, _addr)) => {
                    self.socket = Some(new_socket);
                    break;
                }
                Err(ref e) if e.kind() == IoErrKind::WouldBlock => {
                    if time.elapsed().as_secs() >= self.wait_time.into() {
                        return Err(IoErr::from(IoErrKind::WouldBlock));
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    #[inline]
    fn collect(&mut self, amount: u16) -> Result<(), IoErr> {
        if self.socket.is_none() {
            return Err(IoErr::from(IoErrKind::NotConnected));
        }

        let buf: &mut [u8] = &mut [0u8; 65536];

        self.socket.as_ref().unwrap().set_nonblocking(true).unwrap();

        for _ in 0..amount {
            let now = Instant::now();
            'timed: loop {
                match self.socket.as_ref().unwrap().read(&mut buf[..]) {
                    Ok(bytes) => {
                        let read_time = now.elapsed().as_millis() as u32;
                        let (header, data) = TcpHeader::read_from_slice(&buf)
                            .expect("`TcpHeader::read_from_slice()` failed!");

                        let mut data = data.to_vec();
                        data.resize(bytes - 8, 0);
                        self.datagrams
                            .push((TcpDatagram { header, data }, read_time));
                        break 'timed;
                    }
                    Err(ref e) if e.kind() == IoErrKind::WouldBlock => {
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
/*
pub struct TcpMetadata<'a> {
    local: &'a str,
    remote: Option<&'a str>,
    handler: TcpHandler,
    amount: u16,
    payload: Option<&'a [u8]>,
}

impl<'a> TcpMetadata<'a> {
    pub fn new(socket: &'a str) -> Self {
        let tcp_socket = TcpStream::connect(socket).unwrap();
        TcpMetadata {
            local: &*socket,
            remote: None,
            handler: TcpHandler::new(tcp_listener),
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
}*/

#[cfg(test)]
#[test]
fn socket() {
    // NOTE
    // Port 80 requires sudo privileges on:
    // - [x] MacOS
    // - [ ] Linux
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    let mut tcp_handler = TcpHandler::new(listener);
    match tcp_handler.wait_for_connection() {
        Ok(_) => println!("connection established!"),
        Err(e) => println!("connection failed with error kind: {:?}!", e),
    }
}
