---
bad_links: 
aliases: []
tags: [algorithms, geometry]
---
# Gift Wrapping Algorithm
**Expert**: Computer Scientist specializing in Computational Geometry  
**Objective**: To provide a comprehensive explanation of the Gift Wrapping Algorithm, including any tangentially related items, relevant formulas, derivations, and proofs.  
**Assumptions**: You are seeking an in-depth understanding of the Gift Wrapping Algorithm, a fundamental algorithm in computational geometry. You are comfortable with mathematical notation and concepts, and you are interested in the broader context and implications of the algorithm.

The Gift Wrapping Algorithm, also known as the Jarvis March, is a method used in computational geometry to find the convex hull of a set of points. The convex hull can be visualized as the shape formed by stretching a rubber band around the outermost points in a set.

The algorithm works as follows:

1. Start at the leftmost point (the point with the smallest x-coordinate) of the dataset. This point is guaranteed to be a part of the convex hull.
2. From the current point, consider all other points in the dataset and examine the angle each point makes with the current point. Choose the point that makes the smallest positive angle (or the largest negative angle) with the current point. This point becomes the new current point.
3. Repeat step 2 until the algorithm returns to the starting point.

The pseudocode for the algorithm is as follows:

```
function giftWrapping(points)
    n = length(points)
    hull = new Array()

    // Find leftmost point
    l = 0
    for i from 1 to n
        if points[i].x < points[l].x
            l = i

    p = l
    q

    // Iterate until p becomes l again
    do
        hull.add(points[p])

        q = (p + 1) % n
        for i from 0 to n
            // If i is more counterclockwise than current q, update q
            if orientation(points[p], points[i], points[q]) == 2
                q = i

        p = q

    while p != l

    return hull
```

The `orientation` function used in the pseudocode is a helper function that finds the orientation of an ordered triplet of points. It returns the following:

- 0 if the points are colinear
- 1 if the points are clockwise
- 2 if the points are counterclockwise

The function is defined as follows:

```
function orientation(p, q, r)
    val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y)

    if val == 0 return 0  // colinear
    if val > 0 return 1   // clockwise
    return 2              // counterclockwise
```

The time complexity of the Gift Wrapping Algorithm is O(nh), where n is the number of input points and h is the number of convex hull vertices. In the worst-case scenario (when all the points are part of the convex hull), the time complexity is O(n^2).

> For more information, you can refer to the following resources:
> - [Gift Wrapping Algorithm - Wikipedia](https://www.google.com/search?q=Gift+Wrapping+Algorithm+Wikipedia)
> - [Convex Hull | Set 1 (Jarvis’s Algorithm or Wrapping) - GeeksforGeeks](https://www.google.com/search?q=Convex+Hull+%7C+Set+1+(Jarvis%E2%80%99s+Algorithm+or+Wrapping)+-+GeeksforGeeks)
> - [Computational Geometry - Cornell University](https://www.google.com/search?q=Computational+Geometry+-+Cornell+University)