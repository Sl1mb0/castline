/// This module contains everything related to `castline catch`; 
/// a utility for reading packet internals on either UDP or TCP 
/// connections. This module is meant to accomplish some other 
/// things as well:
///
/// 1. Abstract away IPv4/Ipv6 UDP datagram differences.
/// 2. Translate buffers read from either `TcpStream` or `UdpStream` 
///    into sets of datagrams.
/// 3. Control how many datagrams we want to read from a `TcpStream`
///    or `UdpStream`.
///
/// It's important to go over some caveats about datagram internals before
/// diving into this file. Generally, datagrams consist of a header, and 
/// some data. The header provides meta-information that the protocols 
/// use to communicate with another, as well as information about 
/// how to make sense of the data. 
///
/// There are a couple of differences between TCP and UDP headers. While 
/// UDP and TCP datagrams both have source, destination, length, and
/// checksum fields in their headers; because TCP is a "connected" protocol,
/// TCP headers include information used in host2host signals (ACK, FIN, etc),
/// as well as some other meta-information related to these signals.
///
/// On the other hand, while UDP headers only contain 4 fields, they use
/// "pseudo-headers" to compute the checksum. These pseudo-headers are
/// different each of IPv4 and IPv6. This quote from wikipedia describes
/// how to compute the checksum:
///
/// "All 16-bit words (of the pseudo-header) are summed using one's complement 
/// arithmetic. Add the 16-bit values up. On each addition, if a carry out 
/// (17th bit) is produced, swing that 17th carry bit around and add it to the least 
/// significant bit of  the running total. Finally, the sum is then one's complemented to 
/// yield the value of the UDP checksum field."

use std::net;

// Wot r u m8?
enum Version {
	IPv4,
	IPv6,
}

/// While a `UdpHeader` technically does contain
/// a "pseudo-header", the pseudo-header is only
/// necessary for computing the checksum, which
/// is why we don't store it.
struct UdpHeader {
	source_port: u16,
	dest_port:   u16,
	length:      u16,
	checksum:    u16,
}

impl UdpHeader {
	/// Create `UdpHeader` for `version` from `stream`.
	fn new(version: Version, stream: net::UdpStream) -> Self {
		todo!()
	}
}


/// A datagram for either 
/// IPv4 or IPv6 UDP sockets.
struct UdpDatagram {
	/// IPv4 | IPv6
	version: Version,
	/// Used to compute `data`.
	header: UdpHeader,
	/// data contained by datagram.
	data: Vec<u8>,
}

impl UdpDatagram {
	fn new(version: Version, stream: net::UdpStream) -> Self {
		todo!()
	}
}

/// A `UdpCollector` collects
/// `amount` datagrams from a `UdpStream`.
struct UdpCollector {
	pub amount: u16,
	pub datagrams: Vec<UdpDatagram>,
}

impl UdpCollector {
	fn new(amount: u16) -> UdpCollector {
		UdpCollector{amount, default::Default}	
	}
	
	/// Collect `self.amount` datagrams; _does not_ overwrite
	/// `self.datagrams`; adds new datagrams to end instead.
	fn collect(&mut self, version: Version, stream: net::UdpStream) {
		todo!()
	}
}

struct UdpMetadata {
	/// IPv4 || IPv6
	version: Version

	/// Local host info
	local_ip: str,
	local_port: str,

	/// Used to collect datagrams from
	/// buffers read from `socket`.
	datagram_collector: UdpCollector,
	/// The socket we are currently reading from.
	socket: net::UdpSocket,
}

impl UdpMetadata {
	fn new(ip: str, port: str) -> Self {
		todo!()
	}
}

fn run_catch() {
	todo!()
}
