---
bad_links: 
aliases: []
tags: [numbertheory]
---
# Polynomial Division

Polynomial division is a process similar to long division but with polynomials. It is used to divide a polynomial by another polynomial of the same or lower degree. The result of the division is another polynomial, which is the quotient, and possibly a remainder.

The general form of a polynomial is $P(x) = a_nx^n + a_{n-1}x^{n-1} + … + a_1x + a_0$, where $a_n, a_{n-1}, …, a_1, a_0$ are coefficients and $n$ is the degree of the polynomial.

The process of polynomial division can be broken down into the following steps:

1. Arrange both the dividend and the divisor in descending order of their degrees.
2. Divide the leading term (highest degree) of the dividend by the leading term of the divisor.
3. Multiply the divisor by the result from step 2, and subtract this from the dividend.
4. Bring down the next term from the dividend.
5. Repeat steps 2-4 until you can't bring down any more terms.

The result from step 2 forms the quotient, and the final result from step 3 is the remainder.

For example, let's divide $x^3 - 3x^2 + 2x - 1$ by $x - 1$:

1. Both polynomials are already in descending order.
2. Divide $x^3$ by $x$ to get $x^2$.
3. Multiply $x - 1$ by $x^2$ to get $x^3 - x^2$, and subtract this from $x^3 - 3x^2$ to get $-2x^2$.
4. Bring down $2x$ to get $-2x^2 + 2x$.
5. Repeat steps 2-4: divide $-2x^2$ by $x$ to get $-2x$, multiply $x - 1$ by $-2x$ to get $-2x^2 + 2x$, and subtract this from $-2x^2 + 2x$ to get $0$. Bring down $-1$ to get $-1$.
6. Repeat steps 2-4 one more time: divide $-1$ by $x$ to get $-1$, multiply $x - 1$ by $-1$ to get $-x + 1$, and subtract this from $-1$ to get $0$.

So, $x^3 - 3x^2 + 2x - 1$ divided by $x - 1$ is $x^2 - 2x - 1$ with no remainder.

> For more information, you can refer to the following resources:
> - [Polynomial Division](https://www.google.com/search?q=Polynomial+Division)
> - [Polynomial Long Division](https://www.google.com/search?q=Polynomial+Long+Division)
> - [Polynomial Division: The Remainder Theorem and The Factor Theorem](https://www.google.com/search?q=Polynomial+Division%3A+The+Remainder+Theorem+and+The+Factor+Theorem)
> - [Polynomial Division: Synthetic Division](https://www.google.com/search?q=Polynomial+Division%3A+Synthetic+Division)