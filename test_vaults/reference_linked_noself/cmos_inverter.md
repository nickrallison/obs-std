---
bad_links: 
aliases: [Inverter]
tags: [electronics]
title: CMOS Inverter
date created: Saturday, July 15th 2023, 4:15:30 pm
---
# [[MOSFET Transistor|CMOS]] Inverter

A [[MOSFET Transistor|CMOS]] (Complementary Metal-Oxide-Semiconductor) inverter is a fundamental building block of digital circuitry. It consists of a [[MOSFET Transistor|PMOS]] (P-channel [[MOSFET Transistor|MOSFET Transistor]]) and an [[MOSFET Transistor|NMOS]] (N-channel [[MOSFET Transistor|MOSFET Transistor]]) connected in series. The input to the inverter is connected to the gates of both [[MOSFET Transistor|MOSFETs]], and the output is taken from the common node between the two [[MOSFET Transistor|MOSFETs]].

When the input to the inverter is low (logic 0), the [[MOSFET Transistor|PMOS]] is turned on, and the [[MOSFET Transistor|NMOS]] is turned off. In this state, the [[MOSFET Transistor|PMOS]] creates a low-resistance path between the supply [[Voltage|voltage]] (VDD) and the output, pulling it up to VDD. As a result, the output of the inverter is high (logic 1).

Conversely, when the input to the inverter is high (logic 1), the [[MOSFET Transistor|PMOS]] is turned off, and the [[MOSFET Transistor|NMOS]] is turned on. In this state, the [[MOSFET Transistor|NMOS]] creates a low-resistance path between the output and the ground (GND), pulling it down to GND. As a result, the output of the inverter is low (logic 0).

The [[MOSFET Transistor|CMOS]] inverter operates in a complementary manner, where one [[MOSFET Transistor|MOSFET Transistor]] is on while the other is off, depending on the input. This allows for efficient switching between logic levels and minimizes power consumption. Additionally, the use of both [[MOSFET Transistor|PMOS]] and [[MOSFET Transistor|NMOS]] transistors ensures that the inverter can handle both high and low logic levels effectively.

The [[MOSFET Transistor|CMOS]] inverter is a fundamental component in digital circuit design and is used to construct various logic gates and flip-flops. By combining multiple inverters and other digital circuit elements, complex digital systems can be built.

For more in-depth information, you can refer to the following resources:
- [CMOS Inverter - Wikipedia](https://en.wikipedia.org/wiki/CMOS_inverter)
- [Introduction to CMOS Inverters - All About Circuits](https://www.allaboutcircuits.com/technical-articles/introduction-to-cmos-inverters/)
- [CMOS Inverter: Working, Circuit Diagram, and Its Applications - Electronics Hub](https://www.electronicshub.org/cmos-inverter/)