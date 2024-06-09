---
bad_links: 
aliases: []
tags: [algorithms, geometry]
---
# Graham's Scan
**Expert**: Computer Scientist specializing in Computational Geometry  
**Objective**: To provide a comprehensive explanation of Graham's Scan algorithm, including related concepts, relevant formulas, derivations, and proofs.  
**Assumptions**: You are seeking an in-depth understanding of the Graham's Scan algorithm, a fundamental concept in computational geometry. You are comfortable with mathematical notation and computer science terminology.

Graham's Scan is an algorithm for finding the [[Convex Hull|convex hull]] of a finite set of points in the plane with [[Big-O Notation|time complexity]] $O(n \log n)$. The [[Convex Hull|convex hull]] is the smallest convex polygon that contains all the points of the set.

The algorithm was published by Ronald Graham in 1972. It is one of the simplest and most efficient algorithms for this problem.

Here are the steps of the algorithm:

1. **Find the point with the lowest y-coordinate**: If the lowest y-coordinate exists in more than one point in the set, the point with the lowest x-coordinate out of these is chosen. This point is the starting point (let's call it $P$) and it is guaranteed to be on the [[Convex Hull|convex hull]].

2. **Sort the points**: All remaining points are sorted by the angle they and the point $P$ make with the x-axis. This step is necessary to traverse the points in counterclockwise order around $P$. If two points have the same angle, the point closest to $P$ comes first.

3. **Iterate over the sorted points**: Starting from the point $P$, iterate over the sorted points and for each point, check if moving from the two previous points to this point is a "right turn" or "left turn". If it's a "right turn", this means that the second-to-last point is not part of the [[Convex Hull|convex hull]] and should be removed from the hull. This step is repeated until we are sure that the second-to-last point is part of the [[Convex Hull|convex hull]], then we move on to the next point.

4. **End condition**: The algorithm ends when it gets back to the starting point $P$.

The "right turn" or "left turn" can be determined by the sign of the [[Cross Product|cross product]] of the vectors formed by the points. If we have three points $(x_1, y_1)$, $(x_2, y_2)$, and $(x_3, y_3)$, the [[Cross Product|cross product]] is calculated as follows:

$$
(x_2 - x_1) * (y_3 - y_1) - (y_2 - y_1) * (x_3 - x_1)
$$

If the result is less than 0, it's a "right turn". If it's more than 0, it's a "left turn". If it's 0, the points are collinear.

> For more context and reading, you can refer to the following resources:
> - [Graham's Scan Algorithm](https://en.wikipedia.org/wiki/Graham_scan)
> - [Convex Hull](https://en.wikipedia.org/wiki/Convex_hull)
> - [Cross Product](https://en.wikipedia.org/wiki/Cross_product)