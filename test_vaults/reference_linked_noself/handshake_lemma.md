---
bad_links: 
aliases: [Handshaking Lemma]
tags: [graphtheory]
title: Handshake Lemma
date created: Saturday, July 15th 2023, 4:39:04 pm
---
# Handshake Lemma

The Handshaking Lemma is a fundamental concept in graph theory. It is named so because it can be illustrated using a scenario where people are shaking hands. The lemma states that in any finite [[Undirected Graph|undirected graph]], the sum of the degrees of all vertices is equal to twice the number of edges. In other words, if you add up the number of edges connected to each vertex, you'll get a number that's exactly twice the total number of edges in the graph.

Mathematically, this can be expressed as:

$$
\sum_{v \in V} \deg(v) = 2|E|
$$

where $V$ is the set of vertices, $\deg(v)$ is the degree of vertex $v$ (i.e., the number of edges connected to $v$), and $E$ is the set of edges.

The proof of the Handshaking Lemma is quite straightforward. Each edge in an [[Undirected Graph|undirected graph]] has two endpoints. Therefore, when we sum the degrees of all vertices, each edge is counted twice: once for each endpoint. Hence, the sum of the degrees of all vertices is twice the number of edges.

The Handshaking Lemma has an important corollary: in any graph, the number of vertices of odd degree is even. This follows directly from the lemma because the sum of all degrees (which is even by the lemma) cannot be made up of an odd number of odd integers.

The Handshaking Lemma is fundamental to the study of graph theory and serves as the basis for many proofs and theorems. For example, it is used in proving [[Euler’s Theorem|Euler's theorem]] that a graph has an [[Euler Circuit|Eulerian circuit]] if and only if every vertex has even degree.

> For further reading, you might find the following resources useful:
> - ["Handshaking Lemma and its applications"](https://www.google.com/search?q=Handshaking+Lemma+and+its+applications)
> - ["Graph Theory"](https://www.google.com/search?q=Graph+Theory)
> - ["Euler's Theorem in Graph Theory"](https://www.google.com/search?q=Euler%27s+Theorem+in+Graph+Theory)