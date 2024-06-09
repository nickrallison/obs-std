---
bad_links: 
aliases: []
tags: [coding]
---
# Sympy

SymPy is a Python library used for symbolic mathematics. It aims to become a full-featured computer algebra system (CAS) while keeping the code as simple as possible in order to be comprehensible and easily extensible. SymPy is written entirely in Python and does not require any external libraries.

Symbolic computation deals with the computation of mathematical objects symbolically. This means that the mathematical objects are represented exactly, not approximately, and mathematical expressions with unevaluated variables are left in symbolic form.

Here are some of the capabilities of SymPy:

1. **Basic symbolic arithmetic**: Includes differentiation, expansion, simplification, polynomials, etc. For example, SymPy uses the powerful and efficient polynomial manipulation algorithms to provide functions like expand and simplify. The expand function will expand a polynomial expression, while the simplify function will attempt to simplify a complex expression into a simpler form.

2. **Calculus**: Includes [[Limits|limits]], differentiation, integration, multivariable calculus, series expansion, etc. For example, the diff function can be used to compute the derivative of any expression, and the integrate function can be used to compute the integral of an expression.

3. **Discrete math**: Includes permutations, combinations, discrete probability, etc.

4. **Algebra**: Includes solving equations, systems of equations, [[Differential Equations|differential equations]], etc.

5. **Geometry**: Includes points, lines, rays, circles, ellipses, parabolas, polygons, etc.

6. **Plotting**: SymPy can plot expressions and geometrical entities.

7. **Physics**: Includes mechanics, quantum mechanics, optics, thermodynamics, etc.

8. **Statistics**: Includes [[Random Variable|random variables]], [[Probability Density Function|probability density functions]], etc.

Here is an example of how SymPy can be used to solve a simple algebraic equation:

```python
from sympy import symbols, Eq, solve

x = symbols('x')
eq = Eq(x**2 - 1, 0)
sol = solve(eq, x)
print(sol)
```

This will output `[-1, 1]`, which are the solutions to the equation $x^2 - 1 = 0$.

Sympy can print to $\LaTeX$ by using the following code snippet 
```python

from sympy import symbols, latex

x, y = symbols('x y')
expr = x**2 + 2*y

latex_expr = latex(expr)

print(latex_expr)
```

This will print the Latex version of the expression: $x^{2} + 2 y$.