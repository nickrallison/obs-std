---
bad_links: 
aliases: [ipv4, ipv6, Internet Protocol, IP]
tags: [networks]
---
# IP Address

An IP (Internet Protocol) address is a unique numerical identifier assigned to each device connected to a computer network. It allows devices to communicate with each other over the internet or a local network. IP addresses are essential for routing data packets across networks.

There are two versions of IP addresses in use today: IPv4 and IPv6. IPv4 addresses are 32-bit numbers, typically represented in decimal format (e.g., 192.168.0.1). IPv6 addresses, on the other hand, are 128-bit numbers, represented in hexadecimal format (e.g., 2001:0db8:85a3:0000:0000:8a2e:0370:7334).

To understand IP addresses, it's important to grasp the concept of binary and decimal conversions. In the binary number system, there are only two digits: 0 and 1. In the decimal number system, we have ten digits: 0 to 9. 

To convert a binary number to decimal, we use the following formula:

$$
\text{{Decimal}} = \sum_{i=0}^{n-1} \text{{bit}}_i \times 2^{n-1-i}
$$

Where $\text{{bit}}_i$ represents the $i$th bit of the binary number, and $n$ is the total number of bits.

Conversely, to convert a decimal number to binary, we use the following formula:

$$
\text{{Binary}} = \text{{bit}}_n \text{{bit}}_{n-1} \ldots \text{{bit}}_1 \text{{bit}}_0
$$

Where $\text{{bit}}_i$ represents the $i$th bit of the binary number.

IP addresses are divided into two parts: the network portion and the host portion. The network portion identifies the network to which the device belongs, while the host portion identifies the specific device within that network.

In IPv4, the network portion is determined by the subnet mask, which is a 32-bit number that specifies the number of bits used for the network portion. The host portion is determined by the remaining bits. For example, in the IP address 192.168.0.1 with a subnet mask of 255.255.255.0, the first 24 bits (3 bytes) represent the network portion, and the last 8 bits (1 byte) represent the host portion.

In IPv6, the network portion is determined by the prefix length, which specifies the number of bits used for the network portion. The host portion is determined by the remaining bits. For example, in the IP address 2001:0db8:85a3:0000:0000:8a2e:0370:7334 with a prefix length of 64, the first 64 bits represent the network portion, and the last 64 bits represent the host portion.

IP addresses are assigned by Internet Service Providers (ISPs) or network administrators. They are allocated in blocks to ensure efficient use of address space. The Internet Assigned Numbers Authority (IANA) is responsible for the global coordination of IP address allocation.

In conclusion, an IP address is a unique numerical identifier assigned to devices on a computer network. It is divided into a network portion and a host portion. IPv4 addresses are 32-bit numbers, while IPv6 addresses are 128-bit numbers. Binary and decimal conversions are used to represent and manipulate IP addresses.

> For more information on IP addresses, you can refer to the following resources:
> - [IP Address - Wikipedia](https://en.wikipedia.org/wiki/IP_address)
> - [IPv4 Addressing and Subnetting - Cisco](https://www.cisco.com/c/en/us/support/docs/ip/routing-information-protocol-rip/13788-3.html)
> - [IPv6 Addressing - Cisco](https://www.cisco.com/c/en/us/support/docs/ip/ip-version-6-ipv6/13740-29.html)