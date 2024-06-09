---
bad_links: 
aliases: [NPN, PNP]
tags: [electronics]
---
# BJT [[Transistor|Transistor]]

A Bipolar Junction [[Transistor|Transistor]] (BJT) is a type of [[Transistor|transistor]] that uses both electron and hole charge carriers. The term "bipolar" refers to the use of both types of charge carriers in its operation. There are two types of BJTs, NPN and PNP, which are differentiated by the type of charge carriers that they use for conduction.

A BJT consists of three layers of semiconductor material, either P-type or N-type, sandwiched together. The three layers correspond to the emitter, base, and collector regions. The emitter-base junction is forward biased, meaning that it allows current flow, while the base-collector junction is reverse biased, meaning that it prevents current flow.

The operation of a BJT can be explained by the Ebers-Moll model, which is a set of equations that describe the behavior of a BJT. The Ebers-Moll model is based on the assumption that the [[Transistor|transistor]] is in the active mode, meaning that the emitter-base junction is forward biased and the base-collector junction is reverse biased.

The Ebers-Moll equations are as follows:

1. The collector current $I_C$ is given by:

$$
I_C = I_S \left( e^{\frac{V_{BE}}{V_T}} - 1 \right) - I_S \left( e^{\frac{V_{BC}}{V_T}} - 1 \right)
$$

1. The emitter current $I_E$ is given by:

$$
I_E = I_S \left( e^{\frac{V_{BE}}{V_T}} - 1 \right) + I_S \left( e^{\frac{V_{BC}}{V_T}} - 1 \right)
$$

1. The base current $I_B$ is given by:

$$
I_B = I_E - I_C
$$

where $I_S$ is the saturation current, $V_{BE}$ is the [[Voltage|voltage]] across the base-emitter junction, $V_{BC}$ is the [[Voltage|voltage]] across the base-collector junction, and $V_T$ is the thermal [[Voltage|voltage.]]

The Ebers-Moll model is a good approximation for the behavior of a BJT, but it does not account for all possible operating conditions. For a more accurate model, other factors such as the Early effect, base-width modulation, and high-injection effects must be considered.

> For more in-depth information, you can refer to the following resources:
> - [BJT (Bipolar Junction Transistor) - Electronics Tutorials](https://www.google.com/search?q=BJT+Electronics+Tutorials)
> - [Ebers-Moll Model - Electronics Tutorials](https://www.google.com/search?q=Ebers-Moll+Model+Electronics+Tutorials)
> - [Early Effect - Electronics Tutorials](https://www.google.com/search?q=Early+Effect+Electronics+Tutorials)
> - [Base-Width Modulation - Electronics Tutorials](https://www.google.com/search?q=Base-Width+Modulation+Electronics+Tutorials)
> - [High-Injection Effects - Electronics Tutorials](https://www.google.com/search?q=High-Injection+Effects+Electronics+Tutorials)