---
bad_links: 
aliases: []
tags: [graphtheory, algorithms]
---
# Christofides Algorithm

The Christofides Algorithm is a heuristic algorithm used in computer science and operations research for solving instances of the Traveling Salesman Problem (TSP) in graphs. The algorithm was proposed by Nicos Christofides in 1976 and it provides a solution that is within 1.5 times the optimal solution.

The algorithm operates on a complete, undirected graph with non-negative edge weights satisfying the triangle inequality. The triangle inequality implies that for any three vertices $a$, $b$, and $c$, the direct path from $a$ to $c$ is no longer than the path from $a$ to $b$ and then from $b$ to $c$.

Here are the steps of the Christofides Algorithm:

1. **[[Minimum Spanning Tree.md|Minimum Spanning Tree]] ([[Minimum Spanning Tree.md|MST]])**: Find a minimum spanning tree $T$ of the graph $G$.
2. **Odd Vertex Matching**: Find the set $O$ of vertices with odd degree in $T$. By the Handshaking Lemma, $O$ must have an even number of vertices. Find a minimum-weight perfect matching $M$ in the induced subgraph given by the vertices from $O$.
3. **Combine [[Minimum Spanning Tree.md|MST]] and Matching**: Combine the edges of $T$ and $M$ to form a multigraph $H$ in which each vertex has even degree.
4. **[[Euler Circuit.md|Eulerian Circuit]]**: Form an Eulerian circuit in $H$.
5. **[[Hamiltonian cycle.md|Hamiltonian Circuit]]**: Make the circuit found in the previous step into a Hamiltonian circuit $H'$ by skipping (not revisiting) vertices in the Eulerian circuit. Return $H'$ as the output.

The Christofides Algorithm is a 1.5-approximation algorithm for the metric TSP, meaning that the length of the Hamiltonian circuit returned by the algorithm is no more than 1.5 times the length of the optimal TSP solution. The proof of this approximation ratio is non-trivial and involves several steps, including showing that the weight of the MST is no more than the weight of the optimal TSP solution, and that the weight of the minimum-weight perfect matching on the odd-degree vertices is no more than 0.5 times the weight of the optimal TSP solution.

For further reading, you may want to explore the following resources:

> - [Christofides Algorithm on Wikipedia](https://www.google.com/search?q=Christofides+Algorithm+Wikipedia)
> - [Traveling Salesman Problem on Wikipedia](https://www.google.com/search?q=Traveling+Salesman+Problem+Wikipedia)
> - [Minimum Spanning Tree on Wikipedia](https://www.google.com/search?q=Minimum+Spanning+Tree+Wikipedia)
> - [Eulerian Circuit on Wikipedia](https://www.google.com/search?q=Eulerian+Circuit+Wikipedia)
> - [Hamiltonian Circuit on Wikipedia](https://www.google.com/search?q=Hamiltonian+Circuit+Wikipedia)
> - [Handshaking Lemma on Wikipedia](https://www.google.com/search?q=Handshaking+Lemma+Wikipedia)
> - [Proof of the 1.5 approximation ratio for Christofides Algorithm](https://www.google.com/search?q=Proof+of+the+1.5+approximation+ratio+for+Christofides+Algorithm)