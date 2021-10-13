Guide to `castline`
==================

These next few sections provide examples of how to 
use `castline`, and what each subcommand is capable of.


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

time    bytes   received
========================
30s     627     40%

src                dst              len         chksm       time    bytes
=========================================================================
127.0.0.1:3400     555.92.654.71    523         9999999    20s    531
-------------------------------------------------------------------------
127.0.0.1:8900     534.70.60.234    90          9999999    10s    98
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