castline
========

`castline` is a performance oriented and easy to use networking
swiss army knife a la `nc`. It's called `castline` because
`netcrab` sounded dumb, and transmitting packets to a host is like
fishing; you're hoping they bite.

I was inspired to write this project after reading more about `nc`
on Gordon Lyon's blog.  The original `nc` was released in 1995;
and the last major release was in 1996. I downloaded the source
from there as a reference for this project.

`castline` is supported on Linux and MacOS; and what I mean by that
is that those are the only two systems I test on. If you are interested
in adding windows support though I am happy to review pull requests.


## Features

`castline` includes several major features from `nc`:

- Outbound or inbound connections; TCP or UDP; from any port
- Full DNS reverse/forward checking; with warnings
- Port-scanning abilities; built-in randomizer
- Loose source-routing capability
- Dummy packet generator

And some original features of it's own:

- General performance metrics:
    - Time
    - Memory
    - Packet loss
- Port usage information:
    - Which process uses what
    - What are they doing
- Simple CLI:
  - Subcommands instead of options (for the most part)
  - Get only what you ask for

The original design idea for `nc` was simply a `cat` like utility for networks.
Given some port on a local machine, display all inbound/outbound information on
that port. Obviously `nc` does a bit more than that; and so does `castline`.
For the most part however, `castline` will only print what you ask for. In the case
where ou don't provide specifics, default values are used. These defaults
are described in the [guide][GUIDE].


## Usage

For details on how to use `castline`, see the [guide][GUIDE].


## Installation


## References

[nc-tarball][NCTAR]  
[RFC793][TCPImpl]
[GUIDE][GUIDE]


[NCTAR]:https://sectools.org/tool/netcat/
[TCPImpl]:https://datatracker.ietf.org/doc/html/rfc793
[GUIDE]:https://github.com/sl1mb0/castline/blob/master/GUIDE.md
