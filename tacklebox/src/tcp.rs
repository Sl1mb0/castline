extern crate etherparse;
use etherparse::TcpHeader;
use std::io::{Error, ErrorKind, Read};
use std::net::{TcpListener, TcpStream};
use std::net::Ipv4Addr;
use std::time::Instant;
use std::io::Write;
use std::str::FromStr;

use crate::Sender;

use Error as IoErr;
use ErrorKind as IoErrKind;

#[derive(Default, Debug, Clone)]
pub struct TcpDatagram {
    pub header: TcpHeader,
    pub data: Vec<u8>,
}

pub struct TcpSession<'a> {
    pub local_addr: &'a str,
    pub local_ipv4: [u8; 4], 
    pub local_port: u16,
    pub remote_addr: Option<&'a str>,
    pub remote_ipv4: Option<[u8; 4]>,
    pub remote_port: Option<u16>,
    listener: TcpListener,
    socket: Option<TcpStream>,
}

#[allow(dead_code)]
impl<'a> TcpSession<'a> {
    pub fn new(local: &'a str) -> TcpSession {
        let listener = TcpListener::bind(local).unwrap();

        let (ipv4, port) = local.split_once(':').unwrap();
        let local_ipv4 = Ipv4Addr::from_str(&ipv4).unwrap().octets();
        let local_port = u16::from_str(&port).unwrap();

        TcpSession {
            local_addr: &*local,
            local_ipv4,
            local_port,
            remote_addr: None,
            remote_ipv4: None,
            remote_port: None,
            listener,
            socket: None,
        }
    }

    pub fn connect_to(&mut self, remote: &'a str) -> Result<(), IoErr> {
        self.remote_addr = Some(remote);
        let (ipv4, port) = remote.split_once(':').unwrap();
        self.remote_ipv4 = Some(Ipv4Addr::from_str(&ipv4).unwrap().octets());
        self.remote_port = Some(u16::from_str(&port).unwrap());
        self.socket = Some(TcpStream::connect(remote)?);
        Ok(())
    }

    pub fn wait_for_connection(&mut self, wait_time: u32) -> Result<u32, IoErr> {
        let time = Instant::now();
        self.listener.set_nonblocking(true)?;
        while time.elapsed().as_secs() < wait_time.into() {
            match self.listener.accept() {
                Ok((new_socket, _addr)) => {
                    self.socket = Some(new_socket);
                    break;
                }
                Err(ref e) if e.kind() == IoErrKind::WouldBlock => {
                    if time.elapsed().as_secs() >= wait_time.into() {
                        return Err(IoErr::from(IoErrKind::TimedOut));
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok(time.elapsed().as_secs() as u32)
    }

    #[inline]
    pub fn receive(
        &mut self,
        wait_time: u32,
    ) -> Result<(TcpDatagram, u32), IoErr> {
        if self.socket.is_none() {
            return Err(IoErr::from(IoErrKind::NotConnected));
        }

        let buf: &mut [u8] = &mut [0u8; 65536];

        self.socket.as_ref().unwrap().set_nonblocking(true).unwrap();
        let now = Instant::now();
        loop {
            match self.socket.as_ref().unwrap().read(&mut buf[..]) {
                Ok(bytes) => {
                        let read_time = now.elapsed().as_millis() as u32;
                        let (header, data) = TcpHeader::read_from_slice(&buf)
                            .expect("`TcpHeader::read_from_slice()` failed!");

                        let mut data = data.to_vec();
                        data.resize(bytes - 8, 0);

                        let datagram = (TcpDatagram {header, data}, read_time);
                        return Ok(datagram);
                }
                Err(ref e) if e.kind() == IoErrKind::WouldBlock => {
                    if now.elapsed().as_secs() >= wait_time.into() {
                        return Err(IoErr::from(IoErrKind::TimedOut));
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }
}

impl<'a> Sender<'a> for TcpSession<'a> {
    fn send(&mut self, data: &'a [u8]) -> Result<usize, IoErr> {
        let mut bytes: usize = 0;
        if let Some(_socket) = &self.socket {
            bytes += self.socket.as_ref().unwrap().write(data)?;
        }
        Ok(bytes)
    }
}
