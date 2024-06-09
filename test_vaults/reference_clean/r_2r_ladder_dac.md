---
bad_links: 
aliases: []
tags: [electronics]
---
# R-2R Ladder DAC

The R-2R ladder DAC is a simple, effective, and efficient method to convert a digital signal into an analog signal. It is called an R-2R ladder because of the resistor values used in the network and its ladder-like structure.

The R-2R ladder DAC consists of a network of resistors and switches. The resistors have two distinct values: R and 2R. The switches are connected to the digital input bits and can connect the corresponding node to either the reference voltage (Vref) or ground (0V), depending on whether the bit is 1 or 0.

The operation of the R-2R ladder DAC can be understood by analyzing the network using Ohm's Law and the principle of superposition. The output voltage (Vout) is the sum of the contributions from each bit, each of which is a fraction of the reference voltage determined by the position of the bit in the binary number.

The formula for the output voltage is:

$$
V_{out} = V_{ref} \times \left( \frac{b_{n-1}}{2^1} + \frac{b_{n-2}}{2^2} + \ldots + \frac{b_{0}}{2^n} \right)
$$

where $b_{n-1}$ to $b_{0}$ are the bits of the digital input, $n$ is the number of bits, and $V_{ref}$ is the reference voltage.

The R-2R ladder DAC has several advantages over other types of DACs. It requires fewer unique resistor values (only R and 2R), it has a high resolution (determined by the number of bits), and it has a fast conversion time (determined by the speed of the switches).

Tangentially related items include other types of DACs such as the binary-weighted DAC, which uses a different resistor network and has different performance characteristics, and the concept of resolution and accuracy in DACs, which are important factors in the quality of the analog output signal.

> For more information, you can refer to this detailed explanation of the [R-2R ladder DAC](https://www.google.com/search?q=R-2R+ladder+DAC) and this comparison of different [types of DACs](https://www.google.com/search?q=types+of+DACs).