---
bad_links: 
aliases: [ADC]
tags: [electronics]
---
# Analog-to-Digital Converters

Analog-to-Digital Converters (ADCs) are critical components in digital and mixed-signal systems. They serve as an interface between the analog and digital worlds by converting [[Continuity|continuous]] analog signals into discrete digital values.

## Basic Principle

The basic principle of an ADC involves sampling the analog input signal at regular intervals and then quantizing these samples into a finite set of possible output levels. This process involves two key steps: sampling and quantization.

### Sampling

Sampling is the process of taking measurements of the analog signal at discrete points in time. The rate at which these measurements are taken is known as the sampling rate or sampling frequency ($f_s$). According to the [[Shannon-Nyquist Sampling Criterion|Nyquist-Shannon sampling theorem]], the sampling frequency must be at least twice the highest frequency component of the input signal to accurately represent the signal. Mathematically, this is expressed as:

$$
f_s \geq 2f_{max}
$$

where $f_{max}$ is the highest frequency component of the input signal.

### Quantization

Quantization is the process of mapping the sampled analog values to a finite set of discrete levels. This process inherently introduces some error, known as quantization error. The number of discrete levels is determined by the resolution of the ADC, which is typically expressed in bits. An $n$-bit ADC can represent $2^n$ discrete levels. The smallest change in [[Voltage|voltage]] that can be detected by the ADC, known as the least significant bit (LSB), can be calculated as:

$$
LSB = \frac{V_{ref}}{2^n}
$$

where $V_{ref}$ is the reference [[Voltage|voltage]] of the ADC.

## Types of ADCs

There are several types of ADCs, each with its own method of operation, advantages, and disadvantages. Some of the most common types include:

1. **Flash ADCs**: Also known as parallel ADCs, these are the fastest type of ADC but require a large number of comparators, making them impractical for high-resolution applications.

2. **Successive Approximation Register (SAR) ADCs**: These use a [[Binary Search|binary search]] algorithm to find the closest match to the input [[Voltage|voltage]]. They offer a good balance between speed, resolution, and complexity.

3. **Sigma-Delta ($\Sigma\Delta$) ADCs**: These use oversampling and noise shaping to achieve high resolution. They are commonly used in audio and precision measurement applications.

4. **Dual Slope ADCs**: These are highly accurate but slow, making them suitable for precision measurement applications where speed is not critical.

## Tangentially Related Items

ADCs are often used in conjunction with [[Digital-to-Analog Converters|Digital-to-Analog Converters]] ([[Digital-to-Analog Converters|DACs]]) in digital systems. While ADCs convert analog signals to digital, [[Digital-to-Analog Converters|DACs]] do the opposite, converting digital values back into analog signals.

ADCs are also closely related to the concept of digital signal processing (DSP), which involves manipulating digital signals to improve their quality or extract useful information.

> For more in-depth reading, you may refer to the following resources:
> - [Analog-to-Digital Converters](https://www.google.com/search?q=Analog-to-Digital+Converters)
> - [Nyquist-Shannon Sampling Theorem](https://www.google.com/search?q=Nyquist-Shannon+Sampling+Theorem)
> - [Quantization in Signal Processing](https://www.google.com/search?q=Quantization+in+Signal+Processing)
> - [Types of ADCs](https://www.google.com/search?q=Types+of+ADCs)
> - [Digital Signal Processing](https://www.google.com/search?q=Digital+Signal+Processing)