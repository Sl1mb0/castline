// CASTLINE RELEASE 0.1.0 
/// The implementation of `castline`. A descendant of `netcat`
/// that can do some stress testing. The key added feature of
/// `castline` is the `--feeding-frenzy` option. Which provides 
/// stress-test information about a specified host.
/// 
/// `castline` uses the `tacklebox` crate, which provides an API
/// for the network related subsystems that both Linux and MacOS use.
/// `tacklebox` completely abstracts away system differences, allowing
/// `castline` to just focus on good ol' fishin...

use tacklebox::*;

/// Informatin about the specified host.
#[derive(Debug, Clone, Copy)]
struct Host {
    name: &str,
    addr: &str,
    iaddr: InAddr,
}

impl Host {
    /// Produce a `Host` from a resolved 
    /// numeric or alphabetic IPv4 address.
    fn new() -> Host {
        todo!()
    }
}

/// Information about a port.
struct Port {
    name: &str,
    anum: &str,
    num: u16,
}

impl Port {
    /// Produce a `Port` that corresponds to the actual port 
    /// specified. Can be a numeric or alphabetic value.
    fn new() -> Port {
        todo!()
    }
}

/// Some errors that might happen.
enum HostErr {
    Error0,
    UnknownHost,
    HostNameLookupFail,
    UnknownServerErr,
    NoAddr,
}

fn main() {
    
}
