---
aliases: []
tags: [algorithms]
bad_links:
---
# Topological Sort

Topological sorting for Directed Acyclic Graph (DAG) is a linear [[Ordering|ordering]] of vertices such that for every directed edge $(u, v)$, vertex $u$ comes before $v$ in the [[Ordering|ordering]]. Topological Sorting for a graph is not possible if the graph is not a DAG.

A DAG has at least one topological sort. The classic application of Topological Sort is the scheduling of tasks. In computer science, applications of this type arise in instruction scheduling, [[Ordering|ordering]] of formula cell evaluation when recomputing formula values in spreadsheets, logic synthesis, determining the order of compilation tasks to perform in makefiles, data [[Binary Serialization|serialization,]] and resolving symbol dependencies in linkers.

The algorithm for the topological sort is as follows:

1. Identify a list of "start nodes" which have no incoming edges and insert them into a set S; at least one such node must exist in a non-empty acyclic graph.
2. Remove a node n from S, and add n to tail of the output list L.
3. For each node m with an edge e from n to m, remove edge e from the graph. If m has no other incoming edges then insert m into S.
4. If graph has edges, then output error message (graph has at least one cycle) and stop; otherwise, output the nodes of L in order.

The [[Big-O Notation.md|time complexity]] of this algorithm is $O(V+E)$, where $V$ is the number of vertices and $E$ is the number of edges.

> For more information, you can refer to the following resources:
> - [Topological Sort](https://en.wikipedia.org/wiki/Topological_sorting)
> - [Topological Sort Algorithm](https://www.geeksforgeeks.org/topological-sorting/)
> - [Directed Acyclic Graphs](https://en.wikipedia.org/wiki/Directed_acyclic_graph)