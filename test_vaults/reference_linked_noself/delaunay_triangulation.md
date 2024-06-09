---
bad_links: 
aliases: []
tags: [computergraphics, algorithms]
---
# Delaunay Triangulation

Delaunay triangulation is a fundamental algorithm in computational geometry that operates on a set of discrete points to form a triangulation. It is named after Boris Delaunay, a Russian mathematician.

The Delaunay triangulation of a set of points $P$ in the plane is a triangulation DT($P$) such that no point in $P$ is inside the circumcircle of any triangle in DT($P$). The circumcircle of a triangle is the circle that passes through all three vertices of the triangle. This property of Delaunay triangulations is often called the Delaunay property.

The Delaunay triangulation maximizes the minimum angle of all the angles of the triangles in the triangulation, thus avoiding "skinny" triangles. This property makes it useful in mesh generation, where it's important to avoid skinny triangles.

The Delaunay triangulation of a set of points is unique if and only if no four points are cocircular. If four or more points are cocircular, there are multiple valid Delaunay triangulations.

The Delaunay triangulation can be computed using several algorithms, including the [[Divide and Conquer|divide-and-conquer]] algorithm, the sweep-line algorithm, and the incremental algorithm. The [[Big-O Notation|time complexity]] of these algorithms is $O(n \log n)$, where $n$ is the number of points.

The Delaunay triangulation has many applications in science and engineering, including computer graphics, finite element analysis, and geographic [[Information Theory|information]] systems (GIS).

Here is a simple example of a Delaunay triangulation. Given four points $A$, $B$, $C$, and $D$, where $D$ is inside the circumcircle of triangle $ABC$, the Delaunay triangulation of these points is the two triangles $ABD$ and $BCD$, not the triangles $ABC$ and $BCD$.

The Delaunay triangulation is closely related to the Voronoi diagram. The Voronoi diagram of a set of points is the partition of the plane into regions, where each region contains all the points that are closer to one point in the set than to any other. The Delaunay triangulation is the [[Dual Graph|dual graph]] of the Voronoi diagram, which means that there is an edge between two points in the Delaunay triangulation if and only if their Voronoi regions share a boundary.

For more in-depth information, you might want to check out these resources:

> - ["Delaunay Triangulation"](https://en.wikipedia.org/wiki/Delaunay_triangulation) on Wikipedia
> - ["Voronoi Diagram"](https://en.wikipedia.org/wiki/Voronoi_diagram) on Wikipedia
> - ["Computational Geometry"](https://www.google.com/search?q=Computational+Geometry) on Google Search
> - ["Delaunay Triangulation and Voronoi Diagram"](https://www.google.com/search?q=Delaunay+Triangulation+and+Voronoi+Diagram) on Google Search
> - ["Delaunay Triangulation: Algorithms and Applications"](https://www.google.com/search?q=Delaunay+Triangulation%3A+Algorithms+and+Applications) on Google Search

!delaunay_triangle.png