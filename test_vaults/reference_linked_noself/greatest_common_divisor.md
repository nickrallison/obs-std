---
bad_links: 
aliases: [GCD, Greatest Common Factor, GCF]
tags: [numbertheory]
---
# Greatest Common Divisor

The Greatest Common Divisor (GCD) is a fundamental concept in number theory that represents the largest positive integer that divides two or more integers without leaving a remainder. It is denoted as GCD(a, b), where 'a' and 'b' are the given integers.

There are several methods to find the GCD of two numbers. One common method is the [[Euclid's Algorithm|Euclidean algorithm]], which is based on the observation that the GCD of two numbers is equal to the GCD of the smaller number and the remainder when the larger number is divided by the smaller number. The algorithm can be summarized as follows:

1. Start with two given integers, 'a' and 'b'.
2. Divide 'a' by 'b' and obtain the remainder 'r'.
3. If 'r' is zero, then the GCD is 'b'.
4. If 'r' is not zero, replace 'a' with 'b' and 'b' with 'r', and repeat steps 2 and 3 until 'r' becomes zero.
5. The GCD is the value of 'b' when 'r' becomes zero.

This algorithm can be implemented iteratively or recursively. It is efficient and can find the GCD of two numbers in a relatively short amount of time.

The GCD has several important properties:

1. GCD(a, b) = GCD(b, a): The order of the numbers does not affect the GCD.
2. GCD(a, 0) = a: The GCD of any number and zero is the number itself.
3. GCD(a, a) = a: The GCD of a number with itself is the number itself.
4. GCD(a, b) = GCD(a, b - a): Subtracting one number from the other does not change the GCD.

The GCD is used in various mathematical applications, such as simplifying fractions, finding common factors, and solving linear Diophantine equations.

One related concept is the [[Lowest Common Multiple|Least Common Multiple]] ([[Lowest Common Multiple|LCM]]), which represents the smallest positive integer that is divisible by both 'a' and 'b'. The relationship between GCD and [[Lowest Common Multiple|LCM]] is given by the formula:

LCM(a, b) = (a * b) / GCD(a, b)

This formula can be derived using the [[Prime Factorization|prime factorization]] of the numbers.

For further reading on the topic, you may find the following resources helpful:

- [[Euclid's Algorithm|Euclidean algorithm]]
- [Greatest Common Divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
- [[Lowest Common Multiple|Lowest Common Multiple]]

> "The [[Euclid's Algorithm|Euclidean algorithm]] is a powerful tool for finding the greatest common divisor of two numbers."
