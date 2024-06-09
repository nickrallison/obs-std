---
bad_links: 
aliases: [laplacian]
tags: [calculus, differentialequations, vectorcalculus]
---
# Laplacian Operator

The Laplacian operator is a second order differential operator in the field of mathematics, specifically in the area of vector calculus. It is denoted by the symbol ∇². The Laplacian operator represents the [[Divergence|divergence]] of the [[Gradient|gradient]] of a scalar function. It is used to calculate the Laplacian of a function, which gives a measure of how much the function deviates from being harmonic. The operator is widely used in physics and engineering to describe wave propagation, fluid dynamics, electromagnetism, and other phenomena.

Here is the Laplacian operator in Cartesian coordinates:

$$
\begin{gather*} 
\nabla^2 f = \frac{\partial^2 f}{\partial x^2} + \frac{\partial^2 f}{\partial y^2} + \frac{\partial^2 f}{\partial z^2}
\end{gather*}
$$

And in spherical coordinates:

$$
\begin{gather*} 
\nabla^2 f = \frac{1}{r^2}\frac{\partial }{\partial r}\left(r^2\frac{\partial f}{\partial r}\right) + \frac{1}{r^2sin(\theta)}\frac{\partial }{\partial \theta}\left(sin(\theta)\frac{\partial f}{\partial \theta}\right) + \frac{1}{r^2sin(\theta)^2}\frac{\partial ^2f}{\partial \phi ^2}
\end{gather*}
$$