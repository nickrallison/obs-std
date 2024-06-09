---
bad_links: 
aliases: [Eulerian circuit, Eulerian cycle]
tags: [graphtheory]
title: Euler Circuit
date created: Saturday, July 15th 2023, 4:57:49 pm
---
# Euler Circuit
**Expert**: Mathematician  
**Objective**: To provide a comprehensive explanation of Euler circuits, including related concepts, formulas, derivations, and proofs where applicable.  
**Assumptions**: You have a basic understanding of graph theory and are interested in a detailed explanation of Euler circuits.

An Euler circuit in a graph is a route that passes through every edge exactly once and returns to its starting vertex. This concept is named after the Swiss mathematician Leonhard Euler, who first introduced it in the 18th century.

The existence of an Euler circuit in a graph is determined by the degree of its vertices. In an undirected graph, an Euler circuit exists if and only if the degree of every vertex is even. This is known as the Euler Circuit Theorem.

The degree of a vertex is the number of edges incident to it. In an undirected graph, each edge contributes two to the total degree count (one for each vertex it connects), so the sum of the degrees of all vertices in a graph is always even. If a graph has more than two vertices of odd degree, it cannot have an Euler circuit because each time a route enters a vertex by one edge, it must leave by another edge.

The formal proof of the Euler Circuit Theorem is beyond the scope of this explanation, but it essentially involves showing that a graph with the stated properties can be decomposed into cycles that can be combined to form an Euler circuit.

The Fleury's Algorithm is a procedure to construct an Euler circuit (or Euler path if exists) in a graph. The algorithm is as follows:

1. Start with any vertex in the graph.
2. Choose the next edge in the path to be one whose deletion would not disconnect the graph, if possible.
3. Delete the chosen edge from the graph.
4. Repeat steps 2 and 3 until all edges have been deleted.

The time complexity of Fleury's algorithm is $O(E^2)$, where $E$ is the number of edges in the graph.

> For more information, you can refer to the following resources:
> - [Euler Circuit and Path](https://www.google.com/search?q=Euler+Circuit+and+Path)
> - [Fleury's Algorithm](https://www.google.com/search?q=Fleury%27s+Algorithm)
> - [Graph Theory](https://www.google.com/search?q=Graph+Theory)