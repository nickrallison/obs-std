---
bad_links: 
aliases: []
tags: [numbertheory]
---
# Quadratic Sieve

The Quadratic Sieve (QS) is an algorithm in number theory that is used for integer factorization. It was developed by Carl Pomerance in 1981 and is one of the most efficient methods for factoring large integers. The QS is a general-purpose algorithm, meaning it can factor any composite number (not a prime power or a square), and it is especially effective for numbers with no small factors.

The Quadratic Sieve is based on the principle of Factorization Method.md|Fermat's factorization method]], which states that a number $n$ can be factored if it can be represented as a difference of two squares. That is, if $n = x^2 - y^2 = (x+y)(x-y)$, then $x+y$ and $x-y$ are factors of $n$.

The Quadratic Sieve algorithm follows these general steps:

1. **Polynomial Selection**: Choose a polynomial $f(x) = (x + \sqrt{n})^2 - n$ which is guaranteed to produce a square modulo $n$ for some $x$.

2. **Sieving**: Find many $x$ such that $f(x)$ factors completely over the factor base (a preselected set of small primes and possibly a negative one).

3. **Linear Algebra**: Solve for a nontrivial set of $x$'s such that the product of the corresponding $f(x)$'s is a square. This is done by finding a nontrivial solution to a system of linear equations over the field with two elements.

4. **Square Root**: Compute the square root of the product of the $f(x)$'s modulo $n$ to find a nontrivial square root of 1 modulo $n$.

5. **[[Prime Factorization.md|Factorization]]**: Use the nontrivial square root of 1 modulo $n$ to compute a nontrivial factor of $n$.

The Quadratic Sieve is a significant improvement over previous factorization methods due to its use of smooth numbers (numbers with small prime factors) and the application of linear algebra. The QS has a run-time complexity of $O\left(e^{ \sqrt{\frac{64}{9}} (\ln n)^{\frac{1}{2}} (\ln \ln n)^{\frac{1}{2}}}\right)$, which makes it faster than the continued fraction method and Pollard's $p-1$ method for large numbers.

For a more detailed understanding of the Quadratic Sieve, including the mathematical proofs and derivations, I recommend the following resources:

> - ["The Quadratic Sieve Factoring Algorithm"](https://www.google.com/search?q=The+Quadratic+Sieve+Factoring+Algorithm) by Eric Landquist
> - ["Factoring with the Quadratic Sieve"](https://www.google.com/search?q=Factoring+with+the+Quadratic+Sieve) by Joe Buhler and Peter Stevenhagen
> - ["An Introduction to Quadratic Sieve"](https://www.google.com/search?q=An+Introduction+to+Quadratic+Sieve) by Emmanuel Thomé

Please note that understanding the Quadratic Sieve requires a solid foundation in number theory, algebra, and linear algebra. If you need to brush up on these topics, I recommend the following resources:

> - ["Elementary Number Theory"](https://www.google.com/search?q=Elementary+Number+Theory) by Kenneth H. Rosen
> - ["Linear Algebra Done Right"](https://www.google.com/search?q=Linear+Algebra+Done+Right) by Sheldon Axler
> - ["Introduction to the Theory of Numbers"](https://www.google.com/search?q=Introduction+to+the+Theory+of+Numbers) by G. H. Hardy and E. M. Wright
