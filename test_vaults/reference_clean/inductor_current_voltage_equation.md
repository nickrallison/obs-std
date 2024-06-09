---
aliases: [Inductor]
tags: [electronics]
bad_links:
---
# Inductor Current Voltage Equation

An inductor is a passive electrical component that stores energy in its magnetic field. The current-voltage relationship in an inductor is described by Faraday's law of electromagnetic induction, which states that the induced electromotive force (EMF) in any closed circuit is equal to the rate of change of the magnetic flux through the circuit.
**
The voltage across an inductor $L$ is directly proportional to the rate of change of current through it. This relationship is mathematically represented as:

$$
v(t) = L \frac{di(t)}{dt}
$$

where:
- $v(t)$ is the voltage across the inductor,
- $L$ is the inductance of the inductor, and
- $\frac{di(t)}{dt}$ is the rate of change of current through the inductor.

The above equation is a differential equation, and its solution gives the current $i(t)$ as a function of time. If the voltage $v(t)$ is known, the current can be found by integrating the above equation:

$$
i(t) = \frac{1}{L} \int v(t) dt
$$

The inductor current-voltage equation is fundamental in the analysis of AC circuits, where the current and voltage are sinusoidal functions of time. In such cases, the inductor presents a reactance to the AC current given by:

$$
X_L = 2\pi fL
$$

where:
- $X_L$ is the inductive reactance,
- $f$ is the frequency of the AC signal.

The phase difference between the voltage and current in an inductor in an AC circuit is $90^\circ$, with the current lagging behind the voltage.

> For more context and reading, you may want to look into the following resources:
> - [Faraday's Law of Electromagnetic Induction](https://www.google.com/search?q=Faraday%27s+Law+of+Electromagnetic+Induction)
> - [Inductor](https://www.google.com/search?q=Inductor)
> - [AC Circuits](https://www.google.com/search?q=AC+Circuits)
> - [Inductive Reactance](https://www.google.com/search?q=Inductive+Reactance)