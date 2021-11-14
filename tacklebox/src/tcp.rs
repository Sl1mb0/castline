extern crate etherparse;
use etherparse::TcpHeader;
use std::io::{Error, ErrorKind, Read};
use std::net::{TcpListener, TcpStream};
use std::time::Instant;

use Error as IoErr;
use ErrorKind as IoErrKind;

#[derive(Default, Debug, Clone)]
pub struct TcpDatagram {
    pub header: TcpHeader,
    pub data: Vec<u8>,
}

pub struct TcpMetadata<'a> {
    _addr: &'a str,
    listener: TcpListener,
    pub socket: Option<TcpStream>,
}

#[allow(dead_code)]
impl<'a> TcpMetadata<'a> {
    pub fn new(local: &'a str) -> TcpMetadata {
        let listener = TcpListener::bind(local).unwrap();
        TcpMetadata {
            _addr: &*local,
            listener,
            socket: None,
        }
    }

    pub fn wait_for_connection(&mut self, wait_time: u32) -> Result<(), IoErr> {
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
        Ok(())
    }

    #[inline]
    pub fn receive(
        &mut self,
        amount: u16,
        wait_time: u32,
    ) -> Result<Vec<(TcpDatagram, u32)>, IoErr> {
        if self.socket.is_none() {
            return Err(IoErr::from(IoErrKind::NotConnected));
        }

        let mut datagrams: Vec<(TcpDatagram, u32)> = Vec::new();
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
                        datagrams.push((TcpDatagram { header, data }, read_time));
                        break 'timed;
                    }
                    Err(ref e) if e.kind() == IoErrKind::WouldBlock => {
                        if now.elapsed().as_secs() >= wait_time.into() {
                            break 'timed;
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(datagrams)
    }
}
