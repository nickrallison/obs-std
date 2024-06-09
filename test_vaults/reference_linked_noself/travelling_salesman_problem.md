---
bad_links: 
aliases: [TSP]
tags: [computerscience, algorithms]
---
# Travelling Salesman Problem

The Travelling Salesman Problem (TSP) is a classic algorithmic problem in the field of computer science and operations research which focuses on optimization. In this problem, a salesman is given a list of cities, and must determine the shortest possible route that allows him to visit each city once and return to his original location.

The problem can be formally defined as follows: Given a list of cities and the distances between each pair of cities, what is the shortest possible route that visits each city exactly once and returns to the origin city?

Mathematically, the TSP can be represented as a graph, where the cities are the graph's vertices, paths between cities represent the edges, and the distance of each path represents the edge's weight. The problem then reduces to finding the shortest [[Hamiltonian cycle|Hamiltonian cycle]] in the graph. A [[Hamiltonian cycle|Hamiltonian cycle]] (or [[Hamiltonian cycle|Hamiltonian circuit]]) is a cycle in an undirected or [[Directed Graph|directed graph]] that visits each vertex exactly once and also returns to the origin vertex.

The TSP is NP-hard, meaning that we do not know of any efficient algorithm that can find the optimal solution for all instances of the problem. The problem is also [[NP-Complete|NP-complete]], which means that it is both in [[Nondeterministic Polynomial Time|NP]] and NP-hard. The decision problem version (does there exist a route of a given length or less?) is one of Karp's 21 [[NP-Complete|NP-complete]] problems.

The most straightforward way to solve the TSP is to use a brute force algorithm, which enumerates all possible permutations (routes) and then selects the shortest one. The [[Big-O Notation|time complexity]] of this approach is $O(n!)$, which is infeasible for large $n$.

There are more efficient algorithms to solve the TSP, such as the [[Held-Karp Algorithm|Held-Karp algorithm]], which is a [[Dynamic Programming|dynamic programming]] approach with [[Big-O Notation|time complexity]] $O(n^2 2^n)$. However, this is still exponential time, and thus infeasible for large $n$.

Approximation algorithms, such as the [[Christofides Algorithm|Christofides algorithm]] and 2-opt, provide a way to find solutions that are good enough for practical use in [[Polynomial Time|polynomial time]]. The [[Christofides Algorithm|Christofides algorithm]] guarantees to come within a factor of 1.5 of the optimal solution, and the 2-opt algorithm iteratively removes two edges and replaces these with two different edges that decrease the total distance.

The TSP has several important applications in science and engineering, including planning, logistics, and the manufacture of microchips. In fact, TSP is used in the routing of logistics as in the case of FedEx, UPS, etc. It is also used in DNA sequencing where the aim is to find the shortest possible DNA sequence from a given set of sequences.

> For further reading, you may find the following resources useful:
> - [Travelling Salesman Problem on Wikipedia](https://www.google.com/search?q=Travelling+Salesman+Problem+Wikipedia)
> - [Held-Karp algorithm on Wikipedia](https://www.google.com/search?q=Held-Karp+algorithm+Wikipedia)
> - [Christofides algorithm on Wikipedia](https://www.google.com/search?q=Christofides+algorithm+Wikipedia)
> - [2-opt algorithm on Wikipedia](https://www.google.com/search?q=2-opt+algorithm+Wikipedia)
> - [Hamiltonian path on Wikipedia](https://www.google.com/search?q=Hamiltonian+path+Wikipedia)
> - [NP-completeness on Wikipedia](https://www.google.com/search?q=NP-completeness+Wikipedia)