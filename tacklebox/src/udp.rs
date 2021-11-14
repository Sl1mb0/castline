extern crate etherparse;
use etherparse::UdpHeader;
use std::io;
use std::io::Error as IoErr;
use std::net::UdpSocket;
use std::time::Instant;

pub struct UdpDatagram {
    pub header: UdpHeader,
    pub data: Vec<u8>,
}

pub struct UdpMetadata<'a> {
    pub _addr: &'a str,
    pub socket: UdpSocket,
}

#[allow(dead_code)]
impl<'a> UdpMetadata<'a> {
    pub fn new(local: &'a str) -> Self {
        let socket = UdpSocket::bind(local).unwrap();
        UdpMetadata {
            _addr: &*local,
            socket,
        }
    }

    fn send(&'a self, payload: &'a [u8], amount: u16) -> Result<usize, IoErr> {
        let mut bytes = 0;
        for _ in 0..amount {
            bytes += self.socket.send(payload)?;
        }
        Ok(bytes)
    }

    fn connect_to(&'a self, remote: &'a str) -> Result<(), IoErr> {
        self.socket.connect(remote)?;
        Ok(())
    }

    #[inline]
    pub fn receive(
        &self,
        amount: u16,
        wait_time: u32,
    ) -> Result<Vec<(UdpDatagram, u32)>, IoErr> {
        let mut datagrams: Vec<(UdpDatagram, u32)> = Vec::new();
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

                        datagrams.push((UdpDatagram { header, data }, read_time));
                        break 'timed;
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        if now.elapsed().as_secs() >= wait_time as u64 {
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
