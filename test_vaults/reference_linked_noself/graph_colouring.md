---
bad_links: 
aliases: [colorable, colourable, graph coloring]
tags: [graphtheory]
title: Graph Colouring
date created: Saturday, July 15th 2023, 4:45:31 pm
---
# Graph Colouring

Graph coloring is a method of assigning labels (represented by colors) to the vertices of a graph G such that no two adjacent vertices have the same color. This is also known as a vertex coloring. The smallest number of colors needed to color a graph G is its chromatic number, and it is often denoted as $\chi(G)$.

The concept of graph coloring has many applications in computer science and operations research, particularly in scheduling and partitioning problems. For instance, it can be used to solve scheduling problems where certain tasks cannot be performed at the same time.

The graph coloring problem is a computational problem aiming to determine the minimum number of colors needed for a given graph. It is a [[NP-Complete|NP-complete]] problem, meaning that it is computationally difficult to solve.

The simplest form of graph coloring problem is to decide if a graph can be colored with k colors, which is a decision problem. The corresponding optimization problem is to find the smallest value of k for which this is possible.

**Formal Definition**

A k-coloring of a graph G is a function $c: V(G) \rightarrow \{1, 2, …, k\}$ such that $c(u) \neq c(v)$ whenever $u$ and $v$ are adjacent. The chromatic number $\chi(G)$ of a graph G is the smallest k for which G has a k-coloring.

**[[Four Color Theorem|Four Color Theorem]]**

One of the most famous theorems related to graph coloring is the [[Four Color Theorem|Four Color Theorem]]. It states that any [[Planar Graph|planar graph]] (a graph that can be drawn on a plane without edges crossing) can be colored with no more than four colors. The proof of this theorem is beyond the scope of this explanation as it is quite complex and was one of the first major theorems to be proven using a computer.

**Greedy Coloring Algorithm**

A simple algorithm for coloring a graph is the greedy algorithm. This algorithm colors the vertices of the graph in sequence, always choosing the smallest available color. The algorithm can be formally described as follows:

1. Initialize all vertices as uncolored.
2. Do the following for every vertex v:
   - Choose the smallest available color that can be given to v (i.e., a color that is not used by v's adjacent vertices).
   - Assign the chosen color to v.

The [[Big-O Notation|time complexity]] of this algorithm is $O(V^2 + E)$ in the worst case.

**Welsh-Powell Algorithm**

The Welsh-Powell algorithm is an improvement of the greedy algorithm. It works by [[Ordering|ordering]] the vertices by their degree, and then applying the greedy algorithm. This often results in a better coloring.

**Formulas and Proofs**

The chromatic number of a graph is related to other graph properties. For instance, for any graph G, we have $\chi(G) \leq \Delta(G) + 1$, where $\Delta(G)$ is the maximum degree of G. This is because we can color the graph by applying the greedy algorithm to the vertices in any order.

The proof of this statement is straightforward: when we color a vertex v, it has at most $\Delta(G)$ neighbors, so there are at most $\Delta(G)$ forbidden colors, and hence there is a color available in the set $\{1, 2, …, \Delta(G) + 1\}$.

> For further reading, you may want to look into the following resources:
> - [Graph Coloring (Wikipedia)](https://www.google.com/search?q=Graph+Coloring+Wikipedia)
> - [Four Color Theorem (Wikipedia)](https://www.google.com/search?q=Four+Color+Theorem+Wikipedia)
> - [Greedy Coloring Algorithm (GeeksforGeeks)](https://www.google.com/search?q=Greedy+Coloring+Algorithm+GeeksforGeeks)
> - [Welsh-Powell Algorithm (GeeksforGeeks)](https://www.google.com/search?q=Welsh-Powell+Algorithm+GeeksforGeeks)
> - [Chromatic Number (Wolfram MathWorld)](https://www.google.com/search?q=Chromatic+Number+Wolfram+MathWorld)