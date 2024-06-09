---
bad_links: 
aliases: []
date created: Monday, June 26th 2023, 3:32:28 pm
tags: [signalprocessing]
title: "Carson's Rule"
---

# Carsons Rule

Carson's Rule is a rule used in communications engineering to estimate the bandwidth of a frequency modulated (FM) signal. It's named after John Renshaw Carson who introduced it in 1922. The rule provides a simple and reasonably accurate method to calculate the bandwidth of an FM signal, which is crucial for the design and analysis of communication systems.

The rule states that nearly all (~98%) of the power of a frequency-modulated signal lies within a bandwidth $B_T$ of:

$$
B_T = 2(\Delta f + f_m)
$$

where:
- $\Delta f$ is the peak frequency deviation, i.e., the maximum difference between the instantaneous frequency and the carrier frequency.
- $f_m$ is the highest frequency in the modulating signal.

This formula is derived from the Bessel function expansion of an FM signal, which shows that the bandwidth is theoretically infinite since the Bessel functions extend to infinity. However, Carson's Rule provides a practical estimate because the higher order Bessel functions become negligible beyond a certain point.

Carson's Rule is widely used because of its simplicity, but it's important to note that it's an approximation. It assumes that the modulating signal is a single tone, which is rarely the case in real-world applications. For complex signals, the rule tends to overestimate the bandwidth.

> For more in-depth understanding, you may want to read about the [Bessel function](https://www.google.com/search?q=Bessel+function) and its role in FM signal analysis. Also, understanding the [Fourier Transform](https://www.google.com/search?q=Fourier+Transform) can be beneficial as it's used in deriving the spectrum of an FM signal.