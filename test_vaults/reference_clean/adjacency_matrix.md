---
bad_links: 
aliases: []
tags: [graphtheory]
title: Adjacency Matrix
date created: Saturday, July 15th 2023, 4:42:44 pm
---
# Adjacency Matrix

An adjacency matrix is a square matrix used to represent a finite graph. The elements of the matrix indicate whether pairs of vertices are adjacent or not in the graph. In the special case of a finite simple graph, the adjacency matrix is a (0,1)-matrix with zeros on its diagonal.

If the graph is undirected (i.e., all of its edges are bidirectional), the adjacency matrix is symmetric. The relationship between a graph and the eigenvalues and eigenvectors of its adjacency matrix is studied in spectral graph theory.

The adjacency matrix of a graph should be distinguished from its incidence matrix, a different matrix representation whose elements indicate whether vertex–edge pairs are incident or not, and its degree matrix, which contains information about the degree of each vertex.

## Definition
For a simple graph with vertex set $U = \{u_1, …, u_n\}$, the adjacency matrix is a square $n \times n$ matrix $A$ such that its element $A_{ij}$ is one when there is an edge from vertex $i$ to vertex $j$, and zero when there is no edge. The diagonal elements of the matrix are all zero, since edges from a vertex to itself (loops) are not allowed in simple graphs. It is also sometimes useful in algebraic graph theory to replace the zeros in the adjacency matrix with algebraic variables.

## Properties
1. The adjacency matrix of an undirected simple graph is symmetric, and therefore has a complete set of real eigenvalues and an orthogonal eigenvector basis. The set of eigenvalues of a graph is the spectrum of the graph.
2. The adjacency matrix of a graph is positive semidefinite if and only if the graph is bipartite, which is true if and only if the spectrum is symmetric about the origin.
3. The adjacency matrix can be used to find the degree of a vertex in a constant time.

## Formulas and Derivations
The degree $d_i$ of a vertex $v_i$ can be computed by summing the elements of the $i$-th row (or the $i$-th column):

$$
d_i = \sum_{j=1}^{n} A_{ij}
$$

The walk of length $k$ from a vertex $v_i$ to a vertex $v_j$ is given by the $ij$-th entry in the $k$-th power of the adjacency matrix, $(A^k)_{ij}$.

## Proofs
The proof of the above formula for walks of length $k$ is by induction on $k$. The base case $k = 1$ is the definition of the adjacency matrix. For the inductive case, consider walks of length $k + 1$ from $i$ to $j$. Such a walk must go through some intermediate vertex $v_m$, where the walk from $i$ to $m$ is of length $k$ and the walk from $m$ to $j$ is of length 1. The number of such walks is therefore $\sum_{m=1}^{n} (A^k)_{im} A_{mj}$, which is the $ij$-th entry in the matrix product $A^k A = A^{k+1}$.

> For more information, you can refer to the following resources:
> - [Adjacency Matrix on Wikipedia](https://www.google.com/search?q=site:wikipedia.org+Adjacency+Matrix)
> - [Spectral Graph Theory](https://www.google.com/search?q=Spectral+Graph+Theory)
> - [Algebraic Graph Theory](https://www.google.com/search?q=Algebraic+Graph+Theory)
> - [Incidence Matrix](https://www.google.com/search?q=site:wikipedia.org+Incidence+Matrix)
> - [Degree Matrix](https://www.google.com/search?q=site:wikipedia.org+Degree+Matrix)