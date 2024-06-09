---
bad_links: 
aliases: []
tags: [graphtheory]
title: Bipartite Graph
date created: Sunday, July 16th 2023, 11:06:56 am
---
# Bipartite Graph

A bipartite graph is a type of graph that divides its vertices into two separate sets, such that every edge connects a vertex in the first set to one in the second set. In other words, there are no edges connecting vertices within the same set. This type of graph is commonly used in computer science and discrete mathematics to model relationships between two different classes of objects. For example, it can represent relationships between customers and products, students and courses, or jobs and machines.

A graph G = (V, E) is bipartite if the vertices can be divided into to disjoint  
sets $X$ and $Y$, with  
$(X \cup Y = V) \land (X \cap Y = \emptyset)$,  
such that for each edge $(u, v)\,\epsilon\,E$,  
either $(u\,\epsilon\,X\,\land\,v\,\epsilon\,Y) \lor (v\,\epsilon\,X\,\land\,u\,\epsilon\,Y)$

Example Problem:

Consider the following graph G = (V, E) where V = {1, 2, 3, 4, 5} and E = {(1,2), (2,3), (3,4), (4,5)}. Determine whether this graph is bipartite or not.

Solution:

To determine if a graph is bipartite or not, we need to check if its possible to divide the vertices into two [[Disjoint Sets|disjoint sets]] such that every edge connects a vertex in one set to a vertex in another set.

Lets try to divide the vertices into two sets X and Y:

X = {1, 3, 5} and Y = {2, 4}

Now lets verify if every edge connects a vertex in X to a vertex in Y:

Edge (1,2) connects vertex 1 in X to vertex 2 in Y.

Edge (2,3) connects vertex 2 in Y to vertex 3 in X.

Edge (3,4) connects vertex 3 in X to vertex 4 in Y.

Edge (4,5) connects vertex 4 in Y to vertex 5 in X.

Since all edges connect vertices from different sets and there are no edges connecting vertices within the same set,

Therefore,
$$
\begin{gather*} 
X \cup Y = V \newline
X \cap Y = \emptyset \newline
\text{For each edge } (u,v)\,\epsilon\,E,\,(u\,\epsilon\,X\,\land\,v\,\epsilon\,Y) \lor (v\,\epsilon\,X\,\land\,u\,\epsilon\,Y)\newline
\text{Hence the given graph G is Bipartite.}\newline
\end{gather*}
$$
