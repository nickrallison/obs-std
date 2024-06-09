---
bad_links: 
aliases: [bandwidth]
tags: [signalprocessing]
---
# Signal Bandwidth

Signal Bandwidth, in the context of telecommunications and signal processing, refers to the difference between the upper and lower frequencies in a [[Continuity|continuous]] set of frequencies. It is a key parameter of a signal and system, as it determines the range of frequencies that a system can transmit without significant loss of information.

The bandwidth of a signal can be mathematically represented as:

$$
BW = f_{max} - f_{min}
$$

where $f_{max}$ is the maximum frequency and $f_{min}$ is the minimum frequency.

The concept of bandwidth is closely related to the data rate (the amount of data that can be transmitted in a given amount of time) through the Shannon Capacity formula:

$$
C = B \log_2(1 + SNR)
$$

where $C$ is the maximum achievable data rate (capacity), $B$ is the bandwidth, and $SNR$ is the signal-to-noise ratio. This formula, derived by Claude Shannon, shows that the data rate is directly proportional to the bandwidth: the larger the bandwidth, the higher the potential data rate.

Bandwidth is also related to the concept of the [[Fourier Transform|Fourier Transform]], which allows us to analyze the frequency components of a signal. The [[Fourier Transform|Fourier Transform]] of a time-domain signal gives us its frequency-domain representation, showing us the signal's behavior over a range of frequencies. This is crucial in understanding the bandwidth of a signal.

The [[Fourier Transform|Fourier Transform]] is given by:

$$
F(f) = \int_{-\infty}^{\infty} f(t) e^{-j2\pi ft} dt
$$

where $F(f)$ is the [[Fourier Transform|Fourier Transform]] of the function $f(t)$.

> For more in-depth reading, you may refer to the following resources:
> - [Fourier Transform](https://www.google.com/search?q=Fourier+Transform)
> - [Shannon Capacity](https://www.google.com/search?q=Shannon+Capacity)
> - [Signal Bandwidth](https://www.google.com/search?q=Signal+Bandwidth)