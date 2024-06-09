---
bad_links: 
aliases: ["Taylor Polynomial", "Taylor Approximation"]
tags: [calculus]
---
# Taylor Series

The Taylor Series is a mathematical concept used to represent or approximate functions. It's a series expansion of a function about a point. The series, named after British mathematician Brook Taylor, expresses a function as the infinite sum of terms calculated from the function's derivatives at a single point. It's key in fields like physics and engineering for solving [[Differential Equations|differential equations]] and approximating complex functions. The more terms used in the series, the more accurate the approximation becomes.

Sure, here is the general formula for a Taylor Series expansion:

$$
f(x) = f(a) + f'(a)(x-a) + \frac{f''(a)(x-a)^2}{2!} + \frac{f'''(a)(x-a)^3}{3!} + \cdots
$$

Or in the more compact sigma notation:

$$
f(x) = \sum_{n=0}^{\infty}\frac{f^{(n)}(a)}{n!}(x-a)^n
$$

where $f^{(n)}(a)$ denotes the nth derivative of the function evaluated at point $a$, and $(x-a)^n$ is the nth power of $(x - a)$.