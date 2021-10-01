Castline
=======

`castline` is my personal spin on `nc` in Rust. It's
called `castline` because `netcrab` sounded dumb, and sending packets
to your mom's house is like catch and release fishing.

(Future employers should acknowledge that there is a substantial
amount of cheek in the original `nc` tarball, and the above is my attempt
at maintaining it. Yes I know it's in poor taste; and no I won't make these jokes
if you hire me. Not unless explicit comic relief is requested)

I was inspired to write this project after reading more about `nc`
on Gordon Lyon's blog.  The original `nc` was released in 1995;
and that has been the only major release since. I downloaded the source
from there and that is the only major source of reference for this project.

`castline` is supported on Linux and MacOS. Since Microsoft has somewhat
improved I might and get around to adding it for Windows. Though I'm not 
sure Hobbit would approve.

## Major Features

Original `netcat` features included in `castline`:

- Outbound or inbound TCP/UDP connections; to or from any port
- Full DNS reverse/forward checking
- Ability to use any local source port
- Ability to use any locally-configured network source address
- Built-in port-scanning capabilities, with randomizer
- Built-in loose source-routing capability
- Can read command line arguments from standard input
- Slow-send mode, one line every N seconds
- Hex dump of transmitted and received data

Additional features of `castline`:

- Provides information about packet loss in relation to packet size/amount.
  - Must know how many packets or "requests" are being 
    responded to successfully; how many aren't
  - runs three different tests with three different quantities each of size/amount
    and informs user of percentage of packets not responded to
  - allow user to specify custom packet size/amount; will output total and
    percent of packets lost


My goal is to add more features to `castline`, but I'm a network n00b 
and thus not very familiar with the current landscape of hackery-networky
tools and what they are lacking. I am happy to recieve any 
general feedback, ideas, etc.

## Options & Usage

```
-n: castline will only accept numeric addresses
-v: verbose output; unless specified castline will only output information received from 
-w: limits the amount of time spent establishing a connection
-u: use UDP instead of TCP
-o: given a name as an argument, `castline` will produce a file with that name containing a hexdump 
    of the traffic between hosts. `<` indicates from server `>` indicates to server.
target: given a M-N range, `castline` will try connecting to every port within the range.
--feeding-frenzy | -f: will attempt to "stress" the host; and produce information about packet size/amount
          and the amount of packet loss that occurs for 3 different ratios. packet amount and size can be 
          specified when `amount:size` is passed as an arg; will run one test for each arg passed
          and provide information about packet loss
```

## Installation
