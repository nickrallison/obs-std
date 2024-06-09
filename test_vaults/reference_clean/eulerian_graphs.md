---
bad_links: 
aliases: []
tags: [graphtheory]
---
# Eulerian Graphs

An Eulerian graph is a graph in which all vertices have an even degree. This concept is named after the Swiss mathematician Leonhard Euler, who first introduced it. 

The degree of a vertex in a graph is the number of edges incident to it. In an Eulerian graph, every vertex has an even degree, which means that for every edge that "enters" the vertex, there is another edge that "exits" it. This property allows for the existence of an Eulerian circuit (or Eulerian cycle), which is a closed trail that includes every edge of the graph exactly once.

The concept of Eulerian graphs is closely related to Euler's theorem, which states that a connected graph has an Eulerian circuit if and only if every vertex has an even degree. The theorem can be proven as follows:

**Proof**:

1. If a graph has an Eulerian circuit, then each time a circuit enters a vertex along one edge, it must leave the vertex along another edge. Therefore, the degree of every vertex must be even.

2. Conversely, if a graph is connected and the degree of every vertex is even, then we can construct an Eulerian circuit as follows: Start at any vertex, and keep tracing along edges of the graph, never retracing any edge. Since the graph is connected and has an even degree at every vertex, you will eventually return to the starting vertex, having traced an Eulerian circuit.

The formula for the degree of a vertex is given by:

$$
d(v) = \sum_{u \in V} a_{uv}
$$

where $d(v)$ is the degree of vertex $v$, $V$ is the set of all vertices in the graph, and $a_{uv}$ is the entry in the adjacency matrix $A$ for the graph at row $u$ and column $v$. In an Eulerian graph, $d(v)$ is even for all $v \in V$.

A related concept is the Eulerian path, which is a trail in a graph which visits every edge exactly once. Unlike an Eulerian circuit, an Eulerian path does not need to start and end at the same vertex. A connected graph has an Eulerian path if and only if it has exactly 0 or 2 vertices of odd degree.

> For more information, you can refer to the following resources:
> - [Eulerian Graphs](https://www.google.com/search?q=Eulerian+Graphs)
> - [Euler's Theorem in Graph Theory](https://www.google.com/search?q=Euler%27s+Theorem+in+Graph+Theory)
> - [Adjacency Matrix in Graph Theory](https://www.google.com/search?q=Adjacency+Matrix+in+Graph+Theory)
> - [Degree of a Vertex in Graph Theory](https://www.google.com/search?q=Degree+of+a+Vertex+in+Graph+Theory)
> - [Eulerian Path](https://www.google.com/search?q=Eulerian+Path)