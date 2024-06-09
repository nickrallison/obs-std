---
bad_links: 
aliases: [LCM, Least Common Multiple]
tags: [numbertheory]
---
# Lowest Common Multiple

The Lowest Common Multiple (LCM) of two or more integers is the smallest positive integer that is divisible by each of the given integers without leaving a remainder. In other words, it is the smallest common multiple of the given numbers.

To find the LCM of two numbers, we can use the following formula:

$$
LCM(a, b) = \frac{{|a \cdot b|}}{{\text{GCD}(a, b)}}
$$

where $a$ and $b$ are the given numbers, and $\text{GCD}(a, b)$ represents the Greatest Common Divisor of $a$ and $b$.

The formula for the LCM of more than two numbers can be derived using the LCM of two numbers. Let's say we have three numbers $a$, $b$, and $c$. We can find the LCM of $a$, $b$, and $c$ by finding the LCM of $a$ and $b$, and then finding the LCM of the result with $c$. Mathematically, it can be represented as:

$$
LCM(a, b, c) = LCM(LCM(a, b), c)
$$

This process can be extended to find the LCM of any number of integers.

The LCM is closely related to the concept of prime factorization. To find the LCM of two numbers, we can first express each number as a product of prime factors. Then, the LCM is obtained by taking the highest power of each prime factor that appears in either number.

For example, let's find the LCM of 12 and 18. We can express 12 as $2^2 \cdot 3^1$ and 18 as $2^1 \cdot 3^2$. The LCM is obtained by taking the highest power of each prime factor: $2^2 \cdot 3^2 = 36$. Therefore, the LCM of 12 and 18 is 36.

Proofs related to the LCM involve properties such as divisibility, prime factorization, and the relationship between LCM and GCD. These proofs can be found in various mathematical textbooks and resources.

For further reading and practice, you may find the following resources helpful:

- [LCM and GCD - Brilliant](https://brilliant.org/wiki/least-common-multiple-lcm/)
- [LCM and GCD - Khan Academy](https://www.khanacademy.org/math/pre-algebra/pre-algebra-factors-multiples/pre-algebra-lcm-gcf/a/least-common-multiple)
- [LCM and GCD - Math Is Fun](https://www.mathsisfun.com/least-common-multiple.html)

> I hope this explanation helps! If you have any further questions or need more clarification, feel free to ask.