---
bad_links: 
aliases: []
tags: [electronics]
title: MOSFET Current
date created: Saturday, July 15th 2023, 4:12:54 pm
---
# MOSFET Transistor Current

The MOSFET Transistor (Metal-Oxide-Semiconductor Field-Effect Transistor) current equation is a mathematical expression that describes how current flows through a MOSFET Transistor. The equation is used to calculate the drain current (Id) of a MOSFET Transistor transistor, which depends on various factors such as gate voltage, threshold voltage, and the physical characteristics of the transistor. The equation varies depending on whether the MOSFET Transistor is in saturation mode, triode mode or cut-off mode. Understanding and applying this equation is crucial in designing and analyzing electronic circuits that use MOSFETs.

The MOSFET Transistor current equations for different modes are as follows:

For the cut-off mode:
$$
\begin{gather*} 
I_D = 0 \quad \text{for} \quad V_{GS} < V_{th}
\end{gather*}
$$

For the triode mode:
$$
\begin{gather*} 
I_D = K_n' \cdot (W/L) \cdot ((V_{GS} - V_{th})V_{DS} - V_{DS}^2/2) \quad \text{for} \quad V_{GS} > V_{th}, V_{DS}< (V_{GS}-V_{th})
\end{gather*}
$$

For the saturation mode:
$$
\begin{gather*} 
I_D = 1/2 \cdot K_n' \cdot (W/L) \cdot (V_{GS}-V_{th})^2  (1 + λV_DS)  \quad \text{for}  \quad V_{DS}> (V_{GS}-V_{th})
\end{gather*}
$$

Where:
- $I_D$ is the drain current,
- $K_n$ is the transconductance parameter,
- $W$ and $L$ are the width and length of the MOSFET Transistor channel respectively,
- $V_{GS}$ is the gate-source voltage,
- $V_{DS}$ is the drain-source voltage,
- $λ$ is the channel-length modulation parameter,
- $V_th$ is the threshold voltage.
