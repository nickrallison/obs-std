---
bad_links: 
aliases: [Binet Formula]
tags: [linearalgebra, algorithms]
---
# Binet's Formula

Binet's Formula is a mathematical expression used to calculate the nth term in the Fibonacci sequence without having to compute the preceding terms. The Fibonacci sequence is a series of numbers where each number is the sum of the two preceding ones, usually starting with 0 and 1. Binet's Formula, named after French mathematician Jacques Philippe Marie Binet, provides a direct solution using roots of a quadratic equation, allowing for efficient computation of Fibonacci numbers.

$$
F(n) = \frac{((1 + \sqrt{5}) / 2)^n - ((1 - \sqrt{5}) / 2)^n}{\sqrt{5}}
$$

This closed form formula can also be found from a [[Difference Equations|recurrence equation]] and use a [[Z-Transform|Z-Transform]] to get the closed form:

$$
F(n) = F(n-1) + F(n-2), F(0)=1, F(1)=1
$$

or decomposing a set of matrix equations with an [[Eigendecomposition|Eigendecomposition]] into its [[Diagonalization|diagonalized]]

$$
\begin{bmatrix}   1 & 1 \\   0 & 1    \end{bmatrix} \begin{bmatrix}   F(n)  \\   F(n-1)     \end{bmatrix} = \begin{bmatrix}   F(n+1)  \\   F(n)     \end{bmatrix}
$$

Or written in a clearer way

$$
\begin{bmatrix}   1 & 1 \\   0 & 1    \end{bmatrix}^n \begin{bmatrix}   1  \\   1    \end{bmatrix} = \begin{bmatrix}   F(n+1)  \\   F(n)     \end{bmatrix}
$$