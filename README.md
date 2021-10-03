castline
=======

`castline` is my personal spin on `nc` in Rust. It's called `castline`
because `netcrab` sounded dumb, and transmitting packets to a host is
like fishing; you're hoping they bite.

I was inspired to write this project after reading more about `nc`
on Gordon Lyon's blog.  The original `nc` was released in 1995;
and the last major release was in 1996. I downloaded the source
from there as a reference for this project.

`castline` is supported on Linux and MacOS. Since Microsoft has somewhat
improved it's reputation in the open-source community I might and get around
to adding support for Windows. Though I'm not sure Hobbit would approve.

## Features

`castline` includes several major features from `nc`:

- Outbound or inbound connections; TCP or UDP; from _any_ port
- Full DNS reverse/forward checking; with warnings
- Port-scanning abilities; built-in randomizer
- Loose source-routing capability

And some original features of it's own:

- Percent of packets acknowledged relative to a specified packet amount and size
- CLI that uses subcommands instead of options
- Display information for inbound/outbound datagrams on specific ports
- Automatic DNS lookups for numeric IP addresses

I think it's important to mention that the original design idea for `nc` was
simply a `cat` like utility for networks. Given some port on a local machine,
display all inbound/outbound information on that port. Obviously `nc` does a
bit more than that; and so does `castline`.

However, I really wanted to focus on providing as much information about a host/port
as possible, which is why `castline` can tell you about packet loss relative to a specified amount and size;
as well as display information about inbound/outbound datagrams for a given port.

## References

[nc-tarball][NCTAR]
[RFC793][TCPImpl]



[NCTAR]:https://sectools.org/tool/netcat/
[TCPImpl]:https://datatracker.ietf.org/doc/html/rfc793
