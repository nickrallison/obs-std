---
bad_links: 
aliases: [DFT]
tags: [signalprocessing, calculus]
date created: Monday, June 26th 2023, 3:32:29 pm
title: Discrete Fourier Transform
---

# Discrete Fourier Transform

The Discrete Fourier Transform (DFT) is a mathematical technique used to transform a sequence of complex or real numbers into a sequence of coefficients that correspond to frequencies of a set of sinusoids. It is commonly used in signal processing and data analysis to analyze the frequencies within a signal or dataset.  
In simpler terms, it transforms data from the time (or spatial) domain to the frequency domain. Its like revealing the individual musical notes that make up a complex sound.  
The DFT analyzes the finite period by considering it as one period of a periodically repeating signal, which is valid when the original data is exactly periodic within that period. The result will also be periodic over this range.  
Its important to note that DFT can be computationally expensive, but there are efficient algorithms to compute it, such as Fast Fourier Transform (FFT).

The Discrete Fourier Transform (DFT) doesnt have a single formula, but a set of formulas. Here are the main ones:

For a sequence of N numbers $x_0, x_1, …, x_{N-1}$, the kth DFT coefficient X_k is given by:

$$
X_k = \sum_{n=0}^{N-1} x_n \cdot e^{-i 2\pi kn/N}
$$

And the inverse DFT, which transforms from the frequency domain back to the time domain, is given by:

$$
x_n = \frac{1}{N} \sum_{k=0}^{N-1} X_k \cdot e^{i 2\pi kn/N}
$$

Where:
- $x_n$ are the time-domain samples,
- $X_k$ are the frequency-domain samples,
- N is the total number of samples,
- n is the current sample number (for time-domain),
- k is the current sample number (for frequency-domain),
- i is the imaginary unit.
