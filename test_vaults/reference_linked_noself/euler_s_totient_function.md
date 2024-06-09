---
bad_links: 
aliases: [Eulers Totient Function, totient function]
tags: [numbertheory]
---
# [[Leonhard Euler|Euler’s]] Totient Function

[[Leonhard Euler|Euler's]] Totient Function, denoted as $\phi(n)$, is a fundamental concept in number theory that counts the number of positive integers less than or equal to $n$ that are [[Coprimality|coprime]] ([[Coprimality|relatively prime]]) to $n$. In other words, it calculates the count of positive integers less than or equal to $n$ that do not share any common factors with $n$ except for 1.

The formula to calculate [[Leonhard Euler|Euler's]] Totient Function is given by:

$$
\phi(n) = n \prod_{p|n} \left(1 - \frac{1}{p}\right)
$$

where $p$ represents the prime factors of $n$. The product is taken over all distinct prime factors of $n$.

To understand the derivation of this formula, let's consider a positive integer $n$ and its [[Prime Factorization|prime factorization]]:

$$
n = p_1^{a_1} \cdot p_2^{a_2} \cdot p_3^{a_3} \cdot \ldots \cdot p_k^{a_k}
$$

where $p_1, p_2, \ldots, p_k$ are distinct prime numbers and $a_1, a_2, \ldots, a_k$ are positive integers representing the powers of the respective prime factors.

Now, let's consider a positive integer $m$ less than or equal to $n$. For $m$ to be [[Coprimality|coprime]] to $n$, it should not have any prime factors in common with $n$. Therefore, $m$ can be written as:

$$
m = p_1^{b_1} \cdot p_2^{b_2} \cdot p_3^{b_3} \cdot \ldots \cdot p_k^{b_k}
$$

where $0 \leq b_i \leq a_i$ for $1 \leq i \leq k$. In other words, each prime factor $p_i$ of $n$ can appear with any power $b_i$ between 0 and $a_i$ (inclusive) in $m$.

The number of choices for each $b_i$ is $a_i + 1$ (including 0). Therefore, the total number of positive integers less than or equal to $n$ that are [[Coprimality|coprime]] to $n$ is given by:

$$
\phi(n) = (a_1 + 1) \cdot (a_2 + 1) \cdot (a_3 + 1) \cdot \ldots \cdot (a_k + 1)
$$

Substituting the values of $a_i$ from the [[Prime Factorization|prime factorization]] of $n$, we get:

$$
\phi(n) = p_1^{a_1} \cdot p_2^{a_2} \cdot p_3^{a_3} \cdot \ldots \cdot p_k^{a_k} \cdot \left(1 - \frac{1}{p_1}\right) \cdot \left(1 - \frac{1}{p_2}\right) \cdot \left(1 - \frac{1}{p_3}\right) \cdot \ldots \cdot \left(1 - \frac{1}{p_k}\right)
$$

Simplifying this expression gives us the formula for [[Leonhard Euler|Euler's]] Totient Function:

$$
\phi(n) = n \prod_{p|n} \left(1 - \frac{1}{p}\right)
$$

where the product is taken over all distinct prime factors of $n$.

[[Leonhard Euler|Euler's]] Totient Function has several important properties and applications in number theory and cryptography. It is used in various algorithms, such as the [[RSA Encryption|RSA encryption]] algorithm, and plays a crucial role in the study of modular arithmetic and group theory.

For further reading, you may find the following resources helpful:
- [Wikipedia - Euler's Totient Function](https://en.wikipedia.org/wiki/Euler%27s_totient_function)
- [Brilliant - Euler's Totient Function](https://brilliant.org/wiki/eulers-totient-function/)
- [Number Theory Web - Euler's Totient Function](https://numbertheory.org/ntw/N3.html)

> "The totient function is one of the most important functions in number theory." - Carl Friedrich Gauss