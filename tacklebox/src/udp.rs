extern crate etherparse;
use etherparse::UdpHeader;
use etherparse::PacketBuilder;
use std::io::ErrorKind as IoErrKind;
use std::io::Error as IoErr;
use std::net::Ipv4Addr;
use std::net::UdpSocket;
use std::time::Instant;

use crate::Sender;

use std::str::FromStr;

pub struct UdpDatagram {
    pub header: UdpHeader,
    pub data: Vec<u8>,
}

pub struct UdpSession<'a> {
    pub local_addr: &'a str,
    pub local_ipv4: [u8; 4],
    pub local_port: u16,
    pub remote_addr: Option<&'a str>,
    pub remote_ipv4: Option<[u8; 4]>,
    pub remote_port: Option<u16>,
    socket: UdpSocket,
}

impl<'a> UdpSession<'a> {
    pub fn new(local: &'a str) -> Self {
        let socket = UdpSocket::bind(local).expect("bind() failed!");

        let (ipv4, port) = local.split_once(':').unwrap();
        let local_ipv4 = Ipv4Addr::from_str(&ipv4).unwrap().octets();
        let local_port = u16::from_str(&port).unwrap();

        UdpSession {
            local_addr: &*local,
            local_ipv4,
            local_port,
            remote_addr: None,
            remote_ipv4: None,
            remote_port: None,
            socket,
        }
    }

    pub fn set_remote(&mut self, remote: &'a str) {
        self.remote_addr = Some(remote);
        let (ipv4, port) = remote.split_once(':').unwrap();
        self.remote_ipv4 = Some(Ipv4Addr::from_str(&ipv4).unwrap().octets());
        self.remote_port = Some(u16::from_str(&port).unwrap());
    }

    #[inline]
    pub fn receive(&self, wait_time: u32) -> Result<(UdpDatagram, u32), IoErr> {
        let buf: &mut [u8] = &mut [0u8; 65536];

        self.socket.set_nonblocking(true).unwrap();
        let now = Instant::now();
        loop {
                match self.socket.recv(&mut buf[..]) {
                Ok(bytes) => {
                    let read_time = now.elapsed().as_millis() as u32;
                    let (header, data) = UdpHeader::read_from_slice(&buf)
                        .expect("`UdpHeader::read_from_slice()` failed!");

                    let mut data = data.to_vec();
                    data.resize(bytes - 8, 0);

                    let datagram = (UdpDatagram{header, data}, read_time);
                    return Ok(datagram);
                }
                Err(ref e) if e.kind() == IoErrKind::WouldBlock => {
                    if now.elapsed().as_secs() >= wait_time as u64 {
                        return Err(IoErr::from(IoErrKind::TimedOut));
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }
}

impl<'a> Sender<'a> for UdpSession<'a> {
    fn send(&mut self, payload: &'a [u8]) -> Result<usize, IoErr> {
        let builder = PacketBuilder::ipv4(
            self.local_ipv4,
            self.remote_ipv4
                .expect("No destination address specified in `Sender::send()`!"),
            20, //FIXME allow user to specify TTL
        )
        .udp(
            self.local_port,
            self.remote_port
                .expect("No destination port specified in `Sender::send()`!"),
        );

        let mut packet = Vec::<u8>::with_capacity(builder.size(payload.len()));
        builder.write(&mut packet, &payload).unwrap();

        let remote_addr = &self.remote_addr.unwrap();
        let bytes = self.socket.send_to(&packet, remote_addr)?;
        Ok(bytes)
    }
}
