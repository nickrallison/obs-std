---
bad_links: 
aliases: []
tags: [graphtheory]
title: Vertex Degree
date created: Sunday, July 16th 2023, 10:55:13 am
---
# Vertex Degree

Vertex degree in graph theory refers to the number of edges that are incident to a vertex. In simpler terms, its the number of connections a point has in a network. There are two types of degrees: In-degree (number of incoming edges) and Out-degree (number of outgoing edges). The concept is used in various fields such as computer science, mathematics, and network analysis.

It can be represented in a general form as:

$$
\begin{gather*} 
\text{Degree}(v) = \sum_{u \in N(v)} 1
\end{gather*}
$$

where $v$ is a vertex, $N(v)$ is the set of neighbors of $v$, and the sum counts the number of edges connected to $v$.

For [[Directed Graph|directed graphs]], we can define in-degree and out-degree as follows:

In-degree:
$$
\begin{gather*} 
\text{InDegree}(v) = \sum_{u \in N_{in}(v)} 1
\end{gather*}
$$

Out-degree:
$$
\begin{gather*} 
\text{OutDegree}(v) = \sum_{u \in N_{out}(v)} 1
\end{gather*}
$$

where $N_{in}(v)$ and $N_{out}(v)$ are the sets of incoming and outgoing neighbors of $v$, respectively.

Example Problem:

Consider a [[Directed Graph|directed graph]] G = (V, E) where V = {1, 2, 3, 4} and E = {(1, 2), (2, 3), (3, 4), (4, 1)}. Find the sets $N_{in}(v)$ and $N_{out}(v)$ for each vertex v in V.

Solution:

$$
\begin{gather*} 
N_{in}(1) = \{4\}, N_{out}(1) = \{2\} \newline
N_{in}(2) = \{1\}, N_{out}(2) = \{3\} \newline
N_{in}(3) = \{2\}, N_{out}(3) = \{4\} \newline
N_{in}(4) = \{3\}, N_{out}(4) = \{1\}
\end{gather*}
$$

Explanation:

For each vertex v in V, we look at the edges in E. If an edge starts from v, then the end vertex of that edge is an outgoing neighbor of v. If an edge ends at v, then the start vertex of that edge is an incoming neighbor of v. For example, for vertex 1 we have one outgoing edge to vertex 2 and one incoming edge from vertex 4. Therefore $N_{in}(1)$ is {4} and $N_{out}(1)$ is {2}. Similarly we can find $N_{in}$ and $N_{out}$ for other vertices.
