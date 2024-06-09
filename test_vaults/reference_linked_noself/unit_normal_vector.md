---
bad_links: 
aliases: [normal vector]
tags: [calculus, linearalgebra, vectorcalculus]
---
# Unit Normal Vector

A unit normal vector, in the context of [[Vector Valued Function|vector valued functions]], is a vector that is perpendicular (or "normal") to a function at a given point and has a length (or "magnitude") of one. This concept is often used in calculus and physics to describe directions and magnitudes of forces, [[Gradient|gradients]], and other vector quantities. The unit normal vector is typically found by first calculating the [[Tangent Vector|tangent vector]] to the function at the given point, then finding a vector that is orthogonal to the [[Tangent Vector|tangent vector]], and finally normalizing this vector to have a length of one.

Sure, here are the relevant formulas:

$$
\begin{gather*} 
\overrightarrow{N}(t) = \frac{d^2\overrightarrow{r}}{dt^2}
\end{gather*}
$$

We need to normalize it by dividing its magnitude. The unit normal vector is thus given by:

$$
\begin{gather*} 
\hat{n} = \frac{\overrightarrow{N}}{\left| \overrightarrow {N} \right|}
\end{gather*}
$$

