---
bad_links: 
aliases: []
tags: [electronics, computerarchitecture]
---
# Transistor

A **transistor** is a fundamental building block of modern electronic devices. It is a semiconductor device used to amplify or switch electronic signals and electrical power. The transistor is the key active component in practically all modern electronics.

The transistor was invented in 1947 at Bell Laboratories by John Bardeen, Walter Brattain, and William Shockley. The invention of the transistor revolutionized the field of electronics, and paved the way for smaller and cheaper radios, calculators, and computers, among other things.

Transistors are composed of semiconductor material, usually with at least three layers of two types of material. They have three leads (connections), each of which connects to one of the layers. The layer in the middle is called the base, and the layers on the sides are the emitter and collector for BJTs. They would be called Drain, Source, and Gate for MOSFETs.

There are two types of BJT transistors: **[[bjt_transistor.md|NPN]]** and **[[bjt_transistor.md|PNP]]**, with different circuit symbols. The letters refer to the layers of semiconductor material used to make the transistor. Most transistors used today are [[BJT Transistor|NPN]] because this is the easiest type to make from silicon.

The basic function of a transistor is to control a large amount of current with a small amount of current. This is why they are used as amplifiers or switches.

In terms of operation, a small current entering the base in common-emitter mode controls a larger current between the collector and emitter. The ratio of these two currents (Ic/Ib) is the transistor's DC current gain, often given the symbol β (beta).

The mathematical model of a transistor includes the following equations:

1. Emitter current: $I_E = I_B + I_C$
2. Collector current: $I_C = β \cdot I_B$
3. Emitter current (in terms of β): $I_E = (β + 1) \cdot I_B$

Where:
- $I_E$ is the emitter current
- $I_B$ is the base current
- $I_C$ is the collector current
- β is the current gain

Transistors are also used in digital circuits where they serve as electronic switches that can be either 'on' or 'off', corresponding to the two possible states in a binary system (0 and 1).

> For more in-depth information, you may want to read the following resources:
> - ["Transistor"](https://www.google.com/search?q=Transistor) on Wikipedia
> - ["How do transistors work?"](https://www.google.com/search?q=How+do+transistors+work%3F) on Explain that Stuff
> - ["Transistor as a Switch"](https://www.google.com/search?q=Transistor+as+a+Switch) on Electronics Tutorials
> - ["Transistor as an Amplifier"](https://www.google.com/search?q=Transistor+as+an+Amplifier) on Electronics Tutorials
