Guide to `castline`
==================

These next few sections provide examples of how to 
use `castline`, and what each subcommand is capable of.


#### `catch`

Time and memory metrics for inbound datagrams on the specified port.

Use `-n` to specify the _amount_ of datarams to collect at a time;
use `--time` (or `-t`) to specify the _duration_ to spend waiting
for individual datagrams to appear. If neither flags are used the
default amount is 5 datagrams for a duration of 5 seconds each.

As an example, the following command will attempt to read 5 datagrams
at a time; and wait 30 seconds for each datagram to appear in
the socket's buffer. I.e. if the amount of time it takes for each
datagram to appear in the buffer is at most 30 seconds each, then the
amount of time spent collecting the 5 datagrams would be 5x30 = 150
total seconds.

```
$ castline catch udp 127.0.0.1:3400 -n 5 --time 30

time    bytes   received
========================
30s     627     40%
```


##### Options

| Flag | Description |
|------|-------------|
| `-n`   |  Specify the amount of datagrams to be collected|
|`--time`  | Specify duration to wait for datagrams to appear |



#### `cast`

Send some dummy packets somewhere and see what happens. Use the `-n`
flag to specify the amount of packets you want to send. Can also specify
amount time to wait for responses to appear by using the `--time` flag.

Responses will print to `stdout` as long as they continue to come.

```
$ castline cast tcp 127.0.0.1:3000 "GET http://www.w3.org/pub/WWW/TheProject.html HTTP/1.1"
```


##### Options

| Flag | Description |
|------|-------------|
| `-n`   |  Specify the amount of datagrams to be collected|
|`--time`  | Specify duration to wait for datagrams to appear |



#### `trap`

Generates [AMOUNT] dummy TCP packets, each with [SIZE] amount of data. Sends the packets
to [HOST] from a random port and records how many packets received an ackowledgement;
displays percentage of packets that were _not_ acknowledged. if given a range of addresses
and ports, will send from ports in parallel.


#### `fish`

Inbound/outbound information about a port;
DNS resolver for addresses passed as args:

- what process used it
- what time data was sent/received
- where data went (inbound or outbound; address)
- size of packet read
- if given a range, will randomize ports that are scanned
- DNS resolution warnings and shite
