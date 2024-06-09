---
bad_links: 
aliases: [Euclidean algorithm, euclids algorithm]
tags: [algorithms, computerscience, numbertheory]
title: Euclid's Algorithm
date created: Friday, July 14th 2023, 9:52:17 am
---
# Euclid's Algorithm

Euclid's Algorithm is a method for computing the [[Greatest Common Divisor|greatest common divisor]] ([[Greatest Common Divisor|GCD]]) of two numbers, which is the largest number that divides both of them without leaving a remainder. It is one of the oldest algorithms in common use, having been included in Euclid's "Elements" around 300 BC.

The algorithm is based on the principle that the [[Greatest Common Divisor|greatest common divisor]] of two numbers does not change if the larger number is replaced by its difference with the smaller number. Starting with two positive integers, a and b (where a > b), the algorithm proceeds by repeatedly replacing a with a - b until a equals b, at which point b is the [[Greatest Common Divisor|GCD]] of the original two integers.

The algorithm can be written in pseudocode as follows:

```
function gcd(a, b)
    while a ≠ b
        if a > b
            a := a - b
        else
            b := b - a
    return a
```

However, this version of the algorithm is not efficient for large numbers. A more efficient version, known as the Euclidean division version, replaces the difference of the two numbers with the remainder of the Euclidean division (also called division with remainder). The Euclidean division of a and b yields a quotient q and a remainder r such that a = bq + r. The algorithm then replaces (a, b) with (b, r) repeatedly until the pair is (d, 0), where d is the [[Greatest Common Divisor|GCD]]. The pseudocode for this version is:

```
function gcd(a, b)
    while b ≠ 0
        t := b
        b := a mod b
        a := t
    return a
```

The Euclidean Algorithm is efficient because it reduces the size of the numbers quickly. The number of steps needed is proportional to the number of digits of the smaller integer.

The algorithm has several important applications in number theory, including in the computation of [[Lowest Common Multiple|least common multiples]], in modular arithmetic, and in the [[RSA Encryption|RSA encryption]] algorithm.

> For more context and reading, you can refer to the following resources:
> - [Euclid's Algorithm on Wolfram MathWorld](https://mathworld.wolfram.com/EuclidsAlgorithm.html)
> - [Euclid's Algorithm on Brilliant.org](https://brilliant.org/wiki/euclids-algorithm/)
> - [Euclid's Algorithm on Khan Academy](https://www.khanacademy.org/computing/computer-science/cryptography/modarithmetic/a/the-euclidean-algorithm)