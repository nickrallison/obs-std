---
bad_links: 
aliases: [relatively prime, mutually prime, Coprime]
tags: [numbertheory]
---
# Coprimality

Coprime numbers, also known as relatively prime or mutually prime numbers, are integers that have no common positive integer divisors other than 1. In other words, two numbers are coprime if their greatest common divisor (GCD) is 1.

The GCD of two numbers, denoted as GCD(a, b), is the largest positive integer that divides both a and b without leaving a remainder. If the GCD of two numbers is 1, then they are coprime.

For example, let's consider the numbers 15 and 28. The divisors of 15 are 1, 3, 5, and 15, while the divisors of 28 are 1, 2, 4, 7, 14, and 28. The only positive integer that divides both 15 and 28 is 1, so they are coprime.

Coprime numbers have several interesting properties and applications in number theory. Here are a few:

1. **[[Euler's Totient Function.md|Euler’s Totient Function]]**: The totient function, denoted as φ(n), gives the count of positive integers less than or equal to n that are coprime to n. For example, φ(10) = 4, as there are four numbers (1, 3, 7, and 9) that are coprime to 10.

2. **Modular Arithmetic**: Coprime numbers play a crucial role in modular arithmetic. If a and b are coprime, then for any positive integer n, the congruence equation ax ≡ b (mod n) has a unique solution modulo n.

3. **RSA Cryptography**: The security of the RSA encryption algorithm relies on the difficulty of factoring large numbers into their prime factors. Coprime numbers are used in the generation of the public and private keys in RSA encryption.

4. **[[Chinese Remainder Theorem.md|Chinese Remainder Theorem]]**: The Chinese Remainder Theorem provides a method to solve a system of congruences when the moduli are pairwise coprime.

To determine if two numbers are coprime, you can use the Euclidean algorithm to find their GCD. If the GCD is 1, the numbers are coprime. The Euclidean algorithm is based on the observation that the GCD of two numbers remains the same if the larger number is replaced by its remainder when divided by the smaller number. The algorithm continues until the remainder is 0, at which point the GCD is found.

Here is the Euclidean algorithm in mathematical notation:

$$
\text{GCD}(a, b) = \begin{cases} 
a & \text{if } b = 0 \\
\text{GCD}(b, a \mod b) & \text{otherwise}
\end{cases}
$$

I hope this explanation helps! If you'd like to explore more about coprime numbers, you can refer to the following resources:

> - [Coprime Numbers - Wikipedia](https://en.wikipedia.org/wiki/Coprime_integers)
> - [Euler's Totient Function - Wikipedia](https://en.wikipedia.org/wiki/Euler%27s_totient_function)
> - [Modular Arithmetic - Wikipedia](https://en.wikipedia.org/wiki/Modular_arithmetic)
> - [RSA Cryptography - Wikipedia](https://en.wikipedia.org/wiki/RSA_(cryptosystem))
> - [Chinese Remainder Theorem - Wikipedia](https://en.wikipedia.org/wiki/Chinese_remainder_theorem)