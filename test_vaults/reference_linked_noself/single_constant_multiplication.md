---
bad_links: 
aliases: [SCM]
tags: [electronics, computerscience]
---
# Single Constant Multiplication

Single Constant Multiplication (SCM) is a method used in digital signal processing that involves the multiplication of a variable by a constant. It is employed in several applications such as [[Impulse Response|Impulse Response]] Filter.md|finite [[Impulse Response|impulse response]] filters]], discrete cosine transforms, and [[Fast Fourier Transform|fast Fourier transforms]]. The aim is to reduce the complexity of calculations and improve the speed and efficiency of data processing. SCM utilizes different optimization techniques for achieving this, such as using shift and add operations instead of direct multiplications.

Finding the optimal SCM is an [[NP-Complete|NP-Complete]] problem. The [[State Space|state space]] must be spanned to find the optimal solution which will often look like the following:

$$
17x = 16x + x = (1 \ll 4)x + x
$$
