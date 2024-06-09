---
aliases: [wavelet, CWT, continuous wavelet transform]
tags: [signalprocessing]
title: Wavelet Transform
date created: Sunday, July 16th 2023, 10:08:55 am
bad_links: [Continuity.md]
---
# Wavelet Transform

## Explanation
The Wavelet Transform is a mathematical tool used in signal processing to analyze signals at different scales. It provides a time-frequency representation of a signal, allowing us to analyze both the time and frequency characteristics simultaneously.

The [[Continuity|continuous]] wavelet transform (CWT) is defined as follows:

$$
CWT(a, b) = \int_{-\infty}^{\infty} x(t) \cdot \psi^*\left(\frac{t-b}{a}\right) \, dt
$$

where $x(t)$ is the input signal, $\psi(t)$ is the analyzing wavelet, $a$ is the scale parameter, and $b$ is the translation parameter.

The analyzing wavelet $\psi(t)$ is a small, localized waveform that is used to probe the input signal. It is typically a scaled and translated version of a mother wavelet function $\psi_0(t)$:

$$
\psi(t) = \frac{1}{\sqrt{a}} \psi_0\left(\frac{t-b}{a}\right)
$$

The mother wavelet function $\psi_0(t)$ is a function that satisfies certain properties, such as having zero mean and finite energy. Commonly used wavelets include the Haar wavelet, Daubechies wavelets, and Morlet wavelet.

The CWT provides a time-frequency representation of the signal by varying the scale and translation parameters. By choosing different scales, we can analyze the signal at different resolutions. Smaller scales provide higher frequency resolution, while larger scales provide better time resolution.

The CWT can be implemented using the [[Convolution|convolution operation]]. The wavelet function $\psi(t)$ is convolved with the input signal $x(t)$ at different scales and translations. This [[Convolution|convolution operation]] captures the similarity between the wavelet and the signal at different time points.

One important property of the CWT is the admissibility condition, which ensures that the CWT preserves the energy of the signal. The admissibility condition states that the analyzing wavelet must satisfy:

$$
\int_{-\infty}^{\infty} |\psi(t)|^2 \, dt < \infty
$$

This condition ensures that the CWT does not introduce any additional energy into the signal.

The CWT has several advantages over other time-frequency analysis techniques, such as the [[Fourier Transform|Fourier Transform]]. It can capture both localized and non-stationary features of a signal, making it suitable for analyzing signals with transient or time-varying characteristics. It also provides a multi-resolution analysis, allowing us to analyze the signal at different scales.

In practice, the CWT is often implemented using Wavelet Transform.md|discrete wavelet transforms]] ([[Discrete Wavelet Transform|DWT]]) for computational efficiency. The [[Discrete Wavelet Transform|DWT]] approximates the CWT by discretizing the scale and translation parameters and using a finite set of wavelet functions.

For a more detailed understanding of the Wavelet Transform, you can refer to the following resources:
- [Wavelet Transform - Wikipedia](https://en.wikipedia.org/wiki/Wavelet_transform)
- [A Wavelet Tour of Signal Processing by Mallat](https://www.amazon.com/Wavelet-Tour-Signal-Processing-Third/dp/0123743702)
