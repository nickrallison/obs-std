---
aliases: []
tags: [numbertheory]
bad_links: [Chinese Remainder Theorem.md]
---
# Remainder Theorem

The Remainder Theorem is a fundamental concept in algebra, particularly in the study of polynomial functions. It provides a method for finding the remainder of a polynomial division without having to perform the actual division.

The theorem states that if a polynomial $f(x)$ is divided by a linear divisor of the form $x - a$, the remainder is $f(a)$. In other words, if you substitute $x = a$ into the polynomial, the result is the remainder of the division.

Mathematically, this can be expressed as:

$$
f(x) = (x - a)q(x) + r
$$

where:
- $f(x)$ is the polynomial being divided,
- $x - a$ is the divisor,
- $q(x)$ is the quotient, and
- $r$ is the remainder.

If $r = 0$, it means that $x - a$ is a factor of $f(x)$.

The Remainder Theorem is a specific case of the Factor Theorem, which states that a polynomial $f(x)$ has a factor $(x - a)$ if and only if $f(a) = 0$.

The proof of the Remainder Theorem is straightforward and relies on polynomial division. Here is a sketch of the proof:

1. Assume that $f(x)$ is a polynomial and $x - a$ is a linear divisor.
2. By the process of polynomial division, we can express $f(x)$ as $(x - a)q(x) + r$, where $q(x)$ is the quotient and $r$ is the remainder.
3. The remainder $r$ must be a constant because the degree of the remainder is always less than the degree of the divisor, and the divisor $x - a$ is of degree 1.
4. If we substitute $x = a$ into the equation, we get $f(a) = (a - a)q(a) + r = r$.
5. Therefore, the remainder of the division of $f(x)$ by $x - a$ is $f(a)$.

> For further reading, you might find the following resources helpful:
> - [Remainder Theorem - Wikipedia](https://www.google.com/search?q=Remainder+Theorem+Wikipedia)
> - [Factor Theorem - Wikipedia](https://www.google.com/search?q=Factor+Theorem+Wikipedia)
> - [Polynomial Division - Wikipedia](https://www.google.com/search?q=Polynomial+Division+Wikipedia)