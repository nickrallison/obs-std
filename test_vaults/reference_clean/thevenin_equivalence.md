---
bad_links: 
aliases: []
tags: [electronics]
title: Thevenin Equivalence
date created: Monday, July 24th 2023, 7:25:34 pm
---
# Thevenin Equivalence

Thevenin's theorem, also known as Thevenin equivalence, is a principle used in electrical engineering to simplify complex linear circuits and systems. It states that any combination of voltage sources, current sources, and resistors with two terminals can be replaced by a single voltage source (called Thevenin voltage) and a single series resistor (called Thevenin resistance). This simplification makes it easier to analyze the circuit's behavior. To find the Thevenin equivalent of a circuit, one needs to calculate the open-circuit voltage and short-circuit current at the terminals of the circuit.

Thevenin's theorem can be represented in a general form as follows:

$$
\begin{gather*} 
V_{TH} = V_{OC} \\
R_{TH} = \frac{V_{OC}}{I_{SC}}
\end{gather*}
$$

where:
- $V_{TH}$ is the Thevenin voltage,
- $R_{TH}$ is the Thevenin resistance,
- $V_{OC}$ is the open-circuit voltage, and
- $I_{SC}$ is the short-circuit current.
