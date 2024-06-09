---
aliases: 
tags:
  - devices
bad_links:
---
# OPNSense Router

## How to Set up and Get Running From Reset

1. Plug in LAN to PC
2. Wizard should auto open
	1. set 8.8.8.8 to DNS
	2. No other setup changes
3. Open Terminal
	1. Should be able to ping 192.168.1.1
4. Unplug and replug Modem to renegotiate WAN DHCP
5. Should see WAN IPV4 in OPNSense Dashboard
	1. This means you can access internet from the router

## How to Safely Reset Router

**System** -> **Configuration** -> **Defaults**

## Problems

Webui becomes unresponsive
- Restart Router then restart PC

Can't connect to WAN on Router or PC
- Reset Modem

Not getting gateway assigned when PC connected to router, can't ping or connect to router
- Try plugging the LAN cable into other ports.
- Sometimes when set to default, the LAN and WAN ports get switched, Try plugging LAN connection into WAN port

Potential DNS rebind Attack
- Change Port of OPNSense Web GUI
- Make sure ports forward settings are done correctly