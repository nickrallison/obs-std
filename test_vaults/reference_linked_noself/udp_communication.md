---
bad_links: 
aliases: [UDP, User Datagram Protocol]
tags: [networks]
date created: Friday, July 7th 2023, 12:44:28 pm
title: UDP Communication
---

# UDP Communication

UDP (User Datagram Protocol) is one of the core protocols in the [[IP Address|Internet protocol]] suite. It is a simple, connectionless protocol that works on top of [[IP Address|IP]] ([[IP Address|Internet Protocol]]). Unlike TCP (Transmission Control Protocol), which provides reliable, ordered, and error-checked delivery of a stream of octets between applications running on hosts communicating over an [[IP Address|IP]] network, UDP is an unreliable protocol – it does not guarantee delivery, order of delivery, or a valid-checksum.  
UDP is used primarily for establishing low-latency and loss-tolerating connections between applications on the Internet. It speeds up transmissions by enabling the transfer of data before an agreement is provided by the receiving party. This avoids the overhead of such handshaking process which happens in the TCP.  
In UDP communication, packets are sent from one host to another without any form of acknowledgement. If these packets are lost in transmission or received out-of-order, they are not re-transmitted. This can lead to loss of data integrity but makes UDP suitable for real-time applications like VoIP, streaming media and online gaming where speed is more important than reliability.  
In summary, while TCP communication ensures reliability and order at the expense of speed and overheads due to handshake processes and error checks, UDP communication prioritises speed and efficiency over reliability or order.

# How to Send UDP Messages on Linux
```bash
echo "This is my data" > /dev/udp/192.168.1.29/2390
```
