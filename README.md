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
improved it's reputation in the open-source community I might get around
to adding support for Windows. Though I'm not sure Hobbit would approve.
The fact that Windows isn't _really_ POSIX compliant makes this a lot harder.

## Features

`castline` includes several major features from `nc`:

- Outbound or inbound connections; TCP or UDP; from _almost_  any port
  - I hope to change this
- Full DNS reverse/forward checking; with warnings
- Port-scanning abilities; built-in randomizer
- Loose source-routing capability

And some original features of it's own:

- Percent of packets acknowledged relative to a specified packet amount and size
- CLI that uses subcommands instead of options
- Display information for inbound/outbound datagrams on specific ports
- Automatic DNS lookups for numeric IP addresses

The original design idea for `nc` was simply a `cat` like utility for networks.
Given some port on a local machine, display all inbound/outbound information on
that port. Obviously `nc` does a bit more than that; and so does `castline`.
For the most part however, `castline` will output to stdout until either the
connection drops, stops responding, or ^C is called.

I really wanted to focus on providing as much information about a host/port
as possible, which is why `castline` can tell you about packet loss relative
to a specified amount and size; as well as display information about
inbound/outbound datagrams for a given port. This is also why there is automatic
DNS lookup for numeric IP addresses, so we can give the user as much information
as possible.

## Usage

#### `castline catch [PROTOCOL] [LOCALHOST] [FLAGS]`

Display internal information of inbound datagrams and time spent
waiting for them to appear in the `[LOCALHOST]` buffer.

Use `-n` to specify the _amount_ of datarams to collect at a time;
use `--time` (or `-t`) to specify the _duration_ to spend waiting
for individual datagrams to appear.

If neither flags are used the default amount is 10 datagrams for a duration of 6 seconds each.

As an example, the following command will attempt to read 5 datagrams
at a time; and wait 30 seconds for each datagram to appear in
the socket's buffer. I.e. if the amount of time it takes for each datagram
to appear in the buffer is 30 seconds each, then the amount of time spent
collecting the 5 datagrams would be 5x30 = 150 total seconds.

```
$ castline catch udp 127.0.0.1:3400 -n 5 --time 30
```

Datagram internals and the time spent waiting for them will then
"pretty print" to the console.

##### Options

| Flag | Description |
|------|-------------|
| `-n`   |  Specifiy an amount of datagrams to collect |
|`--time`  | Specify how long to spend waiting for each datagram to appear |



#### `castline cast [PROTOCOL] [LOCALHOST] [REMOTEHOST] [DATA]`

Send `[DATA]` to `[REMOTEHOST]` from `[LOCALHOST]` using the
specified `[PROTOCOL]`.

#### `castline trap [HOST] [AMOUNT]:[SIZE]`

Generates [AMOUNT] dummy TCP packets, each with [SIZE] amount of data. Sends the packets
to [HOST] from a random port and records how many packets received an ackowledgement;
displays percentage of packets that were _not_ acknowledged. if given a range of addresses
and ports, will send from ports in parallel.

#### `castline fish [LOCALHOST] [FLAGS] [REMOTEHOST]`

Inbound/outbound information about a port;
DNS resolver for addresses passed as args:

- what process used it
- what time data was sent/received
- where data went (inbound or outbound; address)
- size of packet read
- if given a range, will randomize ports that are scanned
- DNS resolution warnings and shite

## References

[nc-tarball][NCTAR]  
[RFC793][TCPImpl]


[NCTAR]:https://sectools.org/tool/netcat/
[TCPImpl]:https://datatracker.ietf.org/doc/html/rfc793
