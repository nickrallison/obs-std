---
bad_links: 
aliases: [CSD]
tags: [computerarchitecture]
title: Canonical Signed Digit
date created: Monday, July 24th 2023, 5:58:56 pm
---
# Canonical Signed Digit

Canonical Signed Digit (CSD) is a non-standard positional numeral system which is used in digital signal processing (DSP) for constant multiplications. It is a signed digit representation where each digit can take on the values -1, 0, or +1. The "canonical" part of the name refers to the fact that every number has a unique representation in CSD.

The primary advantage of CSD is that it minimizes the number of non-zero digits, which in turn reduces the complexity of the multiplication operation. This is because multiplication by zero or ±1 is much simpler than multiplication by other numbers.

The CSD representation of a number can be obtained from its [[Binary Representation|binary representation]] by applying the following transformation rules:

1. Whenever there are two consecutive '1's in the [[Binary Representation|binary representation]], replace the least significant '1' and the next higher '1' with '0', and if the next higher bit is '0', replace it with '+1'. If it is '1', replace it with '-1' and continue the transformation to the next higher bit.

2. If there are three consecutive '1's, replace the least significant '1' and the next higher '1' with '0', and replace the next higher bit with '+1' regardless of its current value.

The transformation can be represented mathematically as follows:

Let $b_i$ be the $i$-th bit of the [[Binary Representation|binary representation]], and $c_i$ be the $i$-th digit of the CSD representation. Then,

$$
c_i = b_i - b_{i+1}b_{i+2}
$$

and

$$
b_{i+1}' = b_{i+1} + b_{i+1}b_{i+2}
$$

where $b_{i+1}'$ is the $(i+1)$-th bit after transformation.

The CSD representation has at most $n+1$ non-zero digits, where $n$ is the number of bits in the [[Binary Representation|binary representation]]. This is a significant reduction compared to the [[Binary Representation|binary representation]], which can have up to $n$ non-zero digits.

> For more information, you can refer to the following resources:
> - [Canonical Signed Digit - Wikipedia](https://www.google.com/search?q=Canonical+Signed+Digit+Wikipedia)
> - [Efficient Implementations of Multiplication on DSPs using Canonical Signed Digits](https://www.google.com/search?q=Efficient+Implementations+of+Multiplication+on+DSPs+using+Canonical+Signed+Digits)
> - [A Comparative Study of the Canonical Signed Digit Number System](https://www.google.com/search?q=A+Comparative+Study+of+the+Canonical+Signed+Digit+Number+System)