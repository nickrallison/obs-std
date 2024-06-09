---
bad_links: 
aliases: []
tags: [geometry]
---
# Convex Hull

The Convex Hull of a set of points is the smallest convex polygon that contains all the points. In more formal terms, given a set of points $P$, the Convex Hull is the smallest convex set that contains $P$.

To visualize this, imagine each point as a nail sticking out of a board. If you were to stretch a rubber band around all the nails and let it snap into place, the area enclosed by the rubber band would represent the Convex Hull of those points.

There are several algorithms to compute the Convex Hull of a set of points, including the [[Gift Wrapping Algorithm|Gift Wrapping algorithm]], [[Graham's Scan|Graham's Scan]], and the QuickHull algorithm. Each of these algorithms has its own [[Big-O Notation|time complexity]] and trade-offs.

Let's take a closer look at the [[Graham's Scan|Graham's Scan]] algorithm, which is one of the most efficient Convex Hull algorithms with a [[Big-O Notation|time complexity]] of $O(n \log n)$, where $n$ is the number of input points.

**[[Graham's Scan|Graham's Scan]] Algorithm**:

1. Find the point with the lowest y-coordinate, break ties by choosing the point with the lowest x-coordinate. This point is the starting point (call it $P$).
2. Sort the remaining points based on the angle they and $P$ make with the x-axis.
3. Iterate over the sorted points in order and maintain a stack of candidate points for the Convex Hull. For each point, if moving from the two topmost points in the stack to this point is a clockwise turn, pop points from the stack until it's a counter-clockwise turn.
4. The remaining points in the stack form the Convex Hull.

The [[Total Correctness|correctness]] of [[Graham's Scan|Graham's Scan]] follows from the fact that points are processed in increasing order of their angle with the x-axis, and for each point, it removes all points from the Convex Hull that would create a clockwise turn.

> For more details on Convex Hull and its algorithms, you can refer to the following resources:
> - [Convex Hull | Set 1 (Jarvis’s Algorithm or Wrapping)](https://www.google.com/search?q=Convex+Hull+%7C+Set+1+(Jarvis%E2%80%99s+Algorithm+or+Wrapping))
> - [Convex Hull | Set 2 (Graham’s scan algorithm)](https://www.google.com/search?q=Convex+Hull+%7C+Set+2+(Graham%E2%80%99s+scan+algorithm))
> - [QuickHull Algorithm for Convex Hull](https://www.google.com/search?q=QuickHull+Algorithm+for+Convex+Hull)
> - [Convex Hull - Brilliant.org](https://www.google.com/search?q=Convex+Hull+-+Brilliant.org)