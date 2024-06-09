---
bad_links: 
aliases: []
tags: [signalprocessing, electronics]
title: AM Modulator
date created: Sunday, July 16th 2023, 10:46:04 am
---
# [[Amplitude Modulation|AM]] Modulator

An [[Amplitude Modulation|AM]] modulator, or [[Amplitude Modulation|Amplitude Modulation]] modulator, is a device that is used to vary the strength of a signal in proportion to the [[Information Theory|information]] being sent. This process allows the signal to carry [[Information Theory|information]] such as sound or video. The modulator essentially combines a carrier signal with the input signal to create an output signal that can be transmitted over long distances. Its commonly used in radio broadcasting and other forms of communication systems.

The circuit that modulates an [[Amplitude Modulation|AM]] signal is composed of a modulator. This device combines the carrier signal with the message signal to produce an amplitude-modulated signal. The modulator usually includes components such as oscillators, amplifiers, and transformers.

## Example

Given a carrier signal with frequency 1000 Hz and amplitude 5V, and a message signal with frequency 500 Hz and amplitude 2V, calculate the output signal of the [[Amplitude Modulation|AM]] modulator.

Solution:

The output signal of an [[Amplitude Modulation|AM]] modulator is given by the equation:

$$
\begin{gather*} 
y(t) = (1+mu*cos(2*\pi*f_m*t))*A_c*cos(2*\pi*f_c*t) \newline
\text{where,} \newline
y(t) = \text{output signal} \newline
mu = \text{modulation index (ratio of amplitude of message signal to carrier signal)} \newline
f_m = \text{frequency of message signal} \newline
A_c = \text{amplitude of carrier signal} \newline
f_c = \text{frequency of carrier signal} 
\end{gather*}
$$

Substituting the given values into the equation:

$$
\begin{gather*} 
\mu = A_m/A_c = 2/5 = 0.4 \newline
f_m = 500 Hz, A_c = 5V, f_c = 1000 Hz. 
\end{gather*}
$$

Therefore,

$$
\begin{gather*} 
y(t) = (1+0.4*cos(2*\pi*500*t))*5*cos(2*\pi*1000*t)
\end{gather*}
$$

This is the output signal from the [[Amplitude Modulation|AM]] modulator.
