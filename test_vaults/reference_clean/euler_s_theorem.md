---
bad_links:
aliases: []
tags: [graphtheory]
---
# Euler’s Theorem

Euler's Theorem is a foundational concept in graph theory, a branch of mathematics that studies graphs, which are mathematical structures used to model pairwise relations between objects. Euler's theorem is particularly concerned with Eulerian paths and circuits.

An Eulerian path is a path in a graph that visits every edge exactly once. An Eulerian circuit is an Eulerian path that starts and ends on the same vertex. Euler's theorem states that a connected graph has an Eulerian circuit if and only if every vertex has even degree, and it has an Eulerian path if and only if exactly zero or two vertices have odd degree.

The degree of a vertex is the number of edges incident to it. In simpler terms, it's the number of edges that touch a particular point.

The theorem is named after Leonhard Euler, who first proved it in the 18th century. His proof was based on the "Seven Bridges of Königsberg" problem, which asked whether it was possible to walk through the city of Königsberg crossing each of its seven bridges exactly once.

The formal proof of Euler's theorem is as follows:

**Proof**:

1. If a graph has an Eulerian circuit, then each edge is traversed exactly twice (once in each direction), so each vertex must have even degree.

2. Conversely, if a graph is connected and each vertex has even degree, we can construct an Eulerian circuit as follows: Start at any vertex, and keep traversing edges that have not yet been traversed, removing them from the graph as you go. Since each vertex has even degree, whenever you reach a vertex, there must be an unused edge that you can use to leave it. Continue until you return to the starting vertex. This gives a circuit, and since the graph is connected and we always used an unused edge, this circuit must be Eulerian.

The proof for the existence of an Eulerian path is similar, but we need to allow for exactly two vertices to have odd degree (the start and end points of the path).

Euler's theorem has many applications in computer science, operations research, and logistics, among other fields. For example, it can be used to find efficient routes for garbage collection, mail delivery, or circuit board manufacturing.

> For more information, you can refer to the following resources:
> - [Euler's Theorem - Wikipedia](https://www.google.com/search?q=Euler%27s+Theorem+Wikipedia)
> - [Graph Theory - MathWorld](https://www.google.com/search?q=Graph+Theory+MathWorld)
> - [Eulerian path and circuit for undirected graph - GeeksforGeeks](https://www.google.com/search?q=Eulerian+path+and+circuit+for+undirected+graph+GeeksforGeeks)