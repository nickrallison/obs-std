---
bad_links: 
aliases: [Fundamental Theorem of Arithmetic]
tags: [numbertheory]
---
# The Fundamental Theorem of Arithmetic

The Fundamental Theorem of Arithmetic states that every integer greater than 1 either is a prime number itself or can be represented as the product of prime numbers and that, moreover, this representation is unique, up to (except for) the order of the factors. 

The theorem is fundamental in the sense that the concept of prime number is so basic and important in number theory, and this theorem describes the prime numbers' fundamental property.

The theorem can be formally stated as follows:

For every integer $n > 1$, there exist prime numbers $p_1, p_2, …, p_k$ (which are not necessarily distinct), such that:

$$
n = p_1 \cdot p_2 \cdot ... \cdot p_k
$$

And any two such expressions of $n$ as a product of prime numbers are equivalent up to the order of the factors.

The proof of the Fundamental Theorem of Arithmetic involves two steps: the existence of a prime factorization, and the uniqueness of this factorization.

**Existence**

The existence part can be proven by induction. For the base case, $n=2$ is a prime number, so it is its own prime factorization. For the inductive step, assume that all numbers less than $n$ can be factored into primes. If $n$ is prime, then it is its own prime factorization. If $n$ is composite, then it can be written as the product of two integers $a$ and $b$ (where $2 \leq a, b < n$). By the inductive hypothesis, $a$ and $b$ can be factored into primes, and therefore so can $n$.

**Uniqueness**

The uniqueness part is proven by contradiction. Assume that there are two different factorizations of $n$ into primes. Then there must be a prime that divides one factorization but not the other. But this contradicts the fundamental property of prime numbers, that if a prime divides a product, then it must divide at least one of the factors.

The Fundamental Theorem of Arithmetic has many implications in number theory and other areas of mathematics. For example, it implies that the prime numbers are the "building blocks" of the integers, in the sense that every integer can be built by multiplying prime numbers together in a unique way.

> For more in-depth reading, you may refer to the following resources:
> - [Fundamental Theorem of Arithmetic - Wikipedia](https://www.google.com/search?q=Fundamental+Theorem+of+Arithmetic+site:wikipedia.org)
> - [Proof of the Fundamental Theorem of Arithmetic - Brilliant](https://www.google.com/search?q=Proof+of+the+Fundamental+Theorem+of+Arithmetic+site:brilliant.org)
> - [Applications of the Fundamental Theorem of Arithmetic - Math StackExchange](https://www.google.com/search?q=Applications+of+the+Fundamental+Theorem+of+Arithmetic+site:math.stackexchange.com)