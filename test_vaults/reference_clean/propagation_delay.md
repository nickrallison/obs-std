---
bad_links: 
aliases: [clock to q]
tags: [electronics]
title: Propagation Delay
date created: Saturday, July 15th 2023, 4:19:02 pm
---
# Propagation Delay

Propagation Delay refers to the time taken for a digital signal to travel from the input of a digital logic gate (like AND, OR, NOT) to its output. This delay is caused by the time it takes for transistors within the gate to switch on or off. It is an important factor in the design and operation of digital circuits, as it can affect the speed and efficiency of data transmission. The propagation delay can vary depending on factors such as temperature, voltage, and load conditions.

It can be calculated for simple circuits like an CMOS Inverter as follows:

1. Identify the input and output nodes of the circuit.

2. Calculate the total capacitance at the output node. This includes both the load capacitance (the capacitance due to other gates connected to the output) and the parasitic capacitance (the inherent capacitance of the devices in the circuit).

3. Calculate the total resistance in the path from input to output when a signal transition occurs. This includes both pull-up and pull-down resistances.

4. The propagation delay, denoted as $t_p$, can be calculated using the formula:

$$   
t_p = ln(2) * R_{total} * C_{total}
$$
   where $R_{total}$ is total resistance and $C_{total}$ is total capacitance.

Note: The 0.69 factor comes from an approximation for charging or discharging a capacitor through a resistor, where it takes approximately 69% of R*C time to reach from 0% to 63% of final value.

This method gives an approximate value for propagation delay, but its important to note that actual propagation delays can vary due to factors such as temperature, supply voltage, manufacturing variations etc. For more accurate results, simulation or measurement is required.
