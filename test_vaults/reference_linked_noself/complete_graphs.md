---
bad_links: 
aliases: []
tags: [graphtheory]
---
# Complete Graphs

A complete graph, denoted as $K_n$, is a simple [[Undirected Graph|undirected graph]] in which every pair of distinct vertices is connected by a unique edge. The term "simple" means that there are no loops (edges connected to the same vertex) and no multiple edges between the same pair of vertices. The subscript $n$ represents the number of vertices in the graph.

The number of edges, $E$, in a complete graph can be calculated using the formula:

$$
E = \frac{n(n-1)}{2}
$$

This formula is derived from the fact that each vertex is connected to every other vertex. For the first vertex, there are $(n-1)$ edges, for the second there are $(n-2)$, and so on, until the last vertex which has no new edges. Summing these gives the total number of edges, but since each edge has been counted twice (once for each vertex it connects), we divide by 2.

A complete graph is also a [[Regular Graph|regular graph]], meaning each vertex has the same degree (number of edges incident to it). In a complete graph, the degree of each vertex is $(n-1)$.

A subgraph of a complete graph that forms a cycle (a closed path with no repeated vertices or edges) is called a [[Hamiltonian cycle|Hamiltonian cycle]]. A complete graph with more than 2 vertices always has a [[Hamiltonian cycle|Hamiltonian cycle]].

A complete graph is also a clique, a subset of vertices of an [[Undirected Graph|undirected graph]] such that every two distinct vertices in the clique are adjacent.

> For more information, you can refer to the following resources:
> - [Complete Graph - Wikipedia](https://www.google.com/search?q=Complete+Graph+Wikipedia)
> - [Graph Theory - MathWorld](https://www.google.com/search?q=Graph+Theory+MathWorld)
> - [Hamiltonian Cycle - Wolfram Alpha](https://www.google.com/search?q=Hamiltonian+Cycle+Wolfram+Alpha)
> - [Clique (graph theory) - Wikipedia](https://www.google.com/search?q=Clique+(graph+theory)+Wikipedia)