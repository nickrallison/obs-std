---
bad_links: 
aliases: []
tags: [algorithms]
---
# Voronio Diagram

Voronoi diagrams, named after the Russian mathematician Georgy Voronoi, are a partitioning of a plane into regions based on distance to points in a specific subset of the plane. Given a set of points in the plane, each point in the plane is associated with the nearest point in the given set.

The formal definition of a Voronoi diagram is as follows: Given a set of distinct points $S = \{p_1, p_2, …, p_n\}$ in the Euclidean plane, the Voronoi cell $R_i$ corresponding to a site $p_i$ is defined by

$$
R_i = \{x \in \mathbb{R}^2 : ||x - p_i|| \leq ||x - p_j||, \forall j \neq i\}
$$

where $||x - p_i||$ denotes the Euclidean distance between $x$ and $p_i$. The Voronoi diagram of $S$ is the subdivision of $\mathbb{R}^2$ induced by the Voronoi cells.

The edges of the Voronoi diagram are all the points in the plane that are equidistant to the nearest two or more sites. The vertices of the Voronoi diagram are the points equidistant to three (or more) sites.

Voronoi diagrams have a wide range of applications in fields such as computer graphics, epidemiology, geophysics, and robotics. They are particularly useful in problems involving spatial relationships, such as nearest neighbor queries.

The construction of Voronoi diagrams can be achieved through several algorithms, the most common of which is Fortune's algorithm. This is a sweep line algorithm, which means it processes the points in the plane in one direction, adding and removing edges of the Voronoi diagram as it goes.

## Image
![[th-472731952.jpg]]

> For more in-depth understanding, you may want to read the following resources:
> - [Voronoi Diagrams on Wikipedia](https://www.google.com/search?q=Voronoi+Diagrams+Wikipedia)
> - [Fortune's Algorithm on Wikipedia](https://www.google.com/search?q=Fortune%27s+Algorithm+Wikipedia)
> - [Applications of Voronoi Diagrams](https://www.google.com/search?q=Applications+of+Voronoi+Diagrams)
> - [Voronoi Diagrams: Applications from Archaeology to Zoology](https://www.google.com/search?q=Voronoi+Diagrams%3A+Applications+from+Archaeology+to+Zoology)