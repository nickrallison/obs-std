---
bad_links: 
aliases: [AM]
tags: [signalprocessing, electronics]
title: Amplitude Modulation
date created: Wednesday, July 12th 2023, 12:18:19 pm
---

# Amplitude Modulation

Amplitude Modulation (AM) is a technique used in electronic communication, most commonly for transmitting [[Information Theory|information]] via a radio carrier wave. It works by varying the strength or amplitude of the carrier wave in proportion to the waveform being sent. This waveform may correspond to sounds or light intensity that is to be reproduced by the receiver. AM differs from [[Frequency Modulation|Frequency Modulation]] ([[Frequency Modulation|FM]]) and [[Phase Modulation|Phase Modulation]] ([[Phase Modulation|PM]]), where the frequency and phase of the carrier wave are varied respectively. AM is used in many forms of communication like in broadcasting AM radio, one-way voice communication, and two-way radio communication.

Here is an example of calculating the modulation index in Amplitude Modulation:

$$
\begin{gather*} 
\text{Given: } V_m = 10V, V_c = 5V \\
\text{Modulation Index (m) is given by: } m = \frac{V_m}{V_c} \\
m = \frac{10}{5} \\
m = 2\\
\end{gather*}
$$

Here, $V_m$ is the peak amplitude of the message signal and $V_c$ is the peak amplitude of the carrier signal. The modulation index ($m$) in this case is 2.

## Spectra of an AM Signal

The frequency spectrum is a representation of a signal in the frequency domain. It shows the different frequencies that make up the signal and their respective amplitudes.

$$
m(t) = A[1+u\sin(2\pi f_m t)]\cos(2\pi f_c t)
$$

Where:
- $m(t)$ is the modulated signal
- $A$ is the amplitude of the carrier wave
- $u$ is the modulation index (which is less than or equal to 1)
- $f_m$ is the frequency of the message signal
- $f_c$ is the frequency of the carrier wave

The spectra or frequency domain representation of an AM signal can be obtained by taking the [[Fourier Transform|Fourier transform]] of $m(t)$.

The [[Signal Bandwidth|bandwidth]] required for transmitting an AM signal according to [[Carson's Rule|Carson's rule]] can be calculated using:

$$
BW = 2f_m
$$