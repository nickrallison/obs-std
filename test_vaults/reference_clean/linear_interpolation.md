---
bad_links: 
aliases: [interpolate, interpolation, lerp]
tags: [computergraphics, algorithms]
---
# Linear Interpolation

Linear interpolation is a method of estimating an unknown value between two known values. It is a technique used in mathematics and computing where a straight line is created between two points on a graph, and the value at a certain point along this line is used to estimate an unknown value. This method assumes that there is a linear relationship between these two points. It is often used in statistics, physics, computer graphics and many other fields where it's necessary to estimate values based on existing data.

The formula for linear interpolation between two points $(x_1, y_1)$ and $(x_2, y_2)$ is given by:

$$
\begin{equation*}
y = y_1 + \frac{y_2 - y_1}{x_2 - x_1}(x - x_1)
\end{equation*}
$$