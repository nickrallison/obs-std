---
bad_links: 
aliases: [nmap]
tags: [linux, networks]
---
# Nmap Command
**Expert**: Network Security Analyst  
**Objective**: To provide a comprehensive explanation of the `nmap` command, including its usage, related concepts, and any relevant mathematical formulas or proofs.  
**Assumptions**: You are familiar with basic networking concepts and Unix-like command line interfaces. 

`nmap` (Network Mapper) is a free and open-source utility for network discovery and security auditing. It was designed to rapidly scan large networks, but it also works fine against single hosts. `nmap` uses raw IP packets in novel ways to determine what hosts are available on the network, what services (application name and version) those hosts are offering, what operating systems (and OS versions) they are running, what type of packet filters/firewalls are in use, and dozens of other characteristics.

The basic syntax of `nmap` is as follows:

```
nmap [Scan Type] [Options] {target specification}
```

Here are some common scan types and options:

- `-sS` (TCP SYN scan): This is the default and most popular scan option. It can be performed quickly, scanning thousands of ports per second on a fast network not hampered by restrictive firewalls. It's also relatively unobtrusive and stealthy since it never completes TCP connections.

- `-sU` (UDP scan): This scan can be used to find open UDP ports. It's slower and requires more packets than TCP scanning.

- `-sV` (Version detection): After `nmap` has found some open ports, you can use this option to determine what application is running on those ports, and sometimes even what version of that application.

- `-O` (OS detection): This option attempts to determine what operating system is running on the target host. It's not always accurate, but when it works, it can provide valuable information for further exploitation or patching.

- `-p` (Port range): This option allows you to specify the range of ports you want to scan. For example, `-p 1-100` would scan ports 1 through 100.

- `-v` (Verbose): This option increases the verbosity of `nmap` output, which can be useful for troubleshooting.

- `-oX` (XML output): This option allows you to save the scan results in an XML file, which can be useful for parsing the results with other tools or scripts.

Here's an example of a command that uses some of these options:

```
nmap -sS -sV -O -p 1-100 -v -oX output.xml 192.168.1.1
```

This command performs a TCP SYN scan (`-sS`), version detection (`-sV`), and OS detection (`-O`) on ports 1 through 100 (`-p 1-100`) of the host at IP address 192.168.1.1. It increases verbosity (`-v`) and saves the results in an XML file named `output.xml` (`-oX output.xml`).

Tangentially related to `nmap` are concepts like TCP/IP, UDP, ICMP, and other networking protocols. Understanding these protocols can help you understand how `nmap` works and how to interpret its output. For example, the TCP three-way handshake (SYN, SYN-ACK, ACK) is fundamental to how the TCP SYN scan works.

As for mathematical formulas, derivations, or proofs, they are not typically directly applicable to the usage of `nmap`. However, understanding the mathematics behind cryptographic algorithms can be useful in the broader field of network security.

> For more information, you can refer to the [official Nmap documentation](https://nmap.org/book/man.html) or the book ["Nmap Network Scanning: The Official Nmap Project Guide to Network Discovery and Security Scanning"](https://nmap.org/book/) by Gordon "Fyodor" Lyon, the creator of `nmap`.