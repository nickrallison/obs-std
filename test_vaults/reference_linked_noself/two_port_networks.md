---
bad_links: 
aliases: []
tags: [electronics]
title: Two Port Networks
date created: Monday, July 24th 2023, 7:32:04 pm
---
# Two Port Networks

Two-port networks are a fundamental concept in electrical engineering and network theory. They are used to describe the behavior of a system (the "network") in terms of its input and output. The network is considered "black box" in nature, meaning we are only concerned with what goes in and what comes out, not what happens inside.

A two-port network can be represented by four variables: two input variables and two output variables. These variables can be either [[Voltage|voltages]] or currents. The relationship between these variables is described by the two-port parameters, which can be represented in several forms, including Z-parameters, Y-parameters, H-parameters, G-parameters, T-parameters, and S-parameters. Each of these parameter sets provides a different perspective on the network's behavior.

Let's take a look at each of these parameter sets:

1. **Z-parameters (Impedance parameters)**: These parameters relate the input and output [[Voltage|voltages]] and currents in terms of impedance. The Z-parameters are given by the following matrix equation:

$$
\begin{bmatrix}
V_1 \\
V_2
\end{bmatrix}
=
\begin{bmatrix}
Z_{11} & Z_{12} \\
Z_{21} & Z_{22}
\end{bmatrix}
\begin{bmatrix}
I_1 \\
I_2
\end{bmatrix}
$$

where $V_1$ and $V_2$ are the input and output [[Voltage|voltages]], $I_1$ and $I_2$ are the input and output currents, and $Z_{ij}$ are the Z-parameters.

1. **Y-parameters (Admittance parameters)**: These parameters relate the input and output [[Voltage|voltages]] and currents in terms of admittance. The Y-parameters are given by the following matrix equation:

$$
\begin{bmatrix}
I_1 \\
I_2
\end{bmatrix}
=
\begin{bmatrix}
Y_{11} & Y_{12} \\
Y_{21} & Y_{22}
\end{bmatrix}
\begin{bmatrix}
V_1 \\
V_2
\end{bmatrix}
$$

where $Y_{ij}$ are the Y-parameters.

1. **H-parameters (Hybrid parameters)**: These parameters are a hybrid of impedances and admittances. The H-parameters are given by the following matrix equation:

$$
\begin{bmatrix}
V_1 \\
I_2
\end{bmatrix}
=
\begin{bmatrix}
h_{11} & h_{12} \\
h_{21} & h_{22}
\end{bmatrix}
\begin{bmatrix}
I_1 \\
V_2
\end{bmatrix}
$$

where $h_{ij}$ are the H-parameters.

1. **G-parameters (Inverse hybrid parameters)**: These parameters are the inverse of the hybrid parameters. The G-parameters are given by the following matrix equation:

$$
\begin{bmatrix}
I_1 \\
V_2
\end{bmatrix}
=
\begin{bmatrix}
g_{11} & g_{12} \\
g_{21} & g_{22}
\end{bmatrix}
\begin{bmatrix}
V_1 \\
I_2
\end{bmatrix}
$$

where $g_{ij}$ are the G-parameters.

1. **T-parameters (Transmission parameters)**: These parameters describe the transmission properties of the network. The T-parameters are given by the following matrix equation:

$$
\begin{bmatrix}
V_1 \\
I_1
\end{bmatrix}
=
\begin{bmatrix}
T_{11} & T_{12} \\
T_{21} & T_{22}
\end{bmatrix}
\begin{bmatrix}
V_2 \\
I_2
\end{bmatrix}
$$

where $T_{ij}$ are the T-parameters.

1. **S-parameters (Scattering parameters)**: These parameters are used in high-frequency circuit analysis and describe the power flow in a network. The S-parameters are given by the following matrix equation:

$$
\begin{bmatrix}
a_1 \\
a_2
\end{bmatrix}
=
\begin{bmatrix}
S_{11} & S_{12} \\
S_{21} & S_{22}
\end{bmatrix}
\begin{bmatrix}
b_1 \\
b_2
\end{bmatrix}
$$

where $a_i$ and $b_i$ are the incident and reflected power waves, and $S_{ij}$ are the S-parameters.

Each of these parameter sets provides a different perspective on the network's behavior and can be used in different situations. For example, Z-parameters are often used in low-frequency circuit analysis, while S-parameters are used in high-frequency circuit analysis.

> For more in-depth information, you may want to check out the following resources:
> - [Two-port network (Wikipedia)](https://www.google.com/search?q=Two-port+network+Wikipedia)
> - [Two-port parameters (Wikipedia)](https://www.google.com/search?q=Two-port+parameters+Wikipedia)
> - [Network analysis (electrical circuits) (Wikipedia)](https://www.google.com/search?q=Network+analysis+electrical+circuits+Wikipedia)
> - [Scattering parameters (Wikipedia)](https://www.google.com/search?q=Scattering+parameters+Wikipedia)
> - [Impedance parameters (Wikipedia)](https://www.google.com/search?q=Impedance+parameters+Wikipedia)
> - [Admittance parameters (Wikipedia)](https://www.google.com/search?q=Admittance+parameters+Wikipedia)
> - [Hybrid parameters (Wikipedia)](https://www.google.com/search?q=Hybrid+parameters+Wikipedia)
> - [Inverse hybrid parameters (Wikipedia)](https://www.google.com/search?q=Inverse+hybrid+parameters+Wikipedia)
> - [Transmission parameters (Wikipedia)](https://www.google.com/search?q=Transmission+parameters+Wikipedia)