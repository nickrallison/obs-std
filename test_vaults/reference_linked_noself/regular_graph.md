---
bad_links: 
aliases: []
tags: [graphtheory]
---
# Regular Graph

A **Regular Graph** is a graph where each vertex has the same degree. The degree of a vertex in a graph is the number of edges incident to it. In a regular graph, every vertex has the same number of neighbors; i.e., every vertex has the same degree or valency. 

For example, in a regular graph of degree 3, every vertex is connected to exactly three other vertices. 

The formal definition of a regular graph is as follows:

A graph $G = (V, E)$ is said to be regular, of degree $n$, if all its vertices have the same degree $n$. 

Here, $V$ is the set of vertices and $E$ is the set of edges.

Regular graphs can be further classified into two types:

1. **k-Regular Graph**: A graph is said to be k-regular if the degree of each vertex in the graph is k.

2. **Regular Digraph**: A [[Directed Graph|directed graph]] is regular or k-regular if the indegree and outdegree of each vertex is k.

**[[Handshake Lemma|Handshaking Lemma]]**: This is a fundamental concept related to regular graphs. It states that the sum of the degrees of the vertices of a graph G equals twice the number of edges. Mathematically, it can be represented as:

$$
\sum_{v \in V} deg(v) = 2|E|
$$

where $deg(v)$ is the degree of vertex $v$, $V$ is the set of vertices, and $E$ is the set of edges.

**Proof of [[Handshake Lemma|Handshaking Lemma]]**: 

Consider an edge $e$ in the graph $G$. This edge $e$ contributes exactly 1 to the degree of each of its two vertices. Therefore, when we sum up the degrees of all vertices, each edge is counted twice. Hence, the sum of the degrees of all vertices equals twice the number of edges.

**Tangentially Related Concepts**:

1. **[[Complete Graphs|Complete Graphs]]**: These are a special case of regular graphs where each vertex is connected to every other vertex. A complete graph with $n$ vertices is also an $(n-1)$-regular graph.

2. **[[Bipartite Graph|Bipartite Graphs]]**: A regular [[Bipartite Graph|bipartite graph]] is a graph whose vertices can be divided into two [[Disjoint Sets|disjoint sets]] such that every vertex in the first set is connected to the same number of vertices in the second set.

3. **[[Eulerian Graphs|Eulerian Graphs]]**: An Eulerian graph is a graph containing a closed walk that includes every edge exactly once. All [[Eulerian Graphs|Eulerian graphs]] are regular, but not all regular graphs are Eulerian.

> For further reading, you may want to explore the following resources:
> - [Regular Graph - Wikipedia](https://www.google.com/search?q=Regular+Graph+Wikipedia)
> - [Handshaking Lemma and its applications - GeeksforGeeks](https://www.google.com/search?q=Handshaking+Lemma+and+its+applications+GeeksforGeeks)
> - [Graph Theory - Mathematics LibreTexts](https://www.google.com/search?q=Graph+Theory+Mathematics+LibreTexts)