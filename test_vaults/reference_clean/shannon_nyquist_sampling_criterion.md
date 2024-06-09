---
bad_links: 
aliases: [Nyquist-Shannon sampling theorem, Nyquist-Shannon sampling criterion, Nyquist-Shannon theorem, Nyquist Theorem]
date created: Monday, June 26th 2023, 3:32:29 pm
tags: [signalprocessing]
title: Shannon-Nyquist Sampling Criterion
---

# Shannon-Nyquist Sampling Criterion

The Shannon-Nyquist Sampling Theorem, often simply referred to as the Nyquist Theorem, is a fundamental principle in the field of digital signal processing and telecommunications. It provides a bridge between continuous-time (analog) signals and discrete-time (digital) signals.

The theorem is named after Claude Shannon and Harry Nyquist, who independently formulated the theorem. The theorem states that a bandlimited analog signal can be perfectly reconstructed from an infinite sequence of samples if the sampling rate is greater than twice the highest frequency of the signal.

Mathematically, this can be expressed as:

$$f_s > 2f_{max}$$

where $f_s$ is the sampling frequency (or rate) and $f_{max}$ is the highest frequency component in the signal.

The factor of 2 is the critical part of this theorem. It means that to accurately sample a signal, you need to sample at least twice as fast as the highest frequency you want to capture. This is often referred to as the Nyquist rate.

If the sampling theorem is not adhered to, and the sampling frequency is less than twice the highest frequency component of the signal, a phenomenon known as aliasing occurs. Aliasing is the distortion that occurs when high-frequency signal components are misinterpreted as lower frequency components during the sampling process.

The proof of the Nyquist-Shannon theorem is based on the mathematical field of Fourier analysis. The theorem leverages the fact that a bandlimited function can be represented by an infinite series of sine and cosine functions (its Fourier series), and that these functions can be sampled and perfectly reconstructed if sampled at a rate greater than twice their frequency.

The theorem has profound implications in the field of digital signal processing and has enabled the digital revolution in telecommunications and audio and video technology. It is the fundamental principle behind the conversion of analog signals to digital signals, a process that is ubiquitous in modern technology.

> For further reading, you might find the following resources helpful:
> - [Shannon-Nyquist Sampling Theorem](https://www.google.com/search?q=Shannon-Nyquist+Sampling+Theorem)
> - [Fourier Analysis](https://www.google.com/search?q=Fourier+Analysis)
> - [Aliasing](https://www.google.com/search?q=Aliasing)
> - [Digital Signal Processing](https://www.google.com/search?q=Digital+Signal+Processing)