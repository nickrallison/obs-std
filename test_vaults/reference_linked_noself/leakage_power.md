---
bad_links: 
aliases: [Static Power]
tags: [computerarchitecture]
---
# Leakage Power

Leakage power is a significant component of the total power consumption in digital circuits, especially as technology scales down into the nanometer regime. It refers to the power consumed by a digital circuit when it is in a non-switching state, i.e., when it is idle or in standby mode. 

There are two primary types of leakage currents in CMOS technology: subthreshold leakage and gate leakage. 

1. **Subthreshold Leakage**: This is the current that flows between the source and the drain of a MOSFET when the gate voltage is below the threshold voltage. The subthreshold leakage current, $I_{sub}$, can be approximated by the following equation:

$$
I_{sub} = I_{0} \times e^{(V_{gs}-V_{th})/nV_{T}}
$$

where $I_{0}$ is the leakage current at the threshold voltage, $V_{gs}$ is the gate-source voltage, $V_{th}$ is the threshold voltage, $n$ is the subthreshold swing coefficient, and $V_{T}$ is the thermal voltage.

1. **Gate Leakage**: This is the current that flows directly through the gate oxide due to quantum mechanical tunneling: $I_{gate}$. 

The total leakage power, $P_{leakage}$, can be calculated by multiplying the total leakage current, $I_{leakage}$, by the supply voltage, $V_{dd}$:

$$
P_{leakage} = V_{dd} \times I_{leakage}
$$

where $I_{leakage} = I_{sub} + I_{gate}$.

Leakage power has become a significant concern in digital design due to technology scaling. As the size of transistors decreases, the gate oxide thickness also decreases, leading to an increase in gate leakage. Similarly, the decrease in threshold voltage to maintain performance leads to an increase in subthreshold leakage.

> For further reading, you may want to look into the following resources:
> - [Leakage Power in CMOS Circuits](https://www.google.com/search?q=Leakage+Power+in+CMOS+Circuits)
> - [Understanding and Minimizing CMOS Leakage](https://www.google.com/search?q=Understanding+and+Minimizing+CMOS+Leakage)
> - [Leakage Current Mechanisms and Reduction Techniques in Deep-Submicrometer CMOS Circuits](https://www.google.com/search?q=Leakage+Current+Mechanisms+and+Reduction+Techniques+in+Deep-Submicrometer+CMOS+Circuits)