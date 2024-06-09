---
bad_links: 
aliases: [prim]
tags: [algorithms, datastructures]
---
# Prim's Algorithm

Prim's Algorithm is a greedy algorithm that finds a minimum spanning tree for a weighted undirected graph. This means it finds a subset of the edges that forms a tree that includes every vertex, where the total weight of all the edges in the tree is minimized.

The algorithm operates by building this tree one vertex at a time, from an arbitrary starting vertex, at each step adding the cheapest possible connection from the tree to another vertex.

The formal steps of Prim's Algorithm are as follows:

1. Initialize a tree with a single vertex, chosen arbitrarily from the graph.
2. Grow the tree by one edge: of the edges that connect the tree to vertices not yet in the tree, find the minimum-weight edge, and transfer it to the tree.
3. Repeat step 2 (until all vertices are in the tree).

In terms of time complexity, Prim's algorithm takes $O(E \log V)$ time with a Heap Data Structure.md|binary heap]] or Fibonacci heap, where $E$ is the number of edges and $V$ is the number of vertices.

The algorithm was developed in the context of telecommunications to find the minimum wire needed to connect all buildings in a neighborhood; a problem also known as the minimum spanning tree problem.

> For more in-depth reading, you can refer to the following resources:
> - [Prim's Algorithm - Wikipedia](https://www.google.com/search?q=Prim%27s+Algorithm+-+Wikipedia)
> - [Prim's Algorithm - GeeksforGeeks](https://www.google.com/search?q=Prim%27s+Algorithm+-+GeeksforGeeks)
> - [Prim's Algorithm - Tutorialspoint](https://www.google.com/search?q=Prim%27s+Algorithm+-+Tutorialspoint)