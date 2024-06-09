---
bad_links: 
aliases: []
date created: Monday, July 10th 2023, 2:17:38 pm
tags: [coding, networks]
title: ESP32 UDP Communication
---

# UDP Communication

Here is a script for listening to a port and printing any UDP comms that arrive on that port

```C
/*
WiFi [[UDP Communication.md|UDP]] Send and Receive String
This sketch wait an [[UDP Communication.md|UDP]] packet on localPort using a WiFi shield.
When a packet is received an Acknowledge packet is sent to the client on port remotePort
Circuit:
* WiFi shield attached
created 30 December 2012
by dlf (Metodo2 srl)
*/

#include <SPI.h>
#include <WiFi.h>
#include <WiFiUdp.h>
#include "AsyncUDP.h"

int status = WL_IDLE_STATUS;
char ssid[] = "Prime"; //  your network SSID (name)
char password[] = "4f549a297a";    // your network password (use for WPA, or use as key for WEP)
int keyIndex = 0;            // your network key Index number (needed only for WEP)
const int rele = 23;
AsyncUDP [[UDP Communication.md|udp]];
unsigned int localPort = 2390;      // local port to listen on
char packetBuffer[255]; //buffer to hold incoming packet
char  ReplyBuffer[] = "acknowledged";       // a string to send back
WiFiUDP [[UDP Communication.md|Udp]];

void get_network_info(){
	if(WiFi.status() == WL_CONNECTED) {
		Serial.print("[*] Network information for ");
		Serial.println(ssid);
		Serial.println("[+] BSSID : " + WiFi.BSSIDstr());
		Serial.print("[+] Gateway IP : ");
		Serial.println(WiFi.gatewayIP());
		Serial.print("[+] Subnet Mask : ");
		Serial.println(WiFi.subnetMask());
		Serial.println((String)"[+] RSSI : " + WiFi.RSSI() + " dB");
		Serial.print("[+] ESP32 IP : ");
		Serial.println(WiFi.localIP());
	}
}

void setup(){
	Serial.begin(9600);
	delay(3000);
	WiFi.begin(ssid, password);
	Serial.println("\nConnecting");
	while(WiFi.status() != WL_CONNECTED) {
		Serial.print(".");
		delay(100);
	}
	Serial.println("\nConnected to the WiFi network");
	get_network_info();
	if ([[UDP Communication.md|udp]].listen(localPort)) {
		Serial.print("UDP Listening on IP: ");
		Serial.println(WiFi.localIP());
		[[UDP Communication.md|udp]].onPacket([](AsyncUDPPacket packet) {
			Serial.print("UDP Packet Type: ");
			Serial.print(packet.isBroadcast() ? "Broadcast" : packet.isMulticast() ? "Multicast" : "Unicast");
			Serial.print(", From: ");
			Serial.print(packet.remoteIP());
			Serial.print(":");
			Serial.print(packet.remotePort());
			Serial.print(", To: ");
			Serial.print(packet.localIP());
			Serial.print(":");
			Serial.print(packet.localPort());
			Serial.print(", Length: ");
			Serial.print(packet.length()); //dlzka packetu
			Serial.print(", Data: ");
			Serial.write(packet.data(), packet.length());
			Serial.println();
			String myString = (const char*)packet.data();
			if (myString == "ZAP") {
				Serial.println("Zapinam rele");
				digitalWrite(rele, LOW);
			} else if (myString == "VYP") {
				Serial.println("Vypinam rele");
				digitalWrite(rele, HIGH);
			}
		});
	}
}
void loop() {

}
```

Here is how to UDP messages
