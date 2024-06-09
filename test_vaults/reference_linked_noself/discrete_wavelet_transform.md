---
bad_links: 
aliases: [DWT]
tags: [signalprocessing]
---
# [[Wavelet Transform|Wavelet]] Transform.md|Discrete [[Wavelet Transform|Wavelet]] Transform]]
**Expert**: Signal Processing Engineer  
**Objective**: To provide a comprehensive explanation of [[Wavelet Transform|Wavelet]] Transform.md|Discrete [[Wavelet Transform|Wavelet]] Transform]] (DWT), including relevant formulas, tangentially related items, derivations, and proofs.  
**Assumptions**: You have a basic understanding of signal processing, [[Fourier Transform|Fourier transforms]], and linear algebra.

The [[Wavelet Transform|Wavelet]] Transform.md|Discrete [[Wavelet Transform|Wavelet]] Transform]] (DWT) is a mathematical technique used in signal processing and image analysis to represent data or functions. It is particularly useful for non-stationary signals, where frequency components may vary over time. 

The DWT operates by breaking down a signal into a set of basis functions, called [[Wavelet Transform|wavelets]]. These [[Wavelet Transform|wavelets]] are created by translations and dilations of a fixed mother [[Wavelet Transform|wavelet]] $\psi(t)$. The DWT of a signal $x(t)$ can be computed as:

$$
X(a,b) = \frac{1}{\sqrt{|a|}}\int_{-\infty}^{\infty}x(t)\psi\left(\frac{t-b}{a}\right)dt
$$

where $a$ and $b$ are the scale and translation parameters, respectively. 

The DWT is computed over all scales in a dyadic (powers of two) scale, which makes it particularly efficient for digital signals. The DWT at scale $a=2^j$ and position $b=k2^j$ is given by:

$$
X(j,k) = \frac{1}{\sqrt{2^j}}\int_{-\infty}^{\infty}x(t)\psi\left(\frac{t-k2^j}{2^j}\right)dt
$$

The DWT can be computed efficiently using a filter bank, which consists of a high-pass filter $g[n]$ and a low-pass filter $h[n]$, followed by a downsampler. The filters are derived from the mother [[Wavelet Transform|wavelet]] and its scaling function $\phi(t)$.

The DWT has many applications, including data compression, noise reduction, and feature extraction. It is particularly useful for signals that have transient characteristics, where frequency components change over time.

For more in-depth understanding, you may want to look into the following topics:

- [[Fourier Transform|Fourier Transform]] and its limitations
- [[Wavelet Transform|Wavelet]] Transform]] ([[Wavelet Transform|CWT]]) and its discretization
- Construction of [[Wavelet Transform|wavelets]] and scaling functions
- Multiresolution analysis and filter banks
- Applications of DWT in signal and image processing

> [Discrete Wavelet Transform - Wikipedia](https://www.google.com/search?q=Discrete+Wavelet+Transform+site:wikipedia.org)  
> [Wavelet Analysis - MathWorks](https://www.google.com/search?q=Wavelet+Analysis+site:mathworks.com)  
> [A Really Friendly Guide to Wavelets](https://www.google.com/search?q=A+Really+Friendly+Guide+to+Wavelets)  
> [Wavelets for Computer Graphics: A Primer - Eric J. Stollnitz, Tony D. DeRose, David H. Salesin](https://www.google.com/search?q=Wavelets+for+Computer+Graphics%3A+A+Primer)