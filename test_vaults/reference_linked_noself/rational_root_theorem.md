---
bad_links: 
aliases: []
tags: [numbertheory]
---
# Rational Root Theorem

The Rational Root Theorem is a useful tool in algebra, particularly when dealing with polynomial equations. It provides a method to identify all possible rational roots of a polynomial equation.

The theorem states that if a polynomial equation with integer coefficients has a rational root, then that root can be expressed as a fraction $p/q$, where $p$ is a factor of the constant term (the "free" term) and $q$ is a factor of the leading coefficient.

Mathematically, if we have a polynomial equation of the form:

$$
a_nx^n + a_{n-1}x^{n-1} + \ldots + a_2x^2 + a_1x + a_0 = 0
$$

where $a_n, a_{n-1}, \ldots, a_2, a_1, a_0$ are integers and $a_n \neq 0$, then any rational root of the equation can be expressed as $p/q$, where $p$ is a factor of $a_0$ and $q$ is a factor of $a_n$.

The Rational Root Theorem does not guarantee that a polynomial equation will have rational roots, but it does provide a finite list of candidates to check.

The proof of the Rational Root Theorem is based on [[The Fundamental Theorem of Arithmetic|the Fundamental Theorem of Arithmetic]], which states that every integer greater than 1 can be uniquely factored into prime numbers. 

If $p/q$ is a root of the polynomial, then $a_n(p/q)^n + a_{n-1}(p/q)^{n-1} + \ldots + a_2(p/q)^2 + a_1(p/q) + a_0 = 0$. Multiplying through by $q^n$ gives $a_np^n + a_{n-1}p^{n-1}q + \ldots + a_2p^2q^{n-2} + a_1pq^{n-1} + a_0q^n = 0$. Since $p$ and $q$ are integers, all terms in this equation are integers. Therefore, $a_np^n$ must be divisible by $q^n$, which implies that $q$ is a factor of $a_n$. Similarly, $a_0q^n$ must be divisible by $p^n$, which implies that $p$ is a factor of $a_0$.

The Rational Root Theorem is closely related to the [[Factor Theorem|Factor Theorem]] and the [[Remainder Theorem|Remainder Theorem]]. The [[Factor Theorem|Factor Theorem]] states that a polynomial $f(x)$ has a factor $(x - p)$ if and only if $f(p) = 0$. The [[Remainder Theorem|Remainder Theorem]] states that the remainder of the division of a polynomial $f(x)$ by a linear divisor $x - p$ is equal to $f(p)$.

> For more context and reading, you can refer to the following resources:
> - [Rational Root Theorem](https://www.google.com/search?q=Rational+Root+Theorem)
> - [Fundamental Theorem of Arithmetic](https://www.google.com/search?q=Fundamental+Theorem+of+Arithmetic)
> - [Factor Theorem](https://www.google.com/search?q=Factor+Theorem)
> - [Remainder Theorem](https://www.google.com/search?q=Remainder+Theorem)