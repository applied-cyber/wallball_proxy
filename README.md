# Wallball Proxy

## Overview
This project aims to implement a simple proxy that uses ICMP error messages to
deliver the desired payload. The primary motivation behind this project is to
send covert messages via an intermediate middlebox. The system sends IP packets
to a middlebox, which responds with an ICMP error message. The proxy chunks
up a standard IP packet into the payload of the IP packet, so that the
generated ICMP message contains the first 8 bytes of the IP packet. This allows
the proxy to "bounce" messages off the middlebox, as an arbitrary 8 bytes are
sent from the sender via the middlebox. The proxy also rewrites the source
address of the IP packet, so that the middlebox sends the ICMP message to a
desired client. This allows us to bounce a stream of messages off of a
middlebox, without the participation of the middlebox while enabling covert
message transmission.

## Architecture Description

### Diagram

                +----------+
                | Client 1 |-----------------+
                +----------+                 |
                                             v
                                       +-----------+
                                       | Middlebox |
                                       +-----------+
                                             |
                +----------+                 |
                | Client 2 |<----------------+
                +----------+

### Description
The proxy requires two corrdinating endpoints; `Client 1` and `Client 2`. The
proxy also requires a noisy middlebox; that is, a firewall or router that
responds with ICMP error messages (Time Exceeded, Destination Unreachable,
or Echo Reply message all work). Note that the middlebox does not need to be
in the path between `Client 1` and `Client 2`, and we denote the middlebox
as `Middlebox`.

### Primary Functionality
The proxy functions by fragmenting and sending normal IP packets into ~8-byte
chunks, that can be placed as a fake UDP header in a new IP packet. This
fake IP packet is then sent to the `Middlebox`, which should then respond with
an ICMP error message. From the definition of the ICMP error message format,
this response will include the first 8 bytes of the IP packet - which is the
UDP header we initially constructed. The final step to the proxy design is to
construct the IP packet with a source IP address corresponding to `Client 2`.

Below, the diagram gives an overview of this process:

    +--------------------+                         +---------+
    | Original IP Packet | ----------------------->|  Proxy  |
    +--------------------+                         +---------+
                                                        |
                                                        | 8-byte chunks
                                                        |
                                                        v
    +----------------------+--------------------+----------------+
    |       IP Header      |     UDP Header     |     Payload    |
    +----------------------+--------------------+----------------+
    | Dst Addr = Middlebox |   8-byte payload   |  Fake payload  |
    | Src Addr = Client 2  | as fake UDP header |                |
    +----------------------+--------------------+----------------+


## Remaining Tasks
TODO

## Contributors
- Paul Crews (ptcrews@cs.stanford.edu)
- TODO
