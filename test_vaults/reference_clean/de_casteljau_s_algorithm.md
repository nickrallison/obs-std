---
bad_links: 
aliases: []
tags: [computergraphics, algorithms, imageprocessing]
---
# De Casteljau's Algorithm

De Casteljau's Algorithm is a recursive method used to evaluate points on a Bezier curve. It allows us to compute the position of a point on the curve at a given parameter value. The algorithm works by repeatedly subdividing the control polygon of the Bezier curve until we reach a single point.

Let's consider a Bezier curve defined by control points $P_0, P_1, P_2, \ldots, P_n$. The algorithm can be summarized as follows:

1. Start with the original control polygon.
2. For each iteration, create a new set of control points by interpolating between adjacent control points of the previous iteration.
3. Repeat step 2 until only one point remains.

To interpolate between two control points $P_i$ and $P_{i+1}$ at a given parameter value $t$, we use the following formula:

$$
Q_i(t) = (1-t)P_i + tP_{i+1}
$$

This formula calculates a new point $Q_i(t)$ that lies on the line segment connecting $P_i$ and $P_{i+1}$.

By applying this formula recursively, we can compute the position of a point on the Bezier curve at any parameter value. Here's the step-by-step process:

1. Start with the original control polygon.
2. For each iteration, compute the new set of control points by interpolating between adjacent control points using the formula $Q_i(t) = (1-t)P_i + tP_{i+1}$.
3. Repeat step 2 until only one point remains. This final point is the desired point on the Bezier curve.

The algorithm can be visualized as follows:

1. Start with the original control polygon.
2. For each iteration, compute the new set of control points by interpolating between adjacent control points.
3. Repeat step 2 until only one point remains. This final point is the desired point on the Bezier curve.

De Casteljau's Algorithm is based on the concept of de Casteljau's theorem, which states that the point on a Bezier curve at a given parameter value can be obtained by interpolating between adjacent control points. The algorithm provides a systematic way to apply this theorem and evaluate points on the curve.

I hope this explanation helps! If you'd like to dive deeper into the topic, here are some additional resources:

> - [Bezier Curves - Wikipedia](https://en.wikipedia.org/wiki/B%C3%A9zier_curve)
> - [De Casteljau's Algorithm - GeeksforGeeks](https://www.geeksforgeeks.org/de-casteljaus-algorithm-for-computing-bezier-curve/)
> - [Interactive De Casteljau's Algorithm Visualization](https://www.jasondavies.com/animated-bezier/)