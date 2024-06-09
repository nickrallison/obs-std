---
bad_links: 
aliases: []
tags: [algorithms]
---
# Horner's Rule

Horner's Rule is a method used to evaluate polynomial functions at a given value of x, and it can also be used to rewrite polynomials. It is named after the British mathematician William George Horner, who popularized this method in the West. However, it's worth noting that this method was known to mathematicians much earlier, including Chinese mathematician Zhu Shijie in the 14th century.

The rule is based on the observation that every polynomial of degree n can be written in "nested" form:

$$
P(x) = a_nx^n + a_{n-1}x^{n-1} + \ldots + a_2x^2 + a_1x + a_0
$$

can be rewritten as:

$$
P(x) = a_nx(x(x(\ldots (x(x + a_{n-1}) + a_{n-2}) \ldots ) + a_1) + a_0
$$

This nested form is also known as Horner's form.

The process of evaluating a polynomial at a specific value of x using Horner's Rule is as follows:

1. Set $b_n = a_n$
2. For $i = n-1$ down to $0$, set $b_i = a_i + b_{i+1} * x$
3. The result is $b_0$

This process reduces the number of multiplications and additions needed to evaluate the polynomial, making it more efficient than straightforward evaluation.

Horner's Rule can also be used to perform polynomial division and to find the roots of a polynomial. The process is similar to the evaluation process, but with additional steps to handle the division or root-finding.

For a more in-depth understanding of Horner's Rule, you may want to look into the following resources:

> - ["Horner's Method" on Wolfram MathWorld](https://mathworld.wolfram.com/HornersMethod.html)
> - ["Horner's Scheme" on Encyclopedia of Mathematics](https://encyclopediaofmath.org/wiki/Horner_scheme)
> - ["Horner's Rule for Polynomial Evaluation" on GeeksforGeeks](https://www.geeksforgeeks.org/horners-rule-polynomial-evaluation/)
> - ["Horner's Method" on Brilliant.org](https://brilliant.org/wiki/horners-method/)