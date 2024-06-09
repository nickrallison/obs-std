---
bad_links: 
aliases: [Hamiltonian Circuit]
tags: [graphtheory]
---
# Hamiltonian Cycle

A Hamiltonian cycle, named after Sir William Rowan Hamilton, is a concept in graph theory. It is a closed loop on a graph where every node (vertex) is visited exactly once and the first node is also the last, forming a cycle. 

A graph that contains a Hamiltonian cycle is known as a Hamiltonian graph. The problem of determining whether such a cycle exists in a given graph is known as the Hamiltonian Cycle Problem (HCP). 

The Hamiltonian Cycle Problem is NP-complete, meaning it is computationally difficult to solve. No efficient solution is known for the worst-case scenario, and it is generally believed that no such solution exists.

The mathematical definition of a Hamiltonian cycle can be given as follows:

Given a graph $G = (V, E)$, where $V$ is the set of vertices and $E$ is the set of edges, a Hamiltonian cycle is a cycle that visits each vertex $v \in V$ exactly once, except for the vertex that is both the start and end of the cycle.

The Hamiltonian Cycle Problem can be formally stated as follows:

Given a graph $G = (V, E)$, does there exist a permutation $\pi$ of the vertices $V$ such that for all $i$, $(\pi(i), \pi(i+1))$ is an edge in $E$, where the indices are taken modulo $|V|$ (the number of vertices in $V$)?

The Hamiltonian Cycle Problem is closely related to the Traveling Salesman Problem (TSP), which is also NP-complete. The TSP asks for the shortest possible route that visits each city and returns to the origin city. If we consider each city as a vertex and the distance between cities as edge weights, the TSP can be seen as a weighted version of the HCP.

> For more in-depth reading, you may refer to the following resources:
> - [Hamiltonian Cycle Problem on Wikipedia](https://www.google.com/search?q=Hamiltonian+Cycle+Problem+Wikipedia)
> - [Traveling Salesman Problem on Wikipedia](https://www.google.com/search?q=Traveling+Salesman+Problem+Wikipedia)
> - [NP-Completeness on Wikipedia](https://www.google.com/search?q=NP-Completeness+Wikipedia)
> - [Hamiltonian paths and cycles in graphs on Google Scholar](https://scholar.google.com/scholar?q=Hamiltonian+paths+and+cycles+in+graphs)