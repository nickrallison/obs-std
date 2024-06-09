---
bad_links: 
aliases: []
tags: [calculus]
---
# Pade Approximation

Pade Approximation is a method used in mathematical calculations to approximate functions. It's a type of rational function approximation, which means it approximates a function by dividing one polynomial by another. This method is often superior to other approximations like power series, especially for complex functions or those with singularities. The Pade Approximation can be more accurate and cover a larger range of values than other methods. Its applications include physics, engineering and computer science among others, where it helps to simplify complex calculations.

The general Pade Approximation of order (m,n) for a function $f(x)$ can be written as:

$$
f(x) \approx P_{m,n}(x) = \frac{\sum_{j=0}^{m} a_j x^j}{1 + \sum_{k=1}^{n} b_k x^k}
$$

where $a_j$ and $b_k$ are coefficients determined such that the [[Taylor Series|Taylor series]] expansion of $P_{m,n}(x)$ matches that of $f(x)$ up to the highest possible order.

