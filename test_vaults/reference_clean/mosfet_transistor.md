---
bad_links: 
aliases: [CMOS, NMOS, PMOS, MOS]
tags: [Electronics]
title: MOSFET
date created: Saturday, July 15th 2023, 4:25:21 pm
---
# MOSFET Transistor

A Metal-Oxide-Semiconductor Field-Effect Transistor (MOSFET) is a type of transistor used for amplifying or switching electronic signals. It is one of the basic building blocks in modern electronic devices. The MOSFET is a four-terminal device with source(S), gate(G), drain(D), and body(B) terminals. The body of the MOSFET is frequently connected to the source terminal so it is often considered as a three-terminal device.

The operation of a MOSFET can be divided into three regions: cut-off, triode, and saturation.

1. **Cut-off region (also known as subthreshold region)**: In this region, the gate-source voltage ($V_{GS}$) is less than the threshold voltage ($V_{T}$), and the MOSFET is off – it does not conduct current from the drain to the source. The condition for this region is $V_{GS} < V_{T}$.

2. **Triode region (also known as linear region)**: In this region, the MOSFET is on, and the current from drain to source ($I_{D}$) is controlled by $V_{GS}$ and $V_{DS}$. The MOSFET behaves like a resistor. The conditions for this region are $V_{GS} > V_{T}$ and $V_{DS} < V_{GS} - V_{T}$. The drain current in the triode region can be calculated by the following equation:

$$
I_{D} = \mu C_{ox} \frac{W}{L} ((V_{GS} - V_{T})V_{DS} - \frac{1}{2}V_{DS}^2)
$$

where $\mu$ is the carrier mobility, $C_{ox}$ is the oxide capacitance per unit area, $W$ is the transistor width, and $L$ is the transistor length.

1. **Saturation region (also known as active region)**: In this region, the MOSFET is on, and the drain current is not a function of $V_{DS}$. It is controlled only by $V_{GS}$. The conditions for this region are $V_{GS} > V_{T}$ and $V_{DS} > V_{GS} - V_{T}$. The drain current in the saturation region can be calculated by the following equation:

$$
I_{D} = \frac{1}{2} \mu C_{ox} \frac{W}{L} (V_{GS} - V_{T})^2
$$

The MOSFET has two types of configurations, N-channel MOSFET, and P-channel MOSFET, based on the type of charge carriers that carry current through the channel. In an N-channel MOSFET, electrons are the majority carriers, whereas, in a P-channel MOSFET, holes are the majority carriers.

> For more in-depth information, you can refer to the following resources:
> - [MOSFET - Wikipedia](https://www.google.com/search?q=MOSFET+Wikipedia)
> - [MOSFETs - All About Circuits](https://www.google.com/search?q=MOSFETs+All+About+Circuits)
> - [MOSFET Device Physics and Operation - IOPscience](https://www.google.com/search?q=MOSFET+Device+Physics+and+Operation+IOPscience)