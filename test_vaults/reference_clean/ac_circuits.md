---
bad_links: 
aliases: []
tags: [electronics]
---
# AC Circuits

Alternating Current (AC) circuits are electrical circuits that deal with alternating current, which is electrical current that changes direction periodically. This is in contrast to Direct Current (DC) circuits, where the current flows in one direction only. The standard form of an AC voltage source is $v(t) = V_m \cos(\omega t + \phi)$, where $V_m$ is the peak voltage, $\omega$ is the angular frequency, and $\phi$ is the phase angle.

AC circuits can contain resistors, capacitors, and inductors. The behavior of these components in an AC circuit is different from their behavior in a DC circuit due to the changing nature of the current.

1. **Resistors**: In an AC circuit, resistors behave the same way as they do in a DC circuit. Ohm's law ($V = IR$) still applies, where $V$ is voltage, $I$ is current, and $R$ is resistance. The power dissipated in a resistor is given by $P = IV = I^2R = V^2/R$.

2. **Capacitors**: In an AC circuit, capacitors oppose a change in voltage. The reactance (opposition to AC current) of a capacitor is given by $X_C = 1/(2\pi fC)$, where $f$ is the frequency and $C$ is the capacitance. The current and voltage in a capacitor are out of phase by 90 degrees, with current leading voltage.

3. **Inductors**: In an AC circuit, inductors oppose a change in current. The reactance of an inductor is given by $X_L = 2\pi fL$, where $L$ is the inductance. The current and voltage in an inductor are out of phase by 90 degrees, with voltage leading current.

In AC circuits, the combination of resistance and reactance is called impedance, denoted by $Z$. For a series AC circuit, $Z = \sqrt{R^2 + (X_L - X_C)^2}$, and the current in the circuit is given by $I = V/Z$.

The power in an AC circuit is more complex than in a DC circuit. It involves real power (P), reactive power (Q), and apparent power (S), and is given by $S = P + jQ = VI^*$, where $I^*$ is the complex conjugate of the current. The power factor, which is the cosine of the phase angle between the current and voltage, is also an important concept in AC power.

> For more detailed information, you can refer to the following resources:
> - [AC Circuit - Wikipedia](https://www.google.com/search?q=AC+Circuit+site:wikipedia.org)
> - [AC Power - Wikipedia](https://www.google.com/search?q=AC+Power+site:wikipedia.org)
> - [Impedance and Reactance - Electronics Tutorials](https://www.google.com/search?q=Impedance+and+Reactance+site:electronics-tutorials.ws)
> - [AC Power Analysis - All About Circuits](https://www.google.com/search?q=AC+Power+Analysis+site:allaboutcircuits.com)