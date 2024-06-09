---
bad_links: 
aliases: []
tags: [graphtheory, algorithms]
title: Graph Isomorphism
date created: Saturday, July 15th 2023, 4:54:46 pm
---
# Graph Isomorphism

Graph Isomorphism is a concept in graph theory that refers to the relationship between two graphs. Two graphs are considered isomorphic if there is a one-to-one correspondence between their vertices and edges, such that the resulting adjacency relationships are preserved. In simpler terms, two graphs are isomorphic if they have the same structure, even though their vertices and edges may be labeled or drawn differently. This concept is crucial in various fields including computer science, where it helps in identifying similar structures in different data sets.

## Example

Problem:  
Consider two graphs G1 and G2.

G1 has vertices V1 = {a, b, c, d} and edges E1 = {(a,b), (b,c), (c,d), (d,a)}.

G2 has vertices V2 = {w, x, y, z} and edges E2 = {(w,x), (x,y), (y,z), (z,w)}.

Are these two graphs isomorphic?

Solution:  
To check if the two graphs are isomorphic, we need to find a one-to-one correspondence between the vertices and edges of the two graphs such that the adjacency relationships are preserved.

Lets map the vertices of G1 to G2 as follows:

a -> w,  
b -> x,  
c -> y,  
d -> z.

Now lets check if the edges correspond:

(a,b) in G1 corresponds to (w,x) in G2,  
(b,c) in G1 corresponds to (x,y) in G2,  
(c,d) in G1 corresponds to (y,z) in G2,  
(d,a) in G1 corresponds to (z,w) in G2.

Since all vertices and edges correspond while preserving adjacency relationships, we can conclude that the two graphs are isomorphic.

The solution can be formatted as follows:

$$
\begin{gather*} 
\text{Map vertices: } a \rightarrow w, b \rightarrow x, c \rightarrow y, d \rightarrow z \\
\text{Check edge correspondence: } \\
(a,b) \rightarrow (w,x),\\
(b,c) \rightarrow (x,y),\\
(c,d) \rightarrow (y,z),\\
(d,a) \rightarrow (z,w).\\
\text{Since all vertices and edges correspond while preserving adjacency relationships}, \\
\text{we conclude that } G1 \text{ and } G2 \text{ are isomorphic.}
\end{gather*}
$$
