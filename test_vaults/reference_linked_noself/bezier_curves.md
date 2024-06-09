---
bad_links: 
aliases: [bezier, bezier curve]
tags: [imageprocessing, computergraphics]
---
# Bezier Curves

Bezier curves are a type of [[Vector Valued Function|Vector Valued Function]] commonly used in computer graphics and geometric modeling. They are named after Pierre Bézier, a French engineer who developed the mathematical framework for these curves in the 1960s.

## Definition and Formula
A Bezier curve is defined by a set of control points. The curve is determined by interpolating or approximating these control points. The simplest form of a Bezier curve is a quadratic Bezier curve, which is defined by three control points: P0, P1, and P2.

The formula for a quadratic Bezier curve is:

$$
B(t) = (1 - t)^2 \cdot P0 + 2(1 - t) \cdot t \cdot P1 + t^2 \cdot P2
$$

where:
- $t$ is a parameter that varies between 0 and 1, representing the position along the curve.
- $P0$, $P1$, and $P2$ are the control points.

## Tangentially Related Items
1. **Degree of a Bezier Curve**: The degree of a Bezier curve is one less than the number of control points. For example, a quadratic Bezier curve has a degree of 2.
2. **[[Convex Hull|Convex Hull]] Property**: Bezier curves lie within the [[Convex Hull|convex hull]] of their control points. This property ensures that the curve remains within the region defined by the control points.
3. **[[De Casteljau's Algorithm|De Casteljau's Algorithm]]**: [[De Casteljau's Algorithm|De Casteljau's algorithm]] is a recursive method for evaluating Bezier curves. It involves splitting the curve into smaller segments and interpolating between control points.

## Derivation and Proof
The derivation of the formula for a quadratic Bezier curve involves using the concept of [[Linear Interpolation|interpolation]]. By setting up equations to satisfy the [[Linear Interpolation|interpolation]] conditions at the endpoints and midpoint, we can solve for the coefficients in the formula.

The proof of the formula involves expanding the equation and simplifying it using algebraic manipulations. By substituting the values of $P0$, $P1$, and $P2$ into the formula and simplifying, we can show that the resulting expression satisfies the [[Linear Interpolation|interpolation]] conditions.

For a more detailed derivation and proof, you can refer to the following resources:
- [Derivation of Quadratic Bezier Curve Formula](https://pages.mtu.edu/~shene/COURSES/cs3621/NOTES/spline/Bezier/bezier-der.html)
- [Proof of Quadratic Bezier Curve Formula](https://pages.mtu.edu/~shene/COURSES/cs3621/NOTES/spline/Bezier/bezier-proof.html)

> For further reading on Bezier curves and related topics, you may find the following resources helpful:
> - [Wikipedia - Bezier Curve](https://en.wikipedia.org/wiki/B%C3%A9zier_curve)
> - [MathWorld - Bezier Curve](https://mathworld.wolfram.com/BezierCurve.html)
> - [Geometric Modeling: Bézier Curves](https://www.cs.mtu.edu/~shene/COURSES/cs3621/NOTES/spline/Bezier/bezier-der.html)